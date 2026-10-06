//! Modal dialogs (Visuals, Help), article overlays (CV, About), and CRT screensaver.

use crate::data::{
    ABOUT_LINES_32, ABOUT_LINES_40, ABOUT_LINES_80, RESUME_LINES_32, RESUME_LINES_40,
    RESUME_LINES_80,
};
use crate::state::{App, Tab};
use pixel_ssh_view::{
    horizontal_scroll, Color, Element, LinkElement, RectElement, TextElement, TextStyle, View,
};

pub(crate) struct SystemDialogGeometry {
    pub box_x: u16,
    pub box_y: u16,
    pub box_w: u16,
    pub box_h: u16,
    pub box_cols: u16,
    pub box_rows: u16,
    pub char_h: u16,
}

impl App {
    pub(crate) fn standard_dialog_geometry(&self) -> SystemDialogGeometry {
        self.dialog_geometry(15)
    }

    fn dialog_geometry(&self, compact_rows: u16) -> SystemDialogGeometry {
        let (width, height) = self.view_dimensions();
        let (cols, _) = self.resolution.char_grid();
        let char_h = self.resolution.line_height();
        let box_cols = if cols >= 80 {
            76
        } else {
            cols.saturating_sub(2)
        };
        let box_rows = if cols >= 80 { 15 } else { compact_rows };
        let box_rows = box_rows.min((height / char_h).saturating_sub(2));
        let box_w = box_cols * 8;
        let box_h = box_rows * char_h;
        SystemDialogGeometry {
            box_x: (width.saturating_sub(box_w) / 16) * 8,
            box_y: ((height / char_h).saturating_sub(box_rows) / 2) * char_h,
            box_w,
            box_h,
            box_cols,
            box_rows,
            char_h,
        }
    }

