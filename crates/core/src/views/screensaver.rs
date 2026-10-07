//! Moving, palette-native screensavers. No application chrome or fixed overlays.

use crate::App;
use pixel_ssh_view::{Color, Element, RectElement, TextElement, TextStyle, View, VisualEffects};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Screensaver {
    #[default]
    Starfield,
    Rain,
    Clock,
    Orbits,
    Swarm,
}

impl Screensaver {
    pub const ALL: [Self; 5] = [
        Self::Starfield,
        Self::Rain,
        Self::Clock,
        Self::Orbits,
        Self::Swarm,
    ];
    pub fn next(self) -> Self {
        Self::ALL[(Self::ALL.iter().position(|m| *m == self).unwrap() + 1) % Self::ALL.len()]
    }
    pub fn name(self) -> &'static str {
        match self {
            Self::Starfield => "Starfield",
            Self::Rain => "Digital rain",
            Self::Clock => "Bouncing clock",
            Self::Orbits => "Orbital trails",
            Self::Swarm => "Fireflies",
        }
    }
}

fn glyph(view: &mut View, x: f32, y: f32, ch: char, color: u8, lh: u16) {
    if x >= 0.0
        && y >= 0.0
        && x < (view.width.saturating_sub(8)) as f32
        && y < (view.height.saturating_sub(lh)) as f32
    {
        view.add(Element::Text(TextElement {
            x: (x as u16 / 8) * 8,
            y: (y as u16 / lh) * lh,
            text: ch.to_string(),
            style: TextStyle::new(Color::from_palette(color)).bold(),
        }));
    }
}

fn bounce(step: usize, span: usize) -> usize {
    let span = span.max(1);
    let phase = step % (span * 2);
    if phase <= span {
        phase
    } else {
        span * 2 - phase
    }
}

impl App {
    pub fn start_screensaver(&mut self) {
        self.screensaver_mode = self.screensaver_mode.next();
        self.screensaver_tick = 0;
        self.screensaver_active = true;
    }

    pub(crate) fn render_screensaver(&self, view: &mut View, width: u16, height: u16) {
        // Index 13 is pure black in every display palette; index 0 is often a
        // bright desktop background. Every frame covers the entire old view.
        view.mouse_pos = None;
        view.visual_effects = VisualEffects::clean();
        view.add(Element::Rect(RectElement {
            x: 0,
            y: 0,
            width,
            height,
            color: Color::from_palette(13),
            filled: true,
        }));
        let lh = if self.platform == pixel_ssh_view::Platform::Terminal {
            16
        } else {
            self.resolution.line_height()
        };
        let w = width as f32;
        let h = height as f32;
        let t = self.screensaver_tick as f32 * 0.08;
        match self.screensaver_mode {
            Screensaver::Starfield => {
                for i in 0..96 {
                    let angle = i as f32 * 2.399_963;
                    let depth = (t * 0.28 + i as f32 * 0.061).fract();
                    let distance = depth * depth * w.max(h) * 0.7;
                    glyph(
                        view,
                        w * 0.5 + angle.cos() * distance,
                        h * 0.5 + angle.sin() * distance,
                        if depth < 0.55 { '.' } else { '*' },
                        if depth < 0.55 { 4 } else { 14 },
                        lh,
                    );
                }
            }
            Screensaver::Rain => {
                let rows = height / lh;
                for col in 0..width / 16 {
                    let head = (self.screensaver_tick / (2 + col as usize % 3) + col as usize * 17)
                        % (rows as usize + 10);
                    for tail in 0..8usize {
                        let row = (head + rows as usize + 10 - tail) % (rows as usize + 10);
                        if row < rows as usize {
                            let ch = b"01:+*"
                                [(col as usize * 7 + row + self.screensaver_tick / 3) % 5]
                                as char;
                            glyph(
                                view,
                                (col * 16) as f32,
                                (row as u16 * lh) as f32,
                                ch,
                                if tail == 0 {
                                    14
                                } else if tail < 4 {
                                    8
                                } else {
                                    4
                                },
                                lh,
                            );
                        }
                    }
                }
            }
            Screensaver::Clock => {
                let text = self.clock_formatted();
                let x = bounce(
                    self.screensaver_tick / 2,
                    (width / 8).saturating_sub(text.chars().count() as u16) as usize,
                ) as u16
                    * 8;
                let y = bounce(
                    self.screensaver_tick / 3,
                    (height / lh).saturating_sub(1) as usize,
                ) as u16
                    * lh;
                view.add(Element::Text(TextElement {
                    x,
                    y,
                    text,
                    style: TextStyle::new(Color::from_palette(
                        [6, 8, 12, 14][self.screensaver_tick / 20 % 4],
                    ))
                    .bold(),
                }));
            }
            Screensaver::Orbits => {
                for orbit in 0..3 {
                    for trail in 0..24 {
                        let phase = t + orbit as f32 * 2.1 - trail as f32 * 0.07;
                        glyph(
                            view,
                            w * (0.5 + 0.42 * phase.sin()),
                            h * (0.5 + 0.42 * (phase * 1.7 + orbit as f32).sin()),
                            if trail == 0 { '*' } else { '.' },
                            if trail == 0 { 14 } else { [7, 8, 12][orbit] },
                            lh,
                        );
                    }
                }
            }
            Screensaver::Swarm => {
                for i in 0..48 {
                    let phase = i as f32 * 1.618;
                    glyph(
                        view,
                        w * (0.5 + 0.46 * (t * 0.5 + phase).sin()),
                        h * (0.5 + 0.46 * (t * 0.7 + phase * 1.3).cos()),
                        if (self.screensaver_tick + i) % 11 < 3 {
                            '*'
                        } else {
                            '.'
                        },
                        [6, 8, 12][i % 3],
                        lh,
                    );
                }
            }
        }
    }
}
