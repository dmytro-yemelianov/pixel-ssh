//! About system tab view implementation.

use crate::data::{ABOUT_LINES_32, ABOUT_LINES_40, ABOUT_LINES_80};
use crate::state::App;
use pixel_ssh_view::{Color, Element, Platform, RectElement, TextElement, TextStyle, View};

impl App {
    pub(crate) fn render_about(&self, view: &mut View) {
        if self.platform == Platform::Terminal {
            self.render_about_terminal(view);
            return;
        }
        let (cols, _) = self.system_mode.char_grid();
        match cols {
            100 | 80 => self.render_about_80(view),
            40 => self.render_about_40(view),
            _ => self.render_about_32(view),
        }
    }

    pub(crate) fn render_about_80(&self, view: &mut View) {
        let max_scroll = self.about_max_scroll();

        for (i, &(line_text, pal_idx, bold)) in ABOUT_LINES_80.iter().enumerate() {
            let line_y = 48 + ((i as i32) - (self.about_scroll as i32)) * 16;
            if (48..=320).contains(&line_y) {
                let mut style = TextStyle::new(Color::from_palette(pal_idx));
                if bold {
                    style = style.bold();
                }
                view.add(Element::Text(TextElement {
                    x: 16,
                    y: line_y as u16,
                    text: line_text.to_string(),
                    style,
                }));
            }
        }

        // On Web, render high-resolution dithered portrait sprite inside the framed architect card
        if self.about_scroll == 0 && self.platform == Platform::Web {
            view.add(Element::Sprite(pixel_ssh_view::SpriteElement {
                x: 24,
                y: 64,
                width: 96,
                height: 96,
                data: crate::data::profile::get_profile_sprite(self.palette_mode, 96).to_vec(),
            }));
        }

        // Scrollbar at x = 624
        view.add(Element::Text(TextElement {
            x: 624,
            y: 48,
            text: "▲".to_string(),
            style: TextStyle::new(Color::from_palette(7)).bold(),
        }));

        view.add(Element::Rect(RectElement {
            x: 627,
            y: 64,
            width: 2,
            height: 240,
            color: Color::from_palette(7),
            filled: true,
        }));

        let track_h = 240.0 - 16.0;
        let thumb_ratio = if max_scroll > 0 {
            self.about_scroll as f32 / max_scroll as f32
        } else {
            0.0
        };
        let thumb_y = 64 + (thumb_ratio * track_h) as u16;
        view.add(Element::Rect(RectElement {
            x: 624,
            y: thumb_y,
            width: 8,
            height: 16,
            color: Color::from_palette(6),
            filled: true,
        }));

        view.add(Element::Text(TextElement {
            x: 624,
            y: 304,
            text: "▼".to_string(),
            style: TextStyle::new(Color::from_palette(7)).bold(),
        }));

        view.add(Element::Text(TextElement {
            x: 16,
            y: 336,
            text: format!(
                "Lines {}-{} of {}  [k/j or Wheel to scroll]",
                self.about_scroll + 1,
                (self.about_scroll + 18).min(ABOUT_LINES_80.len()),
                ABOUT_LINES_80.len()
            ),
            style: TextStyle::new(Color::from_palette(4)),
        }));
    }

    pub(crate) fn render_about_40(&self, view: &mut View) {
        let max_scroll = self.about_max_scroll();

        for (i, &(line_text, pal_idx, bold)) in ABOUT_LINES_40.iter().enumerate() {
            let line_y = 24 + ((i as i32) - (self.about_scroll as i32)) * 8;
            if (24..=170).contains(&line_y) {
                let mut style = TextStyle::new(Color::from_palette(pal_idx));
                if bold {
                    style = style.bold();
                }
                view.add(Element::Text(TextElement {
                    x: 4,
                    y: line_y as u16,
                    text: line_text.to_string(),
                    style,
                }));
            }
        }

        if self.about_scroll == 0 && self.platform == Platform::Web {
            view.add(Element::Sprite(pixel_ssh_view::SpriteElement {
                x: 12,
                y: 32,
                width: 64,
                height: 64,
                data: crate::data::profile::get_profile_sprite(self.palette_mode, 64).to_vec(),
            }));
        }

        // Scrollbar at x = 310
        view.add(Element::Text(TextElement {
            x: 310,
            y: 38,
            text: "▲".to_string(),
            style: TextStyle::new(Color::from_palette(7)).bold(),
        }));

        view.add(Element::Rect(RectElement {
            x: 313,
            y: 48,
            width: 2,
            height: 110,
            color: Color::from_palette(3),
            filled: true,
        }));

        let track_h = 110.0 - 16.0;
        let thumb_ratio = if max_scroll > 0 {
            self.about_scroll as f32 / max_scroll as f32
        } else {
            0.0
        };
        let thumb_y = 48 + (thumb_ratio * track_h) as u16;
        view.add(Element::Rect(RectElement {
            x: 310,
            y: thumb_y,
            width: 8,
            height: 16,
            color: Color::from_palette(6),
            filled: true,
        }));

        view.add(Element::Text(TextElement {
            x: 310,
            y: 164,
            text: "▼".to_string(),
            style: TextStyle::new(Color::from_palette(7)).bold(),
        }));

        view.add(Element::Text(TextElement {
            x: 4,
            y: 180,
            text: format!(
                "Lines {}-{} of {} [k/j]",
                self.about_scroll + 1,
                (self.about_scroll + 19).min(ABOUT_LINES_40.len()),
                ABOUT_LINES_40.len()
            ),
            style: TextStyle::new(Color::from_palette(4)),
        }));
    }