    /// Renders an authentic retro 3D Starfield Warp screensaver with live glowing clock.
    pub(crate) fn render_screensaver(&self, view: &mut View, width: u16, height: u16) {
        view.add(Element::Rect(RectElement {
            x: 0,
            y: 0,
            width,
            height,
            color: Color::from_palette(0),
            filled: true,
        }));

        let cx = (width as f32) / 2.0;
        let cy = (height as f32) / 2.0;
        let t = (self.screensaver_tick as f32) * 0.12;

        // 3D Starfield Warp: 64 depth-projected phosphor stars
        for i in 0..64 {
            let seed = (i as f32 * 137.58 + 23.41).fract();
            let angle = (i as f32 * 0.618_034) * std::f32::consts::PI * 2.0 + (seed * 0.5);
            let z = (1.0 - ((t * 0.35 + i as f32 * 0.05) % 1.0)).max(0.02);
            let max_r = (width.max(height) as f32) * 0.65;
            let dist = (1.0 - z) * max_r;
            let sx = cx + angle.cos() * dist;
            let sy = cy + angle.sin() * dist;

            if sx >= 8.0 && sx < (width - 8) as f32 && sy >= 8.0 && sy < (height - 8) as f32 {
                let (ch, color_idx) = if z > 0.7 {
                    ('·', 4)
                } else if z > 0.4 {
                    ('+', 7)
                } else if z > 0.2 {
                    ('*', 14)
                } else {
                    ('O', 6)
                };

                let char_h = self.resolution.line_height() as f32;
                let px = ((sx / 8.0).floor() * 8.0) as u16;
                let py = ((sy / char_h).floor() * char_h) as u16;
                view.add(Element::Text(TextElement {
                    x: px,
                    y: py,
                    text: ch.to_string(),
                    style: TextStyle::new(Color::from_palette(color_idx)).bold(),
                }));
            }
        }

        // Live Center Clock Display with authentic CP437 double border and drop shadow
        let char_h = self.resolution.line_height();
        let clock_str = self.clock_formatted();
        let box_cols = 28u16.min(width / 8);
        let box_w = box_cols * 8;
        let box_rows = 4u16;
        let box_h = box_rows * char_h;
        let box_x = ((width.saturating_sub(box_w)) / 16) * 8;
        let box_y = ((height.saturating_sub(box_h)) / (char_h * 2)) * char_h;

        // Shadow
        view.add(Element::Rect(RectElement {
            x: box_x + 8,
            y: box_y + char_h,
            width: box_w,
            height: box_h,
            color: Color::from_palette(13),
            filled: true,
        }));
        // Box background
        view.add(Element::Rect(RectElement {
            x: box_x,
            y: box_y,
            width: box_w,
            height: box_h,
            color: Color::from_palette(1),
            filled: true,
        }));

        let inner_w = (box_cols.saturating_sub(2)) as usize;
        let top_border = format!("╔{}╗", "═".repeat(inner_w));
        let bot_border = format!("╚{}╝", "═".repeat(inner_w));

        let clock_pulse = match (self.screensaver_tick / 4) % 3 {
            0 => 6,
            1 => 14,
            _ => 7,
        };

        view.add(Element::Text(TextElement {
            x: box_x,
            y: box_y,
            text: top_border,
            style: TextStyle::new(Color::from_palette(7)).bold(),
        }));

        let clock_w = (clock_str.len() as u16) * 8;
        let clock_center_x = box_x + (box_w.saturating_sub(clock_w)) / 2;
        view.add(Element::Text(TextElement {
            x: box_x,
            y: box_y + char_h,
            text: "║".to_string(),
            style: TextStyle::new(Color::from_palette(7)),
        }));
        view.add(Element::Text(TextElement {
            x: clock_center_x,
            y: box_y + char_h,
            text: clock_str,
            style: TextStyle::new(Color::from_palette(clock_pulse)).bold(),
        }));
        view.add(Element::Text(TextElement {
            x: box_x + box_w.saturating_sub(8),
            y: box_y + char_h,
            text: "║".to_string(),
            style: TextStyle::new(Color::from_palette(7)),
        }));

        let label = "CRT PHOSPHOR SAVER";
        let label_w = (label.len() as u16) * 8;
        let label_x = box_x + (box_w.saturating_sub(label_w)) / 2;
        view.add(Element::Text(TextElement {
            x: box_x,
            y: box_y + 2 * char_h,
            text: "║".to_string(),
            style: TextStyle::new(Color::from_palette(7)),
        }));
        view.add(Element::Text(TextElement {
            x: label_x,
            y: box_y + 2 * char_h,
            text: label.to_string(),
            style: TextStyle::new(Color::from_palette(4)),
        }));
        view.add(Element::Text(TextElement {
            x: box_x + box_w.saturating_sub(8),
            y: box_y + 2 * char_h,
            text: "║".to_string(),
            style: TextStyle::new(Color::from_palette(7)),
        }));

        view.add(Element::Text(TextElement {
            x: box_x,
            y: box_y + 3 * char_h,
            text: bot_border,
            style: TextStyle::new(Color::from_palette(7)).bold(),
        }));

        let wake_text = "MOVE MOUSE OR PRESS ANY KEY TO WAKE";
        let wake_w = (wake_text.len() as u16) * 8;
        let wake_x = (width.saturating_sub(wake_w)) / 2;
        let wake_y = (height.saturating_sub(char_h * 2) / char_h) * char_h;
        view.add(Element::Text(TextElement {
            x: wake_x,
            y: wake_y,
            text: wake_text.to_string(),
            style: TextStyle::new(Color::from_palette(8)),
        }));
    }

