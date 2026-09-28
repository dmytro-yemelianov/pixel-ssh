//! Modal dialogs (System, Visuals, Help), article overlays (CV, About), and CRT screensaver.

use crate::data::{
    ABOUT_LINES_32, ABOUT_LINES_40, ABOUT_LINES_80, RESUME_LINES_32, RESUME_LINES_40,
    RESUME_LINES_80,
};
use crate::state::{App, Tab};
use pixel_ssh_view::{Color, Element, LinkElement, RectElement, TextElement, TextStyle, View};

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
    pub(crate) fn system_dialog_geometry(&self) -> SystemDialogGeometry {
        // Header + label + eight choices + footer fit in thirteen rows and
        // leave the compact navigation bar uncovered.
        self.dialog_geometry(13)
    }

    pub(crate) fn standard_dialog_geometry(&self) -> SystemDialogGeometry {
        self.dialog_geometry(15)
    }

    fn dialog_geometry(&self, compact_rows: u16) -> SystemDialogGeometry {
        let (width, height) = self.resolution.resolution();
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
        let article_title = if cols >= 80 {
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

        let (lines, scroll_offset, max_scroll): (&[(&str, u8, bool)], usize, usize) = if is_resume {
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
                    x: box_x + if cols <= 32 { 8 } else { 16 },
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

    /// Renders the System mode / Resolution & Color Theme picker modal dialog.
    pub(crate) fn render_system_modal(
        &self,
        view: &mut View,
        _width: u16,
        _height: u16,
        cols: u16,
    ) {
        let entries: Vec<String> = pixel_ssh_view::ResolutionMode::ALL
            .iter()
            .map(|resolution| {
                let (width, height) = resolution.resolution();
                format!("{:<4} {:>3}x{:<3}", resolution.short_name(), width, height)
            })
            .collect();
        self.render_selector_modal(
            view,
            cols,
            "SYSTEM: RESOLUTION",
            "Resolution",
            &entries,
            pixel_ssh_view::ResolutionMode::ALL
                .iter()
                .position(|resolution| *resolution == self.resolution)
                .unwrap_or(0),
            "#resolution-",
        );
    }

    /// Renders the RGB palette selector. Palette changes do not alter glyphs
    /// or screen geometry.
    pub(crate) fn render_color_modal(&self, view: &mut View, _width: u16, _height: u16, cols: u16) {
        let entries: Vec<String> = pixel_ssh_view::ColorPalette::ALL
            .iter()
            .map(|palette| palette.name().to_string())
            .collect();
        self.render_selector_modal(
            view,
            cols,
            "COLOR: RGB PALETTE",
            "Color palette",
            &entries,
            pixel_ssh_view::ColorPalette::ALL
                .iter()
                .position(|palette| *palette == self.color_palette)
                .unwrap_or(0),
            "#palette-",
        );
    }

    /// Renders the interface theme selector. Theme changes only glyph and
    /// cursor treatment; palette and resolution remain intact.
    pub(crate) fn render_theme_modal(&self, view: &mut View, _width: u16, _height: u16, cols: u16) {
        let entries: Vec<String> = pixel_ssh_view::InterfaceTheme::ALL
            .iter()
            .map(|theme| theme.name().to_string())
            .collect();
        self.render_selector_modal(
            view,
            cols,
            "THEME: INTERFACE GLYPHS",
            "Interface theme",
            &entries,
            pixel_ssh_view::InterfaceTheme::ALL
                .iter()
                .position(|theme| *theme == self.interface_theme)
                .unwrap_or(0),
            "#theme-",
        );
    }

    #[allow(clippy::too_many_arguments)] // Rendering data is explicit at each selector call site.
    fn render_selector_modal(
        &self,
        view: &mut View,
        cols: u16,
        title: &str,
        label: &str,
        entries: &[String],
        selected: usize,
        link_prefix: &str,
    ) {
        let SystemDialogGeometry {
            box_x,
            box_y,
            box_w,
            box_h,
            box_cols,
            box_rows,
            char_h,
        } = self.system_dialog_geometry();
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
        let title = if cols >= 80 {
            title
        } else if title.starts_with("SYSTEM") {
            "SYSTEM"
        } else if title.starts_with("COLOR") {
            "COLOR"
        } else {
            "THEME"
        };
        let top_start = format!("╔═ [ {title} ] ");
        let top = format!(
            "{}{}══════╗",
            top_start,
            "═".repeat((box_cols as usize).saturating_sub(top_start.chars().count() + 7))
        );
        view.add(Element::Text(TextElement {
            x: box_x,
            y: box_y,
            text: top,
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));
        view.add(Element::Link(LinkElement::new(
            box_x + box_w.saturating_sub(40),
            box_y,
            "[X]",
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
        let list_x = box_x + if cols >= 80 { 64 } else { 16 };
        view.add(Element::Text(TextElement {
            x: list_x,
            y: box_y + char_h,
            text: format!("{label} [1-8]"),
            style: TextStyle::new(Color::from_palette(4)).bold(),
        }));
        let available_chars = (box_cols.saturating_sub((list_x - box_x) / 8 + 2)) as usize;
        let entry_chars = available_chars.saturating_sub(10);
        for (index, entry) in entries.iter().enumerate() {
            let y = box_y + (index as u16 + 2) * char_h;
            if y + char_h >= box_y + box_h {
                break;
            }
            let active = index == selected;
            if active {
                view.add(Element::Rect(RectElement {
                    x: list_x.saturating_sub(8),
                    y,
                    width: box_w.saturating_sub((list_x - box_x) + 24),
                    height: char_h,
                    color: Color::from_palette(2),
                    filled: true,
                }));
            }
            let marker = if active { "►" } else { " " };
            let entry = entry.chars().take(entry_chars).collect::<String>();
            let text = format!(
                "{marker} [{}] {:<entry_chars$} {}",
                index + 1,
                entry,
                if active { "[*]" } else { "" }
            );
            let mut option = LinkElement::new(
                list_x,
                y,
                text,
                format!("{link_prefix}{index}"),
                if active {
                    TextStyle::new(Color::from_palette(6)).bold()
                } else {
                    TextStyle::new(Color::from_palette(5))
                },
            );
            // These rows are buttons, not text hyperlinks.  Underlining them
            // draws a scanline across the next compact character row.
            option.style.underline = false;
            view.add(Element::Link(option));
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
        let prompt = "╚══ [1-8] Select • [ESC] Close ";
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
                    TextStyle::new(Color::from_palette(12)).bold(),
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

            let text = format!("{name:<12} [ {bar} ] {pct:>3}%");
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
        let preset_text = if cols == 40 {
            format!("PRESET:{preset:<7} A CRT C D E")
        } else {
            format!("PRE:{preset:<6} A C D E")
        };
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
            ("P H S C T Q", "Projects, Help, System, Color, Theme, Quit"),
            ("Enter", "Open project details / Return to catalog"),
            ("h / l", "Navigate previous / next project in detail view"),
            (
                "j / k",
                "Scroll catalog, article, or project details up/down",
            ),
            ("Wheel", "Smooth vertical mouse-wheel scrolling"),
            ("v / V", "Toggle CRT Visual Effects modal dialog"),
            ("s / S", "Open the resolution selector (SVGA, VGA, EGA...)"),
            ("c / C", "Open the independent RGB color palette selector"),
            ("t / T", "Open the independent interface theme selector"),
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
                "S  Resolution",
                "C  RGB palette",
                "T  UI theme",
                "Q  Return/Quit",
                "V  Visuals",
                "UP/DN j/k Scroll",
                "LT/RT h/l Detail",
                "ENT Open/return",
                "ESC Close dialog",
            ]
        } else {
            [
                "P Projects",
                "H/? Help",
                "S Resolution",
                "C Palette",
                "T Theme",
                "Q Return",
                "V Visuals",
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
