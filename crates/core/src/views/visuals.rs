//! Visuals / CRT shader tab view implementation (Web only).
//!
//! All visual elements, sliders, swatches, and containers are composed
//! 100% out of valid CP437 retro characters (█, ▓, ▒, ░, ┌, ─, ┐, │, └, ┘, ◄, ►, ▲, ▼, ☼, •, ♠, ♣, ♥, ♦).
#![allow(dead_code)]

use crate::state::App;
use pixel_ssh_view::{Color, Element, TextElement, TextStyle, View, VisualEffects};

/// Composes a retro character-mode gauge/slider out of valid CP437 block characters (█, ▓, ▒, ░).
/// Returns `(filled_blocks, empty_track)`.
fn compose_char_slider(val: f32, width_chars: usize) -> (String, String) {
    let total_quarters =
        ((val.clamp(0.0, 1.0) * (width_chars * 4) as f32).round() as usize).min(width_chars * 4);
    let full = total_quarters / 4;
    let rem = total_quarters % 4;
    let partial = match rem {
        1 => "░",
        2 => "▒",
        3 => "▓",
        _ => "",
    };
    let filled_str = format!("{}{}", "█".repeat(full), partial);
    let filled_count = full + if rem > 0 { 1 } else { 0 };
    let empty_count = width_chars.saturating_sub(filled_count);
    let empty_str = "░".repeat(empty_count);
    (filled_str, empty_str)
}

impl App {
    pub(crate) fn render_visuals(&self, view: &mut View) {
        let (cols, _) = self.system_mode.char_grid();
        match cols {
            80 => self.render_visuals_80(view),
            40 => self.render_visuals_40(view),
            _ => self.render_visuals_32(view),
        }
    }

