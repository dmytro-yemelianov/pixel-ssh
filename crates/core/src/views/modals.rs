//! Modal dialogs (System, Visuals, Help), article overlays (CV, About), and CRT screensaver.

use pixel_ssh_view::{Color, Element, LinkElement, RectElement, TextElement, TextStyle, View};
use crate::data::{ABOUT_LINES_32, ABOUT_LINES_40, ABOUT_LINES_80, RESUME_LINES_32, RESUME_LINES_40, RESUME_LINES_80};
use crate::state::{App, Tab};

impl App {
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
            let angle = (i as f32 * 0.6180339887) * std::f32::consts::PI * 2.0 + (seed * 0.5);
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

                let char_h = if height <= 200 { 8.0 } else { 16.0 };
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

        // Live Center Clock Display with glowing retro border
        let clock_str = self.clock_formatted();
        let box_w = 200u16.min(width.saturating_sub(16));
        let box_h = 44u16.min(height.saturating_sub(16));
        let box_x = (width.saturating_sub(box_w)) / 2;
        let box_y = (height.saturating_sub(box_h)) / 2;

        view.add(Element::Rect(RectElement {
            x: box_x,
            y: box_y,
            width: box_w,
            height: box_h,
            color: Color::from_palette(1),
            filled: true,
        }));
        view.add(Element::Rect(RectElement {
            x: box_x,
            y: box_y,
            width: box_w,
            height: box_h,
            color: Color::from_palette(7),
            filled: false,
        }));

        let clock_pulse = match (self.screensaver_tick / 4) % 3 {
            0 => 6,
            1 => 14,
            _ => 7,
        };
        let clock_w = (clock_str.len() as u16) * 8;
        let clock_center_x = box_x + (box_w.saturating_sub(clock_w)) / 2;
        view.add(Element::Text(TextElement {
            x: clock_center_x,
            y: box_y + 6,
            text: clock_str,
            style: TextStyle::new(Color::from_palette(clock_pulse)).bold(),
        }));

        let label = "CRT PHOSPHOR SAVER";
        let label_w = (label.len() as u16) * 8;
        let label_x = box_x + (box_w.saturating_sub(label_w)) / 2;
        view.add(Element::Text(TextElement {
            x: label_x,
            y: box_y + 24,
            text: label.to_string(),
            style: TextStyle::new(Color::from_palette(4)),
        }));

