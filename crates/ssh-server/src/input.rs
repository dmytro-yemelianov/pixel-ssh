use pixel_ssh_view::Key;
use std::time::Duration;
use tokio::time::Instant;

const ESCAPE_TIMEOUT: Duration = Duration::from_millis(150);

#[derive(Default)]
pub(crate) struct InputParser {
    pending: Vec<u8>,
    escape_since: Option<Instant>,
}

impl InputParser {
    pub(crate) fn feed(&mut self, data: &[u8], now: Instant) -> Vec<Key> {
        let mut keys: Vec<_> = self.expire(now).into_iter().collect();
        self.pending.extend_from_slice(data);
        let mut consumed = 0;
        while let Some((length, key)) = parse_key(&self.pending[consumed..]) {
            consumed += length;
            if let Some(key) = key {
                keys.push(key);
            }
        }
        self.pending.drain(..consumed);
        if consumed > 0 {
            self.escape_since = None;
        }
        if self.pending.len() > 64 {
            // Discard malformed, unterminated sequences without growing forever.
            self.pending.clear();
        }
        if self.pending.first() == Some(&0x1b) {
            self.escape_since.get_or_insert(now);
        } else {
            self.escape_since = None;
        }
        keys
    }

    pub(crate) fn expire(&mut self, now: Instant) -> Option<Key> {
        if self
            .escape_since
            .is_some_and(|since| now.duration_since(since) >= ESCAPE_TIMEOUT)
        {
            let standalone = self.pending == [0x1b];
            self.pending.clear();
            self.escape_since = None;
            return standalone.then_some(Key::Escape);
        }
        None
    }
}

// None means the next key is incomplete, including a standalone Escape that
// needs a short timeout to distinguish it from a fragmented terminal sequence.
fn parse_key(bytes: &[u8]) -> Option<(usize, Option<Key>)> {
    let first = *bytes.first()?;
    let key = match first {
        0x1b => {
            let second = *bytes.get(1)?;
            if second == b'[' {
                let end = bytes
                    .iter()
                    .enumerate()
                    .skip(2)
                    .find_map(|(i, byte)| (0x40..=0x7e).contains(byte).then_some(i))?;
                let key = match bytes[end] {
                    b'~' => {
                        let code = std::str::from_utf8(&bytes[2..end])
                            .ok()
                            .and_then(|s| s.split(';').next())
                            .and_then(|s| s.parse::<u8>().ok());
                        match code {
                            Some(1 | 7) => Some(Key::Home),
                            Some(3) => Some(Key::Backspace),
                            Some(4 | 8) => Some(Key::End),
                            Some(5) => Some(Key::PageUp),
                            Some(6) => Some(Key::PageDown),
                            Some(15) => Some(Key::F(5)),
                            Some(code @ 17..=21) => Some(Key::F(code - 11)),
                            Some(code @ 23..=24) => Some(Key::F(code - 12)),
                            _ => None,
                        }
                    }
                    final_byte => navigation_key(final_byte),
                };
                return Some((end + 1, key));
            } else if second == b'O' {
                let third = *bytes.get(2)?;
                let key = match third {
                    b'P'..=b'S' => Some(Key::F(third - b'P' + 1)),
                    _ => navigation_key(third),
                };
                return Some((3, key));
            }
            Some(Key::Escape)
        }
        b'\r' | b'\n' => Some(Key::Enter),
        b'\t' => Some(Key::Tab),
        0x7f | 0x08 => Some(Key::Backspace),
        0x03 => Some(Key::Char('\u{3}')),
        0..=31 => None,
        32..=126 => Some(Key::Char(first as char)),
        _ => {
            let length = match first {
                0xc2..=0xdf => 2,
                0xe0..=0xef => 3,
                0xf0..=0xf4 => 4,
                _ => return Some((1, None)),
            };
            if bytes[1..bytes.len().min(length)]
                .iter()
                .any(|byte| !(0x80..=0xbf).contains(byte))
            {
                return Some((1, None));
            }
            if bytes.len() < length {
                return None;
            }
            return Some((
                length,
                std::str::from_utf8(&bytes[..length])
                    .ok()
                    .and_then(|s| s.chars().next())
                    .map(Key::Char),
            ));
        }
    };
    Some((1, key))
}

fn navigation_key(byte: u8) -> Option<Key> {
    match byte {
        b'A' => Some(Key::Up),
        b'B' => Some(Key::Down),
        b'C' => Some(Key::Right),
        b'D' => Some(Key::Left),
        b'H' => Some(Key::Home),
        b'F' => Some(Key::End),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminal_keys_survive_every_packet_boundary() {
        for (bytes, key) in [
            (b"\x1b[B".as_slice(), Key::Down),
            (b"\x1bOA", Key::Up),
            (b"\x1b[6~", Key::PageDown),
            (b"\x1b[17~", Key::F(6)),
            (b"\x1b[1;5C", Key::Right),
            ("ї".as_bytes(), Key::Char('ї')),
        ] {
            for split in 1..bytes.len() {
                let now = Instant::now();
                let mut parser = InputParser::default();
                assert!(parser.feed(&bytes[..split], now).is_empty());
                assert_eq!(
                    parser.feed(&bytes[split..], now + Duration::from_millis(20)),
                    vec![key.clone()]
                );
            }
        }
    }

    #[test]
    fn standalone_escape_expires_without_losing_following_input() {
        let now = Instant::now();
        let mut parser = InputParser::default();
        assert!(parser.feed(b"\x1b", now).is_empty());
        assert_eq!(parser.expire(now + Duration::from_millis(100)), None);
        assert_eq!(parser.expire(now + ESCAPE_TIMEOUT), Some(Key::Escape));
        assert_eq!(
            parser.feed(b"q", now + ESCAPE_TIMEOUT),
            vec![Key::Char('q')]
        );
    }

    #[test]
    fn unsupported_sequences_do_not_swallow_the_following_key() {
        let mut parser = InputParser::default();
        assert_eq!(
            parser.feed(b"\x1b[99~q\x03", Instant::now()),
            vec![Key::Char('q'), Key::Char('\u{3}')]
        );
    }

    #[test]
    fn malformed_utf8_preserves_following_keys_across_packet_boundaries() {
        for bytes in [
            b"\xe9q\x03".as_slice(),
            b"\xe9\x80q\x03",
            b"\xf0\x80\x80q\x03",
        ] {
            for split in 0..=bytes.len() {
                let now = Instant::now();
                let mut parser = InputParser::default();
                let mut keys = parser.feed(&bytes[..split], now);
                keys.extend(parser.feed(&bytes[split..], now + Duration::from_millis(20)));
                assert_eq!(keys, vec![Key::Char('q'), Key::Char('\u{3}')]);
            }
        }
    }
}