    pub(crate) fn render_visuals_80(&self, view: &mut View) {
        // Section Title (Row 3: y = 48)
        view.add(Element::Text(TextElement {
            x: 16,
            y: 48,
            text: "CRT & RETRO DISPLAY POST-PROCESSING (WebGL2):".to_string(),
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));

        view.add(Element::Text(TextElement {
            x: 16,
            y: 64,
            text: format!(
                "Preset: {:<7} • select a preset or tune each property below",
                self.visual_effects.preset_name()
            ),
            style: TextStyle::new(Color::from_palette(4)),
        }));

        // Presets row at Row 5: y = 80 composed of character buttons
        let presets = [
            (
                "A: Clean",
                16,
                self.visual_effects == VisualEffects::clean(),
            ),
            (
                "W: CRT",
                112,
                self.visual_effects == VisualEffects::crt_trinitron(),
            ),
            (
                "C: Arcade",
                208,
                self.visual_effects == VisualEffects::crt_arcade(),
            ),
            (
                "D: Bloom",
                312,
                self.visual_effects == VisualEffects::phosphor_bloom(),
            ),
            (
                "E: Glitch",
                408,
                self.visual_effects == VisualEffects::retro_glitch(),
            ),
            (
                "R: Reset",
                512,
                self.visual_effects == VisualEffects::default(),
            ),
        ];

        for (label, bx, active) in presets {
            let text = format!("[ {label} ]");
            let style = if active {
                TextStyle::new(Color::from_palette(0))
                    .with_bg(Color::from_palette(6))
                    .bold()
            } else {
                TextStyle::new(Color::from_palette(7)).with_bg(Color::from_palette(1))
            };
            view.add(Element::Text(TextElement {
                x: bx,
                y: 80,
                text,
                style,
            }));
        }

        // Horizontal divider composed of CP437 horizontal line characters (Row 6: y = 96)
        view.add(Element::Text(TextElement {
            x: 16,
            y: 96,
            text: "─".repeat(76),
            style: TextStyle::new(Color::from_palette(7)),
        }));

        // 9 Sliders at Rows 7..15: y = 112..240 (16px per row)
        let sliders = [
            ("[Z] Scanline Density:      ", self.visual_effects.scanlines),
            (
                "[X] Pixel Grid Separation: ",
                self.visual_effects.pixel_grid,
            ),
            ("[M] Chromatic Aberration:  ", self.visual_effects.chromatic),
            ("[V] Phosphor Afterglow:    ", self.visual_effects.afterglow),
            ("[B] CRT Barrel Curvature:  ", self.visual_effects.curvature),
            ("[N] Signal Jitter:         ", self.visual_effects.jitter),
            ("[G] Magnet Deflection:     ", self.visual_effects.magnet),
            (
                "[H] Sporadic Antenna Hum:  ",
                self.visual_effects.antenna_hum,
            ),
            ("[Y] White Noise Snow:      ", self.visual_effects.noise),
        ];

        for (i, (label, val)) in sliders.into_iter().enumerate() {
            let is_sel = i == self.selected_fx_slider;
            let sy = 112 + (i as u16) * 16;
            let prefix = if is_sel { "► " } else { "  " };

            // Label
            view.add(Element::Text(TextElement {
                x: 16,
                y: sy,
                text: format!("{prefix}{label}"),
                style: if is_sel {
                    TextStyle::new(Color::from_palette(14)).bold()
                } else {
                    TextStyle::new(Color::from_palette(7))
                },
            }));

            // 20-character slider composed entirely of CP437 block characters
            let (filled_str, empty_str) = compose_char_slider(val, 20);
            let slider_x = 248;

            // Opening bracket
            view.add(Element::Text(TextElement {
                x: slider_x,
                y: sy,
                text: "[".to_string(),
                style: TextStyle::new(if is_sel {
                    Color::from_palette(14)
                } else {
                    Color::from_palette(7)
                }),
            }));

            // Filled gauge blocks
            if !filled_str.is_empty() {
                view.add(Element::Text(TextElement {
                    x: slider_x + 8,
                    y: sy,
                    text: filled_str.clone(),
                    style: TextStyle::new(if is_sel {
                        Color::from_palette(14)
                    } else {
                        Color::from_palette(6)
                    })
                    .bold(),
                }));
            }

            // Empty track dots
            if !empty_str.is_empty() {
                let empty_x = slider_x + 8 + (filled_str.chars().count() as u16) * 8;
                view.add(Element::Text(TextElement {
                    x: empty_x,
                    y: sy,
                    text: empty_str,
                    style: TextStyle::new(if is_sel {
                        Color::from_palette(3)
                    } else {
                        Color::from_palette(1)
                    }),
                }));
            }

            // Closing bracket
            view.add(Element::Text(TextElement {
                x: slider_x + 8 + 20 * 8,
                y: sy,
                text: "]".to_string(),
                style: TextStyle::new(if is_sel {
                    Color::from_palette(14)
                } else {
                    Color::from_palette(7)
                }),
            }));

            // Percentage value
            view.add(Element::Text(TextElement {
                x: 428,
                y: sy,
                text: format!("{:3.0}%", val * 100.0),
                style: TextStyle::new(if is_sel {
                    Color::from_palette(14)
                } else {
                    Color::from_palette(5)
                }),
            }));

            // Click hint
            view.add(Element::Text(TextElement {
                x: 476,
                y: sy,
                text: "[Click / ◄ ► Tune]".to_string(),
                style: TextStyle::new(if is_sel {
                    Color::from_palette(12)
                } else {
                    Color::from_palette(8)
                }),
            }));
        }

        let border_color = Color::from_palette(7);

        // Visual Calibration Test Card container composed entirely of valid CP437 box-drawing characters
        let card_title = format!(
            " CALIBRATION TEST CARD & COLOR RAMP - SYSTEM: {} ",
            self.system_mode.name()
        );
        let total_cols: usize = 76;
        let dashes = total_cols.saturating_sub(card_title.chars().count() + 4);
        let top_border = format!(
            "┌──{}{}{}┐",
            card_title,
            "─".repeat(dashes),
            if (card_title.chars().count() + 4 + dashes) < total_cols {
                "─"
            } else {
                ""
            }
        );
        view.add(Element::Text(TextElement {
            x: 16,
            y: 272,
            text: top_border,
            style: TextStyle::new(border_color),
        }));

        // Row 1: 16 color swatches composed of valid CP437 full block characters '██' with side borders
        view.add(Element::Text(TextElement {
            x: 16,
            y: 288,
            text: "│".to_string(),
            style: TextStyle::new(border_color),
        }));
        for i in 0..16 {
            view.add(Element::Text(TextElement {
                x: 56 + i * 32,
                y: 288,
                text: "██".to_string(),
                style: TextStyle::new(Color::from_palette(i as u8)).bold(),
            }));
        }
        view.add(Element::Text(TextElement {
            x: 16 + 75 * 8,
            y: 288,
            text: "│".to_string(),
            style: TextStyle::new(border_color),
        }));

        // Row 2: Standard ASCII character set test
        view.add(Element::Text(TextElement {
            x: 16,
            y: 304,
            text: "│".to_string(),
            style: TextStyle::new(border_color),
        }));
        view.add(Element::Text(TextElement {
            x: 24,
            y: 304,
            text: " ABCDEFGHIJKLMNOPQRSTUVWXYZ 0123456789 !@#$%^&*()_+-=~[];',./             "
                .to_string(),
            style: TextStyle::new(Color::from_palette(5)),
        }));
        view.add(Element::Text(TextElement {
            x: 16 + 75 * 8,
            y: 304,
            text: "│".to_string(),
            style: TextStyle::new(border_color),
        }));

        // Row 3: Authentic CP437 raster glyphs
        view.add(Element::Text(TextElement {
            x: 16,
            y: 320,
            text: "│".to_string(),
            style: TextStyle::new(border_color),
        }));
        view.add(Element::Text(TextElement {
            x: 24,
            y: 320,
            text: " Raster glyphs: ░▒▓█ │┤┐└┴┬├─┼ ◄► ▲▼ ☼• ♠♣♥♦ (Hardware ROM Matrix)        "
                .to_string(),
            style: TextStyle::new(Color::from_palette(6)),
        }));
        view.add(Element::Text(TextElement {
            x: 16 + 75 * 8,
            y: 320,
            text: "│".to_string(),
            style: TextStyle::new(border_color),
        }));

        // Row 4: Shortcuts hint
        view.add(Element::Text(TextElement {
            x: 16,
            y: 336,
            text: "│".to_string(),
            style: TextStyle::new(border_color),
        }));
        view.add(Element::Text(TextElement {
            x: 24,
            y: 336,
            text: " Presets [A-R] │ FX [Z-N, G, H, Y] │ [▲/▼] Select │ [◄/►] Tune            "
                .to_string(),
            style: TextStyle::new(Color::from_palette(12)).bold(),
        }));
        view.add(Element::Text(TextElement {
            x: 16 + 75 * 8,
            y: 336,
            text: "│".to_string(),
            style: TextStyle::new(border_color),
        }));

        // Bottom border (76 chars)
        view.add(Element::Text(TextElement {
            x: 16,
            y: 352,
            text: format!("└{}┘", "─".repeat(74)),
            style: TextStyle::new(border_color),
        }));
    }

    pub(crate) fn render_visuals_40(&self, view: &mut View) {
        view.add(Element::Text(TextElement {
            x: 4,
            y: 12,
            text: "CRT DISPLAY EFFECTS (WebGL2):".to_string(),
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));

        view.add(Element::Text(TextElement {
            x: 4,
            y: 20,
            text: format!("PRESET: {}", self.visual_effects.preset_name()),
            style: TextStyle::new(Color::from_palette(4)),
        }));

        let presets = [
            ("CLN", 4, self.visual_effects == VisualEffects::clean()),
            (
                "CRT",
                68,
                self.visual_effects == VisualEffects::crt_trinitron(),
            ),
            (
                "ARC",
                132,
                self.visual_effects == VisualEffects::crt_arcade(),
            ),
            (
                "BLM",
                196,
                self.visual_effects == VisualEffects::phosphor_bloom(),
            ),
            (
                "GLT",
                260,
                self.visual_effects == VisualEffects::retro_glitch(),
            ),
        ];

        for (label, bx, active) in presets {
            let text = format!("[{label}]");
            let style = if active {
                TextStyle::new(Color::from_palette(0))
                    .with_bg(Color::from_palette(6))
                    .bold()
            } else {
                TextStyle::new(Color::from_palette(7)).with_bg(Color::from_palette(1))
            };
            view.add(Element::Text(TextElement {
                x: bx,
                y: 26,
                text,
                style,
            }));
        }

        let sliders = [
            ("Z:Scan ", self.visual_effects.scanlines),
            ("X:Grid ", self.visual_effects.pixel_grid),
            ("M:Chrom", self.visual_effects.chromatic),
            ("V:Glow ", self.visual_effects.afterglow),
            ("B:Curv ", self.visual_effects.curvature),
            ("N:Jitt ", self.visual_effects.jitter),
            ("G:Magn ", self.visual_effects.magnet),
            ("H:Hum  ", self.visual_effects.antenna_hum),
            ("Y:Nois ", self.visual_effects.noise),
        ];

        let mut sy = 45;
        for (i, (label, val)) in sliders.into_iter().enumerate() {
            let is_sel = i == self.selected_fx_slider;
            let prefix = if is_sel { "►" } else { " " };
            view.add(Element::Text(TextElement {
                x: 2,
                y: sy,
                text: format!("{prefix}{label}"),
                style: if is_sel {
                    TextStyle::new(Color::from_palette(14)).bold()
                } else {
                    TextStyle::new(Color::from_palette(7))
                },
            }));

            // 10-character gauge slider
            let (filled_str, empty_str) = compose_char_slider(val, 10);
            let slider_x = 92;

            view.add(Element::Text(TextElement {
                x: slider_x,
                y: sy,
                text: "[".to_string(),
                style: TextStyle::new(if is_sel {
                    Color::from_palette(14)
                } else {
                    Color::from_palette(7)
                }),
            }));

            if !filled_str.is_empty() {
                view.add(Element::Text(TextElement {
                    x: slider_x + 8,
                    y: sy,
                    text: filled_str.clone(),
                    style: TextStyle::new(if is_sel {
                        Color::from_palette(14)
                    } else {
                        Color::from_palette(6)
                    })
                    .bold(),
                }));
            }

            if !empty_str.is_empty() {
                let empty_x = slider_x + 8 + (filled_str.chars().count() as u16) * 8;
                view.add(Element::Text(TextElement {
                    x: empty_x,
                    y: sy,
                    text: empty_str,
                    style: TextStyle::new(if is_sel {
                        Color::from_palette(3)
                    } else {
                        Color::from_palette(1)
                    }),
                }));
            }

            view.add(Element::Text(TextElement {
                x: slider_x + 8 + 10 * 8,
                y: sy,
                text: "]".to_string(),
                style: TextStyle::new(if is_sel {
                    Color::from_palette(14)
                } else {
                    Color::from_palette(7)
                }),
            }));

            view.add(Element::Text(TextElement {
                x: 196,
                y: sy,
                text: format!("{:3.0}%", val * 100.0),
                style: TextStyle::new(Color::from_palette(5)),
            }));

            sy += 9;
        }

        // Swatches
        for i in 0..16 {
            view.add(Element::Text(TextElement {
                x: 4 + i * 19,
                y: 132,
                text: "██".to_string(),
                style: TextStyle::new(Color::from_palette(i as u8)).bold(),
            }));
        }

        view.add(Element::Text(TextElement {
            x: 4,
            y: 146,
            text: "ABCDEF 0123456789 ░▒▓█ ◄►▲▼".to_string(),
            style: TextStyle::new(Color::from_palette(7)),
        }));

        view.add(Element::Text(TextElement {
            x: 4,
            y: 160,
            text: "[A-E] Preset │ [Z-Y] FX │ [◄/►] Tune".to_string(),
            style: TextStyle::new(Color::from_palette(12)),
        }));
    }

    pub(crate) fn render_visuals_32(&self, view: &mut View) {
        view.add(Element::Text(TextElement {
            x: 2,
            y: 12,
            text: "CRT VISUAL FX (WebGL2):".to_string(),
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));

        view.add(Element::Text(TextElement {
            x: 2,
            y: 20,
            text: format!("PRESET:{}", self.visual_effects.preset_name()),
            style: TextStyle::new(Color::from_palette(4)),
        }));

        let presets = [
            ("CLN", 2, self.visual_effects == VisualEffects::clean()),
            (
                "CRT",
                52,
                self.visual_effects == VisualEffects::crt_trinitron(),
            ),
            (
                "ARC",
                102,
                self.visual_effects == VisualEffects::crt_arcade(),
            ),
            (
                "BLM",
                152,
                self.visual_effects == VisualEffects::phosphor_bloom(),
            ),
            (
                "GLT",
                202,
                self.visual_effects == VisualEffects::retro_glitch(),
            ),
        ];

        for (label, bx, active) in presets {
            let text = format!("[{label}]");
            let style = if active {
                TextStyle::new(Color::from_palette(0))
                    .with_bg(Color::from_palette(6))
                    .bold()
            } else {
                TextStyle::new(Color::from_palette(7)).with_bg(Color::from_palette(1))
            };
            view.add(Element::Text(TextElement {
                x: bx,
                y: 26,
                text,
                style,
            }));
        }

        let sliders = [
            ("Z:Scan", self.visual_effects.scanlines),
            ("X:Grid", self.visual_effects.pixel_grid),
            ("M:Chr ", self.visual_effects.chromatic),
            ("V:Glow", self.visual_effects.afterglow),
            ("B:Curv", self.visual_effects.curvature),
            ("N:Jitt", self.visual_effects.jitter),
            ("G:Magn", self.visual_effects.magnet),
            ("H:Hum ", self.visual_effects.antenna_hum),
            ("Y:Nois", self.visual_effects.noise),
        ];

        let mut sy = 44;
        for (i, (label, val)) in sliders.into_iter().enumerate() {
            let is_sel = i == self.selected_fx_slider;
            let prefix = if is_sel { "►" } else { " " };
            view.add(Element::Text(TextElement {
                x: 1,
                y: sy,
                text: format!("{prefix}{label}"),
                style: if is_sel {
                    TextStyle::new(Color::from_palette(14)).bold()
                } else {
                    TextStyle::new(Color::from_palette(7))
                },
            }));

            // 8-character gauge slider
            let (filled_str, empty_str) = compose_char_slider(val, 8);
            let slider_x = 64;

            view.add(Element::Text(TextElement {
                x: slider_x,
                y: sy,
                text: "[".to_string(),
                style: TextStyle::new(if is_sel {
                    Color::from_palette(14)
                } else {
                    Color::from_palette(7)
                }),
            }));

            if !filled_str.is_empty() {
                view.add(Element::Text(TextElement {
                    x: slider_x + 8,
                    y: sy,
                    text: filled_str.clone(),
                    style: TextStyle::new(if is_sel {
                        Color::from_palette(14)
                    } else {
                        Color::from_palette(6)
                    })
                    .bold(),
                }));
            }

            if !empty_str.is_empty() {
                let empty_x = slider_x + 8 + (filled_str.chars().count() as u16) * 8;
                view.add(Element::Text(TextElement {
                    x: empty_x,
                    y: sy,
                    text: empty_str,
                    style: TextStyle::new(if is_sel {
                        Color::from_palette(3)
                    } else {
                        Color::from_palette(1)
                    }),
                }));
            }

            view.add(Element::Text(TextElement {
                x: slider_x + 8 + 8 * 8,
                y: sy,
                text: "]".to_string(),
                style: TextStyle::new(if is_sel {
                    Color::from_palette(14)
                } else {
                    Color::from_palette(7)
                }),
            }));

            view.add(Element::Text(TextElement {
                x: 152,
                y: sy,
                text: format!("{:3.0}%", val * 100.0),
                style: TextStyle::new(Color::from_palette(5)),
            }));

            sy += 8;
        }

        // Swatches
        for i in 0..16 {
            view.add(Element::Text(TextElement {
                x: 2 + i * 15,
                y: 122,
                text: "█".to_string(),
                style: TextStyle::new(Color::from_palette(i as u8)).bold(),
            }));
        }

        view.add(Element::Text(TextElement {
            x: 2,
            y: 136,
            text: "0123456789 ZX-SPECTRUM".to_string(),
            style: TextStyle::new(Color::from_palette(7)),
        }));

        view.add(Element::Text(TextElement {
            x: 2,
            y: 150,
            text: "[A-E] Preset │ [◄/►] Tune".to_string(),
            style: TextStyle::new(Color::from_palette(12)),
        }));
    }
}
