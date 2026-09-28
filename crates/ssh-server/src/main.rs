use log::info;
use rand::rngs::OsRng;
use russh::keys::{Algorithm, PrivateKey};
use russh::server::{Auth, Handler, Msg, Server, Session};
use russh::*;
use std::collections::HashMap;
use std::io::Write;
use std::sync::Arc;
use tokio::sync::mpsc::{error::TrySendError, Sender};
use tokio::sync::Mutex;

use pixel_ssh_core::{App, Tab};
use pixel_ssh_render_ansi::AnsiRenderer;
use pixel_ssh_view::{InputEvent, Key};

struct ClientHandler {
    app: App,
    renderer: AnsiRenderer,
    cols: u16,
    rows: u16,
    tx: Option<Sender<Vec<u8>>>,
}

struct AppServer {
    clients: Arc<Mutex<HashMap<usize, Arc<Mutex<ClientHandler>>>>>,
    next_id: usize,
}

impl Server for AppServer {
    type Handler = AppSession;

    fn new_client(&mut self, _peer_addr: Option<std::net::SocketAddr>) -> Self::Handler {
        let id = self.next_id;
        self.next_id += 1;
        AppSession {
            id,
            clients: self.clients.clone(),
        }
    }
}

struct AppSession {
    id: usize,
    clients: Arc<Mutex<HashMap<usize, Arc<Mutex<ClientHandler>>>>>,
}

impl Drop for AppSession {
    fn drop(&mut self) {
        let clients = self.clients.clone();
        let id = self.id;
        tokio::spawn(async move {
            clients.lock().await.remove(&id);
        });
    }
}

impl Handler for AppSession {
    type Error = russh::Error;

    async fn auth_none(&mut self, user: &str) -> Result<Auth, Self::Error> {
        info!("Anonymous SSH auth requested for user: '{user}'");
        // Only the anonymous guest account can open an application session.
        Ok(if user == "guest" {
            Auth::Accept
        } else {
            Auth::Reject {
                proceed_with_methods: None,
            }
        })
    }

    async fn auth_password(&mut self, user: &str, _pass: &str) -> Result<Auth, Self::Error> {
        info!("Password auth requested for user: '{user}'");
        Ok(Auth::Reject {
            proceed_with_methods: None,
        })
    }

    async fn auth_publickey(
        &mut self,
        user: &str,
        _publickey: &russh::keys::PublicKey,
    ) -> Result<Auth, Self::Error> {
        info!("Publickey auth requested for user: '{user}'");
        Ok(Auth::Reject {
            proceed_with_methods: None,
        })
    }

    async fn channel_open_session(
        &mut self,
        _channel: Channel<Msg>,
        _session: &mut Session,
    ) -> Result<bool, Self::Error> {
        info!("Channel open session for client {}", self.id);
        let mut clients = self.clients.lock().await;
        clients.insert(
            self.id,
            Arc::new(Mutex::new(ClientHandler {
                app: App::new_terminal(),
                renderer: AnsiRenderer::new(80, 50),
                cols: 80,
                rows: 50,
                tx: None,
            })),
        );
        Ok(true)
    }

    async fn pty_request(
        &mut self,
        channel: ChannelId,
        term: &str,
        col_width: u32,
        row_height: u32,
        _pix_width: u32,
        _pix_height: u32,
        _modes: &[(Pty, u32)],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        info!("PTY request: term={term}, cols={col_width}, rows={row_height}");
        let client = self.clients.lock().await.get(&self.id).cloned();
        if let Some(client) = client {
            let mut client = client.lock().await;
            client.cols = if col_width >= 40 {
                (col_width as u16).clamp(40, 300)
            } else {
                80
            };
            client.rows = if row_height >= 10 {
                (row_height as u16).clamp(10, 150)
            } else {
                25
            };
            let (cols, rows) = (client.cols, client.rows);
            client.app.set_terminal_size(cols, rows);
            client.renderer.resize(cols, rows);

            let (tx, mut rx) = tokio::sync::mpsc::channel::<Vec<u8>>(2);
            let handle = session.handle();
            tokio::spawn(async move {
                while let Some(msg) = rx.recv().await {
                    if handle.data(channel, msg.into()).await.is_err() {
                        break;
                    }
                }
            });

            // Enter alternate screen buffer (\x1b[?1049h), clear screen, move to (1,1)
            let mut init_seq = String::from("\x1b[?1049h\x1b[2J\x1b[H");
            let view = client.app.render();
            let ansi = client.renderer.render_view(&view);
            init_seq.push_str(&ansi);
            let _ = tx.try_send(init_seq.into_bytes());
            client.tx = Some(tx);
        }
        Ok(())
    }