    /// Renders About or CV (Resume) as a reader article window overlaying on top of Projects.
    pub(crate) fn render_article_overlay(
        &self,
        view: &mut View,
        _width: u16,
        height: u16,
        cols: u16,
    ) {
        let is_resume = self.current_tab == Tab::Resume;
        let article_title = if let Some(index) = self.document {
            crate::documents::DOCUMENTS[index].1
        } else if cols >= 80 {
            if is_resume {
                "ARTICLE: CURRICULUM VITAE"
            } else {
                "ARTICLE: ABOUT ARCHITECT"
            }
        } else if is_resume {
            "CV"
        } else {
            "ABOUT"
        };
        let char_h: u16 = self.resolution.line_height();

        let pad_cols = if cols >= 80 {
            2u16
        } else if cols >= 40 {
            1u16
        } else {
            0u16
        };
        let box_x = pad_cols * 8;
        let box_cols = cols.saturating_sub(pad_cols * 2);
        let box_w = box_cols * 8;
        let box_y = char_h * 2;
        let box_rows = ((height.saturating_sub(char_h * 5)) / char_h).max(4);
        let box_h = box_rows * char_h;

        // Dialog drop shadow (pure black index 13)
        view.add(Element::Rect(RectElement {
            x: box_x + 8,
            y: box_y + char_h,
            width: box_w,
            height: box_h,
            color: Color::from_palette(13),
            filled: true,
        }));

        // Dark card background
        view.add(Element::Rect(RectElement {
            x: box_x,
            y: box_y,
            width: box_w,
            height: box_h,
            color: Color::from_palette(0),
            filled: true,
        }));

        // Header bar inside article box
        view.add(Element::Rect(RectElement {
            x: box_x,
            y: box_y,
            width: box_w,
            height: char_h,
            color: Color::from_palette(1),
            filled: true,
        }));

        // Top Border with embedded Title and Close indicator slot
        let title_part = format!("╔═ [ {article_title} ] ");
        let close_btn_text = if cols >= 80 { "[ESC: X]" } else { "[X]" };
        let close_spaces = " ".repeat(close_btn_text.chars().count() + 2);
        let close_slot = format!("{close_spaces}═╗");
        let filler_len = (box_cols as usize)
            .saturating_sub(title_part.chars().count() + close_slot.chars().count());
        let top_str = format!("{}{}{}", title_part, "═".repeat(filler_len), close_slot);
        view.add(Element::Text(TextElement {
            x: box_x,
            y: box_y,
            text: top_str,
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));

        // Clickable close button overlay aligned exactly in the slot
        let close_x =
            (box_x + box_w).saturating_sub(((close_btn_text.chars().count() + 2) as u16) * 8);
        view.add(Element::Link(LinkElement::new(
            close_x,
            box_y,
            close_btn_text,
            "#close-article",
            TextStyle::new(Color::from_palette(10)).bold(),
        )));

        let document_lines: Vec<_> = self
            .document_lines
            .iter()
            .map(|(s, p, b)| (s.as_str(), *p, *b))
            .collect();
        let (lines, scroll_offset, max_scroll): (&[(&str, u8, bool)], usize, usize) =
            if self.document.is_some() {
                (&document_lines, self.about_scroll, self.about_max_scroll())
            } else if is_resume {
                let l: &[(&str, u8, bool)] = match cols {
                    100 | 80 => RESUME_LINES_80,
                    40 => RESUME_LINES_40,
                    _ => RESUME_LINES_32,
                };
                (l, self.resume_scroll, self.resume_max_scroll())
            } else {
                let l: &[(&str, u8, bool)] = match cols {
                    100 | 80 => ABOUT_LINES_80,
                    40 => ABOUT_LINES_40,
                    _ => ABOUT_LINES_32,
                };
                (l, self.about_scroll, self.about_max_scroll())
            };

        let visible_lines = (box_rows.saturating_sub(2)) as usize;
        let content_y_start = box_y + char_h;

        for (i, (text, pal_idx, bold)) in lines.iter().enumerate() {
            let row_idx = (i as i32) - (scroll_offset as i32);
            if row_idx >= 0 && (row_idx as usize) < visible_lines {
                let y = content_y_start + (row_idx as u16) * char_h;
                let mut style = TextStyle::new(Color::from_palette(*pal_idx));
                if *bold {
                    style = style.bold();
                }
                view.add(Element::Text(TextElement {
                    x: box_x
                        + if cols <= 32 {
                            8
                        } else if cols == 40 {
                            0
                        } else {
                            16
                        },
                    y,
                    text: horizontal_scroll(
                        text,
                        box_cols.saturating_sub(if cols == 40 {
                            0
                        } else if cols <= 32 {
                            1
                        } else {
                            2
                        }) as usize,
                        self.tick,
                    ),
                    style,
                }));
            }
        }

        // On Web, render high-resolution dithered portrait sprite inside the framed architect card
        if !is_resume
            && self.document.is_none()
            && self.platform == pixel_ssh_view::Platform::Web
            && scroll_offset == 0
        {
            let sprite_size = if cols >= 80 { 96 } else { 64 };
            let sprite_x = box_x + 16;
            let sprite_y = content_y_start + char_h;
            if sprite_y + sprite_size < (box_y + box_h).saturating_sub(char_h) {
                view.add(Element::Sprite(pixel_ssh_view::SpriteElement {
                    x: sprite_x,
                    y: sprite_y,
                    width: sprite_size,
                    height: sprite_size,
                    data: crate::data::profile::get_profile_sprite(self.palette_mode, sprite_size)
                        .to_vec(),
                }));
            }
        }

        // Side borders and Authentic CP437 Text-Mode Scrollbar on the right border
        for r in 1..(box_rows - 1) {
            let ry = box_y + r * char_h;
            view.add(Element::Text(TextElement {
                x: box_x,
                y: ry,
                text: "║".to_string(),
                style: TextStyle::new(Color::from_palette(3)),
            }));

            let (scroll_char, scroll_fg) = if max_scroll > 0 && box_rows > 3 {
                if r == 1 {
                    ("▲", 6)
                } else if r == box_rows - 2 {
                    ("▼", 6)
                } else {
                    let track_len = (box_rows - 4).max(1) as usize;
                    let thumb_idx = ((scroll_offset as f32 / max_scroll as f32)
                        * ((track_len.saturating_sub(1)) as f32))
                        .round() as usize;
                    if (r - 2) as usize == thumb_idx {
                        ("█", 14)
                    } else {
                        ("░", 3)
                    }
                }
            } else {
                ("║", 3)
            };

            view.add(Element::Text(TextElement {
                x: box_x + box_w.saturating_sub(8),
                y: ry,
                text: scroll_char.to_string(),
                style: TextStyle::new(Color::from_palette(scroll_fg)),
            }));
        }

        // Bottom status / close prompt
        let bot_y = box_y + (box_rows - 1) * char_h;
        view.add(Element::Rect(RectElement {
            x: box_x,
            y: bot_y,
            width: box_w,
            height: char_h,
            color: Color::from_palette(1),
            filled: true,
        }));
        let prompt_text = if cols >= 80 {
            "╚══ [j/k/Wheel] Scroll • [ESC] Return "
        } else if cols >= 40 {
            "╚═ [j/k] Scroll • [ESC] "
        } else {
            "╚═ [j/k] • [ESC] "
        };
        let filler_len = (box_cols as usize).saturating_sub(prompt_text.chars().count() + 1);
        let bot_border = format!("{}{}{}", prompt_text, "═".repeat(filler_len), "╝");
        view.add(Element::Text(TextElement {
            x: box_x,
            y: bot_y,
            text: bot_border,
            style: TextStyle::new(Color::from_palette(4)),
        }));
    }

    /// Renders the Visuals effects modal dialog.
    pub(crate) fn render_visuals_modal(
        &self,
        view: &mut View,
        _width: u16,
        _height: u16,
        cols: u16,
    ) {
        if cols < 80 {
            self.render_compact_visuals_modal(view, cols);
            return;
        }
        let SystemDialogGeometry {
            box_x,
            box_y,
            box_w,
            box_h,
            box_cols,
            box_rows,
            char_h,
        } = self.standard_dialog_geometry();

        // Dialog drop shadow (pure black index 13)
        view.add(Element::Rect(RectElement {
            x: box_x + 8,
            y: box_y + char_h,
            width: box_w,
            height: box_h,
            color: Color::from_palette(13),
            filled: true,
        }));

        // Dialog background
        view.add(Element::Rect(RectElement {
            x: box_x,
            y: box_y,
            width: box_w,
            height: box_h,
            color: Color::from_palette(0),
            filled: true,
        }));

        // Header bar
        view.add(Element::Rect(RectElement {
            x: box_x,
            y: box_y,
            width: box_w,
            height: char_h,
            color: Color::from_palette(1),
            filled: true,
        }));

        // Top Border with embedded Title and Close indicator slot
        let title = "CRT DISPLAY & VISUAL EFFECTS";
        let title_part = format!("╔═ [ {title} ] ");
        let close_slot = "      ═╗";
        let filler_len = (box_cols as usize)
            .saturating_sub(title_part.chars().count() + close_slot.chars().count());
        let top_str = format!("{}{}{}", title_part, "═".repeat(filler_len), close_slot);
        view.add(Element::Text(TextElement {
            x: box_x,
            y: box_y,
            text: top_str,
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));

        // Clickable close button overlay
        let close_x = (box_x + box_w).saturating_sub(48);
        view.add(Element::Link(LinkElement::new(
            close_x,
            box_y,
            "[X]",
            "#close-modal",
            TextStyle::new(Color::from_palette(10)).bold(),
        )));

        // Side borders
        for r in 1..(box_rows - 1) {
            let ry = box_y + r * char_h;
            view.add(Element::Text(TextElement {
                x: box_x,
                y: ry,
                text: "║".to_string(),
                style: TextStyle::new(Color::from_palette(3)),
            }));
            view.add(Element::Text(TextElement {
                x: box_x + box_w.saturating_sub(8),
                y: ry,
                text: "║".to_string(),
                style: TextStyle::new(Color::from_palette(3)),
            }));
        }

        // Presets row at row 1
        let preset_y = box_y + char_h;
        view.add(Element::Text(TextElement {
            x: box_x + 24,
            y: preset_y,
            text: "PRESETS:".to_string(),
            style: TextStyle::new(Color::from_palette(4)).bold(),
        }));
        let presets = [
            ("Clean", "#clean", 104),
            ("Trinitron", "#trinitron", 176),
            ("Arcade", "#arcade", 280),
            ("Bloom", "#bloom", 368),
            ("Glitch", "#glitch", 440),
        ];
        for (name, link, offset) in presets {
            if box_x + offset + 48 < box_x + box_w {
                view.add(Element::Link(LinkElement::new(
                    box_x + offset,
                    preset_y,
                    format!("[{name}]"),
                    link,
                    TextStyle::new(Color::from_palette(
                        if self.visual_effects.preset_name()
                            == match name {
                                "Trinitron" => "CRT",
                                other => other,
                            }
                        {
                            6
                        } else {
                            12
                        },
                    ))
                    .bold(),
                )));
            }
        }

        // Horizontal divider between presets and sliders
        let sep_str = format!("╟{}╢", "─".repeat((box_cols.saturating_sub(2)) as usize));
        view.add(Element::Text(TextElement {
            x: box_x,
            y: box_y + 2 * char_h,
            text: sep_str,
            style: TextStyle::new(Color::from_palette(3)),
        }));
        view.add(Element::Text(TextElement {
            x: box_x + 24,
            y: box_y + 2 * char_h,
            text: format!(" Preset: {} ", self.visual_effects.preset_name()),
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));

        let sliders = [
            ("Scanlines", self.visual_effects.scanlines),
            ("Pixel Grid", self.visual_effects.pixel_grid),
            ("Chromatic", self.visual_effects.chromatic),
            ("Afterglow", self.visual_effects.afterglow),
            ("Curvature", self.visual_effects.curvature),
            ("Jitter", self.visual_effects.jitter),
            ("Magnet", self.visual_effects.magnet),
            ("Hum / RF", self.visual_effects.antenna_hum),
            ("White Noise", self.visual_effects.noise),
        ];

        let slider_start_y = box_y + 3 * char_h;
        for (i, (name, val)) in sliders.iter().enumerate() {
            let y = slider_start_y + (i as u16) * char_h;
            if y + char_h >= box_y + box_h {
                break;
            }

            let pct = (*val * 100.0).round() as u8;
            let num_blocks = (pct as usize * 14) / 100;
            let mut bar = String::with_capacity(16);
            for b in 0..14 {
                if b < num_blocks {
                    bar.push('█');
                } else {
                    bar.push('░');
                }
            }

            let marker = if i == self.selected_fx_slider {
                "►"
            } else {
                " "
            };
            let text = format!("{marker}{name:<12} [ {bar} ] {pct:>3}%");
            view.add(Element::Text(TextElement {
                x: box_x + 16,
                y,
                text,
                style: TextStyle::new(Color::from_palette(if i == self.selected_fx_slider {
                    6
                } else {
                    5
                })),
            }));
        }

        // Bottom border
        let bot_y = box_y + (box_rows - 1) * char_h;
        view.add(Element::Rect(RectElement {
            x: box_x,
            y: bot_y,
            width: box_w,
            height: char_h,
            color: Color::from_palette(1),
            filled: true,
        }));
        let prompt = if cols >= 80 {
            "╚══ Click presets or sliders to adjust • [ESC] Close "
        } else {
            "╚═ Click presets/sliders • [ESC] "
        };
        let filler_len = (box_cols as usize).saturating_sub(prompt.chars().count() + 1);
        let bot_border = format!("{}{}{}", prompt, "═".repeat(filler_len), "╝");
        view.add(Element::Text(TextElement {
            x: box_x,
            y: bot_y,
            text: bot_border,
            style: TextStyle::new(Color::from_palette(4)),
        }));
    }

    fn render_compact_visuals_modal(&self, view: &mut View, cols: u16) {
        let SystemDialogGeometry {
            box_x,
            box_y,
            box_w,
            box_h,
            box_cols,
            box_rows,
            char_h,
        } = self.standard_dialog_geometry();
        view.add(Element::Rect(RectElement {
            x: box_x + 8,
            y: box_y + char_h,
            width: box_w,
            height: box_h,
            color: Color::from_palette(13),
            filled: true,
        }));
        view.add(Element::Rect(RectElement {
            x: box_x,
            y: box_y,
            width: box_w,
            height: box_h,
            color: Color::from_palette(0),
            filled: true,
        }));
        view.add(Element::Rect(RectElement {
            x: box_x,
            y: box_y,
            width: box_w,
            height: char_h,
            color: Color::from_palette(1),
            filled: true,
        }));
        let title = "VISUALS";
        let top_start = format!("╔═[ {title} ]");
        let top = format!(
            "{}{}══╗",
            top_start,
            "═".repeat((box_cols as usize).saturating_sub(top_start.chars().count() + 3))
        );
        view.add(Element::Text(TextElement {
            x: box_x,
            y: box_y,
            text: top,
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));
        view.add(Element::Link(LinkElement::new(
            box_x + box_w.saturating_sub(32),
            box_y,
            "X",
            "#close-modal",
            TextStyle::new(Color::from_palette(10)).bold(),
        )));
        for row in 1..box_rows.saturating_sub(1) {
            let y = box_y + row * char_h;
            view.add(Element::Text(TextElement {
                x: box_x,
                y,
                text: "║".to_string(),
                style: TextStyle::new(Color::from_palette(3)),
            }));
            view.add(Element::Text(TextElement {
                x: box_x + box_w.saturating_sub(8),
                y,
                text: "║".to_string(),
                style: TextStyle::new(Color::from_palette(3)),
            }));
        }
        let preset = self.visual_effects.preset_name();
        let preset_text = format!("PRESET: {preset}");
        view.add(Element::Text(TextElement {
            x: box_x + 8,
            y: box_y + char_h,
            text: preset_text.chars().take((box_cols - 2) as usize).collect(),
            style: TextStyle::new(Color::from_palette(4)).bold(),
        }));
        let sliders = [
            ("Scan", self.visual_effects.scanlines),
            ("Grid", self.visual_effects.pixel_grid),
            ("Chrom", self.visual_effects.chromatic),
            ("Glow", self.visual_effects.afterglow),
            ("Curve", self.visual_effects.curvature),
            ("Jitt", self.visual_effects.jitter),
            ("Magn", self.visual_effects.magnet),
            ("Hum", self.visual_effects.antenna_hum),
            ("Noise", self.visual_effects.noise),
        ];
        for (index, (name, value)) in sliders.iter().enumerate() {
            let y = box_y + (index as u16 + 2) * char_h;
            let selected = index == self.selected_fx_slider;
            let marker = if selected { "►" } else { " " };
            let width = if cols == 40 { 10 } else { 7 };
            let filled = ((*value * width as f32).round() as usize).min(width);
            let line = format!(
                "{marker}{name:<5}[{}{}]{:>3}%",
                "█".repeat(filled),
                "░".repeat(width - filled),
                (*value * 100.0).round() as u8
            );
            view.add(Element::Text(TextElement {
                x: box_x + 8,
                y,
                text: line.chars().take((box_cols - 2) as usize).collect(),
                style: if selected {
                    TextStyle::new(Color::from_palette(6)).bold()
                } else {
                    TextStyle::new(Color::from_palette(5))
                },
            }));
        }
        view.add(Element::Text(TextElement {
            x: box_x + 8,
            y: box_y + 12 * char_h,
            text: "1 Cl 2 Cr 3 Ar 4 Bl 5 Gl".to_string(),
            style: TextStyle::new(Color::from_palette(4)),
        }));
        let bottom_y = box_y + (box_rows - 1) * char_h;
        view.add(Element::Rect(RectElement {
            x: box_x,
            y: bottom_y,
            width: box_w,
            height: char_h,
            color: Color::from_palette(1),
            filled: true,
        }));
        let prompt = "╚═ ◄► tune  ESC close ";
        let bottom = format!(
            "{}{}╝",
            prompt,
            "═".repeat((box_cols as usize).saturating_sub(prompt.chars().count() + 1))
        );
        view.add(Element::Text(TextElement {
            x: box_x,
            y: bottom_y,
            text: bottom,
            style: TextStyle::new(Color::from_palette(4)),
        }));
    }

    /// Renders the Help & keyboard navigation modal dialog.
    pub(crate) fn render_help_modal(&self, view: &mut View, _width: u16, _height: u16, cols: u16) {
        if cols < 80 {
            self.render_compact_help_modal(view, cols);
            return;
        }
        let SystemDialogGeometry {
            box_x,
            box_y,
            box_w,
            box_h,
            box_cols,
            box_rows,
            char_h,
        } = self.standard_dialog_geometry();

        // Dialog drop shadow (pure black index 13)
        view.add(Element::Rect(RectElement {
            x: box_x + 8,
            y: box_y + char_h,
            width: box_w,
            height: box_h,
            color: Color::from_palette(13),
            filled: true,
        }));

        // Dialog background
        view.add(Element::Rect(RectElement {
            x: box_x,
            y: box_y,
            width: box_w,
            height: box_h,
            color: Color::from_palette(0),
            filled: true,
        }));

        // Header bar
        view.add(Element::Rect(RectElement {
            x: box_x,
            y: box_y,
            width: box_w,
            height: char_h,
            color: Color::from_palette(1),
            filled: true,
        }));

        // Top Border with embedded Title and Close indicator slot
        let title = "SYSTEM HELP & KEYBOARD SHORTCUTS";
        let title_part = format!("╔═ [ {title} ] ");
        let close_slot = "      ═╗";
        let filler_len = (box_cols as usize)
            .saturating_sub(title_part.chars().count() + close_slot.chars().count());
        let top_str = format!("{}{}{}", title_part, "═".repeat(filler_len), close_slot);
        view.add(Element::Text(TextElement {
            x: box_x,
            y: box_y,
            text: top_str,
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));

        // Clickable close button overlay
        let close_x = (box_x + box_w).saturating_sub(48);
        view.add(Element::Link(LinkElement::new(
            close_x,
            box_y,
            "[X]",
            "#close-modal",
            TextStyle::new(Color::from_palette(10)).bold(),
        )));

        // Side borders
        for r in 1..(box_rows - 1) {
            let ry = box_y + r * char_h;
            view.add(Element::Text(TextElement {
                x: box_x,
                y: ry,
                text: "║".to_string(),
                style: TextStyle::new(Color::from_palette(3)),
            }));
            view.add(Element::Text(TextElement {
                x: box_x + box_w.saturating_sub(8),
                y: ry,
                text: "║".to_string(),
                style: TextStyle::new(Color::from_palette(3)),
            }));
        }

        let shortcuts = [
            (
                "P L H S V Q",
                "Projects, Links, Help, cycle System, Visuals, Quit",
            ),
            ("Enter", "Open project details / Return to catalog"),
            ("h / l", "Navigate previous / next project in detail view"),
            (
                "j / k",
                "Scroll catalog, article, or project details up/down",
            ),
            ("Wheel", "Smooth vertical mouse-wheel scrolling"),
            ("v / V", "Toggle CRT Visual Effects modal dialog"),
            (
                "s / S",
                "Cycle system (SVGA, SGA, VGA, EGA, CGA, C64, Atari, ZX)",
            ),
            ("? / h", "Toggle this Help & Navigation shortcuts dialog"),
            (
                "Top-R",
                "Move mouse to Top-Right corner for CRT screensaver",
            ),
            (
                "ESC  ",
                "Close active modal dialog / Return to Projects view",
            ),
        ];

        let row_start_y = box_y + char_h;
        for (i, (keys, desc)) in shortcuts.iter().enumerate() {
            let y = row_start_y + ((i + 1) as u16) * char_h;
            if y + char_h >= box_y + box_h {
                break;
            }

            let text = format!("{keys:<7} : {desc}");
            view.add(Element::Text(TextElement {
                x: box_x + 24,
                y,
                text,
                style: TextStyle::new(Color::from_palette(5)),
            }));
        }

        // Bottom border
        let bot_y = box_y + (box_rows - 1) * char_h;
        view.add(Element::Rect(RectElement {
            x: box_x,
            y: bot_y,
            width: box_w,
            height: char_h,
            color: Color::from_palette(1),
            filled: true,
        }));
        let prompt = if cols >= 80 {
            "╚══ Press [ESC] or click [X] to return "
        } else {
            "╚═ [ESC] Return "
        };
        let filler_len = (box_cols as usize).saturating_sub(prompt.chars().count() + 1);
        let bot_border = format!("{}{}{}", prompt, "═".repeat(filler_len), "╝");
        view.add(Element::Text(TextElement {
            x: box_x,
            y: bot_y,
            text: bot_border,
            style: TextStyle::new(Color::from_palette(4)),
        }));
    }

    fn render_compact_help_modal(&self, view: &mut View, cols: u16) {
        let SystemDialogGeometry {
            box_x,
            box_y,
            box_w,
            box_h,
            box_cols,
            box_rows,
            char_h,
        } = self.standard_dialog_geometry();
        view.add(Element::Rect(RectElement {
            x: box_x + 8,
            y: box_y + char_h,
            width: box_w,
            height: box_h,
            color: Color::from_palette(13),
            filled: true,
        }));
        view.add(Element::Rect(RectElement {
            x: box_x,
            y: box_y,
            width: box_w,
            height: box_h,
            color: Color::from_palette(0),
            filled: true,
        }));
        view.add(Element::Rect(RectElement {
            x: box_x,
            y: box_y,
            width: box_w,
            height: char_h,
            color: Color::from_palette(1),
            filled: true,
        }));
        let title = "HELP";
        let start = format!("╔═[ {title} ]");
        let top = format!(
            "{}{}══╗",
            start,
            "═".repeat((box_cols as usize).saturating_sub(start.chars().count() + 3))
        );
        view.add(Element::Text(TextElement {
            x: box_x,
            y: box_y,
            text: top,
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));
        view.add(Element::Link(LinkElement::new(
            box_x + box_w.saturating_sub(32),
            box_y,
            "X",
            "#close-modal",
            TextStyle::new(Color::from_palette(10)).bold(),
        )));
        for row in 1..box_rows.saturating_sub(1) {
            let y = box_y + row * char_h;
            view.add(Element::Text(TextElement {
                x: box_x,
                y,
                text: "║".to_string(),
                style: TextStyle::new(Color::from_palette(3)),
            }));
            view.add(Element::Text(TextElement {
                x: box_x + box_w.saturating_sub(8),
                y,
                text: "║".to_string(),
                style: TextStyle::new(Color::from_palette(3)),
            }));
        }
        let rows = if cols == 40 {
            [
                "P  Projects",
                "H/? Help",
                "S  Cycle system",
                "V  Visuals",
                "Q  Return/Quit",
                "UP/DN j/k Scroll",
                "LT/RT h/l Detail",
                "ENT Open/return",
                "ESC Close dialog",
            ]
        } else {
            [
                "P Projects",
                "H/? Help",
                "S Cycle system",
                "V Visuals",
                "Q Return",
                "UP/DN j/k Scroll",
                "LT/RT h/l Detail",
                "ENT Open/back",
                "ESC Close",
            ]
        };
        for (index, line) in rows.iter().enumerate() {
            let y = box_y + (index as u16 + 1) * char_h;
            if y >= box_y + (box_rows - 1) * char_h {
                break;
            }
            view.add(Element::Text(TextElement {
                x: box_x + 8,
                y,
                text: line.chars().take((box_cols - 2) as usize).collect(),
                style: TextStyle::new(Color::from_palette(5)),
            }));
        }
        let bottom_y = box_y + (box_rows - 1) * char_h;
        view.add(Element::Rect(RectElement {
            x: box_x,
            y: bottom_y,
            width: box_w,
            height: char_h,
            color: Color::from_palette(1),
            filled: true,
        }));
        let prompt = "╚═ ESC close ";
        let bottom = format!(
            "{}{}╝",
            prompt,
            "═".repeat((box_cols as usize).saturating_sub(prompt.chars().count() + 1))
        );
        view.add(Element::Text(TextElement {
            x: box_x,
            y: bottom_y,
            text: bottom,
            style: TextStyle::new(Color::from_palette(4)),
        }));
    }
}