    pub(crate) fn render_about_32(&self, view: &mut View) {
        let max_scroll = self.about_max_scroll();

        for (i, &(line_text, pal_idx, bold)) in ABOUT_LINES_32.iter().enumerate() {
            let line_y = 24 + ((i as i32) - (self.about_scroll as i32)) * 8;
            if (24..=160).contains(&line_y) {
                let mut style = TextStyle::new(Color::from_palette(pal_idx));
                if bold {
                    style = style.bold();
                }
                view.add(Element::Text(TextElement {
                    x: 2,
                    y: line_y as u16,
                    text: line_text.to_string(),
                    style,
                }));
            }
        }

        if self.about_scroll == 0 && self.platform == Platform::Web {
            view.add(Element::Sprite(pixel_ssh_view::SpriteElement {
                x: 10,
                y: 32,
                width: 64,
                height: 64,
                data: crate::data::profile::get_profile_sprite(self.palette_mode, 64).to_vec(),
            }));
        }

        // Scrollbar at x = 246
        view.add(Element::Text(TextElement {
            x: 246,
            y: 43,
            text: "▲".to_string(),
            style: TextStyle::new(Color::from_palette(7)).bold(),
        }));

        view.add(Element::Rect(RectElement {
            x: 249,
            y: 53,
            width: 2,
            height: 100,
            color: Color::from_palette(3),
            filled: true,
        }));

        let track_h = 100.0 - 12.0;
        let thumb_ratio = if max_scroll > 0 {
            self.about_scroll as f32 / max_scroll as f32
        } else {
            0.0
        };
        let thumb_y = 53 + (thumb_ratio * track_h) as u16;
        view.add(Element::Rect(RectElement {
            x: 246,
            y: thumb_y,
            width: 8,
            height: 12,
            color: Color::from_palette(6),
            filled: true,
        }));

        view.add(Element::Text(TextElement {
            x: 246,
            y: 155,
            text: "▼".to_string(),
            style: TextStyle::new(Color::from_palette(7)).bold(),
        }));

        view.add(Element::Text(TextElement {
            x: 2,
            y: 171,
            text: format!(
                "Lines {}-{} of {} [k/j]",
                self.about_scroll + 1,
                (self.about_scroll + 18).min(ABOUT_LINES_32.len()),
                ABOUT_LINES_32.len()
            ),
            style: TextStyle::new(Color::from_palette(4)),
        }));
    }

    pub(crate) fn render_about_terminal(&self, view: &mut View) {
        let width = self.terminal_cols * 8;
        let rows = self.terminal_rows;
        let max_scroll = self.about_max_scroll();
        let visible_rows = (rows as usize).saturating_sub(5).max(6);

        for (i, &(line_text, pal_idx, bold)) in ABOUT_LINES_80.iter().enumerate() {
            let row_idx = (i as i32) - (self.about_scroll as i32);
            if row_idx >= 0 && (row_idx as usize) < visible_rows {
                let y = (3 + row_idx as u16) * 16;
                let mut style = TextStyle::new(Color::from_palette(pal_idx));
                if bold {
                    style = style.bold();
                }
                view.add(Element::Text(TextElement {
                    x: 8,
                    y,
                    text: line_text.to_string(),
                    style,
                }));
            }
        }

        // Scrollbar at right edge
        let sb_x = width.saturating_sub(16);
        view.add(Element::Text(TextElement {
            x: sb_x,
            y: 48,
            text: "▲".to_string(),
            style: TextStyle::new(Color::from_palette(7)).bold(),
        }));

        let track_top = 64;
        let track_bottom = (rows.saturating_sub(3)) * 16;
        let track_h = track_bottom.saturating_sub(track_top);
        view.add(Element::Rect(RectElement {
            x: sb_x + 3,
            y: track_top,
            width: 2,
            height: track_h,
            color: Color::from_palette(7),
            filled: true,
        }));

        let thumb_ratio = if max_scroll > 0 {
            self.about_scroll as f32 / max_scroll as f32
        } else {
            0.0
        };
        let thumb_travel = track_h.saturating_sub(16) as f32;
        let thumb_y = track_top + (thumb_ratio * thumb_travel) as u16;
        view.add(Element::Rect(RectElement {
            x: sb_x,
            y: thumb_y,
            width: 8,
            height: 16,
            color: Color::from_palette(6),
            filled: true,
        }));

        view.add(Element::Text(TextElement {
            x: sb_x,
            y: track_bottom,
            text: "▼".to_string(),
            style: TextStyle::new(Color::from_palette(7)).bold(),
        }));

        // Scroll guidance status at row rows - 2
        let hint_y = (rows.saturating_sub(2)) * 16;
        view.add(Element::Text(TextElement {
            x: 8,
            y: hint_y,
            text: format!(
                "Lines {}-{} of {} | Use [Up/Down], [k/j], [PgUp/PgDn] to scroll",
                self.about_scroll + 1,
                (self.about_scroll + visible_rows).min(ABOUT_LINES_80.len()),
                ABOUT_LINES_80.len()
            ),
            style: TextStyle::new(Color::from_palette(4)),
        }));
    }
}