    async fn window_change_request(
        &mut self,
        channel: ChannelId,
        col_width: u32,
        row_height: u32,
        _pix_width: u32,
        _pix_height: u32,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        let client = self.clients.lock().await.get(&self.id).cloned();
        if let Some(client) = client {
            let mut client = client.lock().await;
            client.cols = if col_width >= 40 {
                (col_width as u16).clamp(40, 300)
            } else {
                80
            };
            client.rows = if row_height >= 10 {
                (row_height as u16).clamp(10, 150)
            } else {
                25
            };
            let (cols, rows) = (client.cols, client.rows);
            client.app.set_terminal_size(cols, rows);
            client.renderer.resize(cols, rows);

            // Re-render and send with clear to avoid any artifacts on resize
            let mut redraw = String::from("\x1b[2J\x1b[H");
            let view = client.app.render();
            let ansi = client.renderer.render_view(&view);
            redraw.push_str(&ansi);
            if let Some(tx) = &client.tx {
                if tx.try_send(redraw.into_bytes()).is_err() {
                    client.renderer.force_full_redraw();
                }
            } else {
                let _ = session.data(channel, redraw.into_bytes().into());
            }
        }
        Ok(())
    }

    async fn data(
        &mut self,
        channel: ChannelId,
        data: &[u8],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        let client = self.clients.lock().await.get(&self.id).cloned();
        if let Some(client) = client {
            let mut client = client.lock().await;
            // Parse ANSI terminal key sequences
            let mut i = 0;
            let mut dirty = false;

            while i < data.len() {
                // Ctrl+C: exit session cleanly
                if data[i] == 0x03 {
                    if let Some(tx) = &client.tx {
                        let _ = tx.try_send(
                            "\x1b[?1049l\x1b[?25h\r\nConnection closed.\r\n"
                                .as_bytes()
                                .to_vec(),
                        );
                    } else {
                        let _ = session.data(
                            channel,
                            "\x1b[?1049l\x1b[?25h\r\nConnection closed.\r\n"
                                .as_bytes()
                                .to_vec()
                                .into(),
                        );
                    }
                    let _ = session.close(channel);
                    return Ok(());
                }

                let key = if data[i] == b'\x1b' {
                    if i + 1 >= data.len() {
                        i += 1;
                        Some(Key::Escape)
                    } else if data[i + 1] == b'[' {
                        if i + 2 < data.len() {
                            match data[i + 2] {
                                b'A' => {
                                    i += 3;
                                    Some(Key::Up)
                                }
                                b'B' => {
                                    i += 3;
                                    Some(Key::Down)
                                }
                                b'C' => {
                                    i += 3;
                                    Some(Key::Right)
                                }
                                b'D' => {
                                    i += 3;
                                    Some(Key::Left)
                                }
                                b'H' => {
                                    i += 3;
                                    Some(Key::Home)
                                }
                                b'F' => {
                                    i += 3;
                                    Some(Key::End)
                                }
                                b'1'..=b'6' if i + 3 < data.len() && data[i + 3] == b'~' => {
                                    let digit = data[i + 2];
                                    i += 4;
                                    match digit {
                                        b'1' => Some(Key::Home),
                                        b'4' => Some(Key::End),
                                        b'5' => Some(Key::PageUp),
                                        b'6' => Some(Key::PageDown),
                                        b'3' => Some(Key::Backspace),
                                        _ => None,
                                    }
                                }
                                b'1' if i + 4 < data.len() && data[i + 4] == b'~' => {
                                    let second = data[i + 3];
                                    i += 5;
                                    match second {
                                        b'5' => Some(Key::F(5)),
                                        b'7' => Some(Key::F(6)),
                                        b'8' => Some(Key::F(7)),
                                        b'9' => Some(Key::F(8)),
                                        _ => None,
                                    }
                                }
                                b'2' if i + 4 < data.len() && data[i + 4] == b'~' => {
                                    let second = data[i + 3];
                                    i += 5;
                                    match second {
                                        b'0' => Some(Key::F(9)),
                                        b'1' => Some(Key::F(10)),
                                        _ => None,
                                    }
                                }
                                _ => {
                                    i += 3;
                                    while i < data.len() && (data[i] < 0x40 || data[i] > 0x7E) {
                                        i += 1;
                                    }
                                    if i < data.len() {
                                        i += 1;
                                    }
                                    None
                                }
                            }
                        } else {
                            i += 2;
                            None
                        }
                    } else if data[i + 1] == b'O' && i + 2 < data.len() {
                        match data[i + 2] {
                            b'H' => {
                                i += 3;
                                Some(Key::Home)
                            }
                            b'F' => {
                                i += 3;
                                Some(Key::End)
                            }
                            b'A' => {
                                i += 3;
                                Some(Key::Up)
                            }
                            b'B' => {
                                i += 3;
                                Some(Key::Down)
                            }
                            b'C' => {
                                i += 3;
                                Some(Key::Right)
                            }
                            b'D' => {
                                i += 3;
                                Some(Key::Left)
                            }
                            b'P' => {
                                i += 3;
                                Some(Key::F(1))
                            }
                            b'Q' => {
                                i += 3;
                                Some(Key::F(2))
                            }
                            b'R' => {
                                i += 3;
                                Some(Key::F(3))
                            }
                            b'S' => {
                                i += 3;
                                Some(Key::F(4))
                            }
                            _ => {
                                i += 3;
                                None
                            }
                        }
                    } else {
                        i += 1;
                        Some(Key::Escape)
                    }
                } else if data[i] == b'\r' || data[i] == b'\n' {
                    i += 1;
                    Some(Key::Enter)
                } else if data[i] == b'\t' {
                    i += 1;
                    Some(Key::Tab)
                } else if data[i] == 0x7f || data[i] == 0x08 {
                    i += 1;
                    Some(Key::Backspace)
                } else if data[i] >= 32 {
                    let ch = data[i] as char;
                    i += 1;
                    Some(Key::Char(ch))
                } else {
                    i += 1;
                    None
                };

                if let Some(k) = key {
                    // Check if user pressed 'q' on main screen (not in detail, not in help)
                    if (k == Key::Char('q') || k == Key::Char('Q'))
                        && !client.app.show_detail
                        && client.app.current_tab != Tab::Help
                    {
                        if let Some(tx) = &client.tx {
                            let _ = tx.try_send(
                                "\x1b[?1049l\x1b[?25h\r\nGoodbye!\r\n".as_bytes().to_vec(),
                            );
                        } else {
                            let _ = session.data(
                                channel,
                                "\x1b[?1049l\x1b[?25h\r\nGoodbye!\r\n"
                                    .as_bytes()
                                    .to_vec()
                                    .into(),
                            );
                        }
                        let _ = session.close(channel);
                        return Ok(());
                    }

                    if client.app.update(InputEvent::KeyDown(k)) {
                        dirty = true;
                    }
                }
            }

            if dirty || data.is_empty() {
                let view = client.app.render();
                let ansi = client.renderer.render_view(&view);
                if !ansi.is_empty() {
                    if let Some(tx) = &client.tx {
                        if tx.try_send(ansi.into_bytes()).is_err() {
                            client.renderer.force_full_redraw();
                        }
                    } else {
                        let _ = session.data(channel, ansi.into_bytes().into());
                    }
                }
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    info!("Starting Pixel-First Native SSH Server...");

    // Keep a stable host identity across restarts so SSH clients can verify it.
    let host_key_path =
        std::env::var("SSH_HOST_KEY").unwrap_or_else(|_| "ssh_host_ed25519_key".to_string());
    let key = if std::path::Path::new(&host_key_path).exists() {
        russh::keys::load_secret_key(&host_key_path, None)?
    } else {
        let key = PrivateKey::random(&mut OsRng, Algorithm::Ed25519)?;
        let mut options = std::fs::OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let mut file = options.open(&host_key_path)?;
        russh::keys::encode_pkcs8_pem(&key, &mut file)?;
        file.flush()?;
        key
    };

    let config = russh::server::Config {
        inactivity_timeout: Some(std::time::Duration::from_secs(3600)),
        auth_rejection_time: std::time::Duration::from_secs(0),
        auth_rejection_time_initial: Some(std::time::Duration::from_secs(0)),
        keys: vec![key],
        ..Default::default()
    };
    let config = Arc::new(config);

    let port = std::env::var("SSH_PORT").unwrap_or_else(|_| "2222".to_string());
    let addr = format!("0.0.0.0:{port}");
    info!("SSH Server listening on ssh://guest@{addr} (No password required)");

    let clients: Arc<Mutex<HashMap<usize, Arc<Mutex<ClientHandler>>>>> =
        Arc::new(Mutex::new(HashMap::new()));
    let ticker_clients = clients.clone();

    // Spawn background marquee and clock ticker loop (10 Hz = 100ms interval, matching Web client)
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(100));
        loop {
            interval.tick().await;
            let snapshot: Vec<_> = ticker_clients
                .lock()
                .await
                .iter()
                .map(|(&id, client)| (id, client.clone()))
                .collect();
            let mut closed = Vec::new();
            for (id, client) in snapshot {
                let mut client = client.lock().await;
                if let Some(tx) = client.tx.clone() {
                    client.app.tick();
                    let view = client.app.render();
                    let ansi = client.renderer.render_view(&view);
                    if !ansi.is_empty() {
                        match tx.try_send(ansi.into_bytes()) {
                            Ok(()) => {}
                            Err(TrySendError::Full(_)) => client.renderer.force_full_redraw(),
                            Err(TrySendError::Closed(_)) => closed.push(id),
                        }
                    }
                }
            }
            if !closed.is_empty() {
                let mut clients_guard = ticker_clients.lock().await;
                for id in closed {
                    clients_guard.remove(&id);
                }
            }
        }
    });

    let mut server = AppServer {
        clients,
        next_id: 1,
    };

    server.run_on_address(config, addr.as_str()).await?;

    Ok(())
}