        let wake_text = "MOVE MOUSE OR PRESS ANY KEY TO WAKE";
        let wake_w = (wake_text.len() as u16) * 8;
        let wake_x = (width.saturating_sub(wake_w)) / 2;
        let wake_y = height.saturating_sub(20);
        view.add(Element::Text(TextElement {
            x: wake_x,
            y: wake_y,
            text: wake_text.to_string(),
            style: TextStyle::new(Color::from_palette(8)),
        }));
    }

    /// Renders About or CV (Resume) as a reader article window overlaying on top of Projects.
    pub(crate) fn render_article_overlay(&self, view: &mut View, width: u16, height: u16, cols: u16) {
        let is_resume = self.current_tab == Tab::Resume;
        let article_title = if cols >= 80 {
            if is_resume { "ARTICLE: CURRICULUM VITAE" } else { "ARTICLE: ABOUT ARCHITECT" }
        } else if is_resume {
            "CV"
        } else {
            "ABOUT"
        };
        let char_h: u16 = self.palette_mode.line_height();

        let pad_x = if cols >= 80 { 16u16 } else { 4u16 };
        let pad_top = char_h * 2;
        let pad_bot = char_h * 2;

        let box_x = pad_x;
        let box_y = pad_top;
        let box_w = width.saturating_sub(pad_x * 2);
        let box_h = ((height.saturating_sub(pad_top + pad_bot)) / char_h) * char_h;

        // Dark card background & double border
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
            height: box_h,
            color: Color::from_palette(7),
            filled: false,
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

        view.add(Element::Text(TextElement {
            x: box_x + 4,
            y: box_y,
            text: format!("╔═ [ {} ]", article_title),
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));

        // Close button at top right of article
        let close_btn_text = if cols >= 80 { "[ESC: X]" } else { "[X]" };
        let close_btn_w = (close_btn_text.len() as u16) * 8;
        let close_x = (box_x + box_w).saturating_sub(close_btn_w + 4);
        view.add(Element::Link(LinkElement::new(
            close_x,
            box_y,
            close_btn_text,
            "#close-article",
            TextStyle::new(Color::from_palette(10)).bold(),
        )));

        let (lines, scroll_offset, max_scroll): (&[(&str, u8, bool)], usize, usize) = if is_resume {
            let l: &[(&str, u8, bool)] = match cols {
                100 | 80 => &RESUME_LINES_80,
                40 => &RESUME_LINES_40,
                _ => &RESUME_LINES_32,
            };
            (l, self.resume_scroll, self.resume_max_scroll())
        } else {
            let l: &[(&str, u8, bool)] = match cols {
                100 | 80 => &ABOUT_LINES_80,
                40 => &ABOUT_LINES_40,
                _ => &ABOUT_LINES_32,
            };
            (l, self.about_scroll, self.about_max_scroll())
        };

        let visible_lines = ((box_h.saturating_sub(char_h * 2)) / char_h) as usize;
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
                    x: box_x + 8,
                    y,
                    text: text.to_string(),
                    style,
                }));
            }
        }

        // On Web, render high-resolution dithered portrait sprite inside the framed architect card
        if !is_resume && self.platform == pixel_ssh_view::Platform::Web && scroll_offset == 0 {
            let sprite_size = if cols >= 80 { 96 } else { 64 };
            let sprite_x = box_x + 16;
            let sprite_y = content_y_start + char_h;
            if sprite_y + sprite_size < (box_y + box_h).saturating_sub(char_h) {
                view.add(Element::Sprite(pixel_ssh_view::SpriteElement {
                    x: sprite_x,
                    y: sprite_y,
                    width: sprite_size,
                    height: sprite_size,
                    data: crate::data::profile::get_profile_sprite(self.palette_mode, sprite_size).to_vec(),
                }));
            }
        }

        // Scrollbar on the right border
        let scroll_x = (box_x + box_w).saturating_sub(12);
        let track_h = box_h.saturating_sub(char_h * 2);
        if max_scroll > 0 && track_h > char_h {
            let thumb_ratio = (scroll_offset as f32) / (max_scroll as f32);
            let thumb_pos = (thumb_ratio * (track_h.saturating_sub(char_h) as f32)) as u16;
            let thumb_y = content_y_start + ((thumb_pos / char_h) * char_h);
            view.add(Element::Rect(RectElement {
                x: scroll_x,
                y: thumb_y,
                width: 6,
                height: char_h,
                color: Color::from_palette(7),
                filled: true,
            }));
        }

        // Bottom status / close prompt
        let bot_y = (box_y + box_h).saturating_sub(char_h);
        view.add(Element::Rect(RectElement {
            x: box_x,
            y: bot_y,
            width: box_w,
            height: char_h,
            color: Color::from_palette(1),
            filled: true,
        }));
        let prompt_text = if cols >= 80 {
            "[j/k/Wheel] Scroll  •  Press [ESC] to return to Projects"
        } else if cols >= 40 {
            "[j/k] Scroll • [ESC] Return"
        } else {
            "[j/k] • [ESC]"
        };
        view.add(Element::Text(TextElement {
            x: box_x + 4,
            y: bot_y,
            text: prompt_text.to_string(),
            style: TextStyle::new(Color::from_palette(4)),
        }));
    }

    /// Renders the System mode / Resolution & Color Theme picker modal dialog.
    pub(crate) fn render_system_modal(&self, view: &mut View, width: u16, height: u16, cols: u16) {
        let char_h: u16 = if height <= 200 { 8 } else { 16 };
        let box_w = if cols >= 80 { 580u16.min(width.saturating_sub(24)) } else { width.saturating_sub(16) };
        let box_h = (13 * char_h + 16).min(height.saturating_sub(24));
        let box_x = (width.saturating_sub(box_w)) / 2;
        let box_y = (height.saturating_sub(box_h)) / 2;

        // Authentic Volkov Commander drop shadow (pure black index 13)
        view.add(Element::Rect(RectElement {
            x: box_x + 8,
            y: box_y + 8,
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
            height: box_h,
            color: Color::from_palette(3),
            filled: false,
        }));

        view.add(Element::Rect(RectElement {
            x: box_x + 1,
            y: box_y + 1,
            width: box_w.saturating_sub(2),
            height: char_h,
            color: Color::from_palette(1),
            filled: true,
        }));
        view.add(Element::Text(TextElement {
            x: box_x + 8,
            y: box_y + 1,
            text: "SELECT DISPLAY RESOLUTION & COLOR THEME".to_string(),
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));
        let close_x = (box_x + box_w).saturating_sub(40);
        view.add(Element::Link(LinkElement::new(
            close_x,
            box_y + 1,
            "[X]",
            "#close-modal",
            TextStyle::new(Color::from_palette(10)).bold(),
        )));

        let row_start_y = box_y + char_h + 4;
        if cols >= 80 {
            let col1_x = box_x + 12;
            let col2_x = box_x + 300;

            // Section Headers
            view.add(Element::Text(TextElement {
                x: col1_x,
                y: row_start_y,
                text: "── RESOLUTION [1-8] ──".to_string(),
                style: TextStyle::new(Color::from_palette(4)).bold(),
            }));
            view.add(Element::Text(TextElement {
                x: col2_x,
                y: row_start_y,
                text: "── COLOR THEME [A-H] ──".to_string(),
                style: TextStyle::new(Color::from_palette(4)).bold(),
            }));

            // Resolution List (Left Column)
            for (i, res) in pixel_ssh_view::ResolutionMode::ALL.iter().enumerate() {
                let y = row_start_y + ((i + 1) as u16) * char_h;
                if y + char_h > box_y + box_h - char_h { break; }

                let is_cur = self.resolution == *res;
                let (w, h) = res.resolution();

                if is_cur {
                    view.add(Element::Rect(RectElement {
                        x: col1_x.saturating_sub(4),
                        y,
                        width: 270,
                        height: char_h,
                        color: Color::from_palette(2),
                        filled: true,
                    }));
                }

                let marker = if is_cur { "►" } else { " " };
                let line_text = format!(
                    "{} [{}] {:<4} {:>3}x{:<3} {}",
                    marker, i + 1, res.short_name(), w, h,
                    if is_cur { "[*]" } else { "" }
                );

                let fg = if is_cur { Color::from_palette(6) } else { Color::from_palette(5) };
                view.add(Element::Text(TextElement {
                    x: col1_x,
                    y,
                    text: line_text,
                    style: TextStyle::new(fg).bold(),
                }));
            }

            // Color Theme List (Right Column)
            for (i, theme) in pixel_ssh_view::ColorTheme::ALL.iter().enumerate() {
                let y = row_start_y + ((i + 1) as u16) * char_h;
                if y + char_h > box_y + box_h - char_h { break; }

                let is_cur = self.color_theme == *theme;
                let key_char = match i {
                    0 => 'A', 1 => 'B', 2 => 'C', 3 => 'D',
                    4 => 'E', 5 => 'F', 6 => 'G', _ => 'H',
                };

                if is_cur {
                    view.add(Element::Rect(RectElement {
                        x: col2_x.saturating_sub(4),
                        y,
                        width: 260,
                        height: char_h,
                        color: Color::from_palette(2),
                        filled: true,
                    }));
                }

                let marker = if is_cur { "►" } else { " " };
                let line_text = format!(
                    "{} [{}] {:<16} {}",
                    marker, key_char, theme.short_name(),
                    if is_cur { "[*]" } else { "" }
                );

                let fg = if is_cur { Color::from_palette(6) } else { Color::from_palette(5) };
                view.add(Element::Text(TextElement {
                    x: col2_x,
                    y,
                    text: line_text,
                    style: TextStyle::new(fg).bold(),
                }));
            }
        } else {
            // Compact 40-col stacked list
            for (i, res) in pixel_ssh_view::ResolutionMode::ALL.iter().take(4).enumerate() {
                let y = row_start_y + (i as u16) * char_h;
                let is_cur = self.resolution == *res;
                let (w, h) = res.resolution();
                let marker = if is_cur { "►" } else { " " };
                let line_text = format!("{} [{}] {:<4} {:>3}x{:<3}", marker, i + 1, res.short_name(), w, h);
                view.add(Element::Text(TextElement {
                    x: box_x + 8,
                    y,
                    text: line_text,
                    style: TextStyle::new(if is_cur { Color::from_palette(6) } else { Color::from_palette(5) }).bold(),
                }));
            }
            for (i, theme) in pixel_ssh_view::ColorTheme::ALL.iter().take(4).enumerate() {
                let y = row_start_y + ((i + 5) as u16) * char_h;
                let is_cur = self.color_theme == *theme;
                let key_char = match i { 0 => 'A', 1 => 'B', 2 => 'C', _ => 'D' };
                let marker = if is_cur { "►" } else { " " };
                let line_text = format!("{} [{}] {:<14}", marker, key_char, theme.short_name());
                view.add(Element::Text(TextElement {
                    x: box_x + 8,
                    y,
                    text: line_text,
                    style: TextStyle::new(if is_cur { Color::from_palette(6) } else { Color::from_palette(5) }).bold(),
                }));
            }
        }

        let foot_y = (box_y + box_h).saturating_sub(char_h + 2);
        view.add(Element::Text(TextElement {
            x: box_x + 8,
            y: foot_y,
            text: "Click or press [1-8] Resolution • [A-H] Theme • [ESC] Close".to_string(),
            style: TextStyle::new(Color::from_palette(4)),
        }));
    }

    /// Renders the Visuals effects modal dialog.
    pub(crate) fn render_visuals_modal(&self, view: &mut View, width: u16, height: u16, cols: u16) {
        let char_h: u16 = if height <= 200 { 8 } else { 16 };
        let box_w = if cols >= 80 { 540u16.min(width.saturating_sub(32)) } else { width.saturating_sub(16) };
        let box_h = (13 * char_h + 16).min(height.saturating_sub(24));
        let box_x = (width.saturating_sub(box_w)) / 2;
        let box_y = (height.saturating_sub(box_h)) / 2;

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
            height: box_h,
            color: Color::from_palette(7),
            filled: false,
        }));

        view.add(Element::Rect(RectElement {
            x: box_x + 1,
            y: box_y + 1,
            width: box_w.saturating_sub(2),
            height: char_h,
            color: Color::from_palette(1),
            filled: true,
        }));
        view.add(Element::Text(TextElement {
            x: box_x + 8,
            y: box_y + 1,
            text: "CRT DISPLAY & VISUAL EFFECTS".to_string(),
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));
        let close_x = (box_x + box_w).saturating_sub(40);
        view.add(Element::Link(LinkElement::new(
            close_x,
            box_y + 1,
            "[X]",
            "#close-modal",
            TextStyle::new(Color::from_palette(10)).bold(),
        )));

        let preset_y = box_y + char_h + 4;
        view.add(Element::Text(TextElement {
            x: box_x + 8,
            y: preset_y,
            text: "PRESETS:".to_string(),
            style: TextStyle::new(Color::from_palette(4)),
        }));
        let presets = [
            ("Clean", "#clean", 80),
            ("Trinitron", "#trinitron", 140),
            ("Arcade", "#arcade", 230),
            ("Bloom", "#bloom", 305),
            ("Glitch", "#glitch", 365),
        ];
        for (name, link, offset) in presets {
            if box_x + offset + 50 < box_x + box_w {
                view.add(Element::Link(LinkElement::new(
                    box_x + offset,
                    preset_y,
                    format!("[{}]", name),
                    link,
                    TextStyle::new(Color::from_palette(12)),
                )));
            }
        }

        let sliders = [
            ("Scanlines", self.visual_effects.scanlines),
            ("Pixel Grid", self.visual_effects.pixel_grid),
            ("Chromatic", self.visual_effects.chromatic),
            ("Afterglow", self.visual_effects.afterglow),
            ("Curvature", self.visual_effects.curvature),
            ("Jitter", self.visual_effects.jitter),
            ("Hum / RF", self.visual_effects.antenna_hum),
            ("White Noise", self.visual_effects.noise),
        ];

        let slider_start_y = preset_y + char_h + 4;
        for (i, (name, val)) in sliders.iter().enumerate() {
            let y = slider_start_y + (i as u16) * char_h;
            if y + char_h > box_y + box_h - char_h { break; }

            let pct = (*val * 100.0).round() as u8;
            let num_blocks = (pct as usize * 12) / 100;
            let mut bar = String::with_capacity(16);
            for b in 0..12 {
                if b < num_blocks {
                    bar.push('█');
                } else {
                    bar.push('░');
                }
            }

            let text = format!("{:<11} [ {} ] {:>3}%", name, bar, pct);
            view.add(Element::Text(TextElement {
                x: box_x + 8,
                y,
                text,
                style: TextStyle::new(Color::from_palette(5)),
            }));
        }

        let foot_y = (box_y + box_h).saturating_sub(char_h + 2);
        view.add(Element::Text(TextElement {
            x: box_x + 8,
            y: foot_y,
            text: "Click presets or sliders to adjust • [ESC] Close".to_string(),
            style: TextStyle::new(Color::from_palette(4)),
        }));
    }

    /// Renders the Help & keyboard navigation modal dialog.
    pub(crate) fn render_help_modal(&self, view: &mut View, width: u16, height: u16, cols: u16) {
        let char_h: u16 = if height <= 200 { 8 } else { 16 };
        let box_w = if cols >= 80 { 540u16.min(width.saturating_sub(32)) } else { width.saturating_sub(16) };
        let box_h = (13 * char_h + 16).min(height.saturating_sub(24));
        let box_x = (width.saturating_sub(box_w)) / 2;
        let box_y = (height.saturating_sub(box_h)) / 2;

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
            height: box_h,
            color: Color::from_palette(8),
            filled: false,
        }));

        view.add(Element::Rect(RectElement {
            x: box_x + 1,
            y: box_y + 1,
            width: box_w.saturating_sub(2),
            height: char_h,
            color: Color::from_palette(1),
            filled: true,
        }));
        view.add(Element::Text(TextElement {
            x: box_x + 8,
            y: box_y + 1,
            text: "SYSTEM HELP & KEYBOARD SHORTCUTS".to_string(),
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));
        let close_x = (box_x + box_w).saturating_sub(40);
        view.add(Element::Link(LinkElement::new(
            close_x,
            box_y + 1,
            "[X]",
            "#close-modal",
            TextStyle::new(Color::from_palette(10)).bold(),
        )));

        let shortcuts = [
            ("1 - 4", "Switch workstation views: Projects, CV, About, Contact"),
            ("Enter", "Open project details / Return to catalog"),
            ("h / l", "Navigate previous / next project in detail view"),
            ("j / k", "Scroll catalog, article, or project details up/down"),
            ("Wheel", "Smooth vertical mouse-wheel scrolling"),
            ("v / V", "Toggle CRT Visual Effects modal dialog"),
            ("s / S", "Toggle System Resolution modal dialog (SVGA, VGA, EGA...)"),
            ("? / h", "Toggle this Help & Navigation shortcuts dialog"),
            ("Top-R", "Move mouse to Top-Right corner for CRT screensaver"),
            ("ESC  ", "Close active modal dialog / Return to Projects view"),
        ];

        let row_start_y = box_y + char_h + 4;
        for (i, (keys, desc)) in shortcuts.iter().enumerate() {
            let y = row_start_y + (i as u16) * char_h;
            if y + char_h > box_y + box_h - char_h { break; }

            let text = format!("{:<7} : {}", keys, desc);
            view.add(Element::Text(TextElement {
                x: box_x + 8,
                y,
                text,
                style: TextStyle::new(Color::from_palette(5)),
            }));
        }

        let foot_y = (box_y + box_h).saturating_sub(char_h + 2);
        view.add(Element::Text(TextElement {
            x: box_x + 8,
            y: foot_y,
            text: "Press [ESC] or click [X] to return".to_string(),
            style: TextStyle::new(Color::from_palette(4)),
        }));
    }
}
