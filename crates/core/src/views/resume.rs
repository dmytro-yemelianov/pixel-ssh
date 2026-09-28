//! Resume tab view implementation.

use crate::data::{RESUME_LINES_32, RESUME_LINES_40, RESUME_LINES_80};
use crate::state::App;
use pixel_ssh_view::{
    horizontal_scroll, Color, Element, LinkElement, Platform, RectElement, TextElement, TextStyle,
    View,
};

impl App {
    pub(crate) fn render_resume(&self, view: &mut View) {
        if self.platform == Platform::Terminal {
            self.render_resume_terminal(view);
            return;
        }
        let (cols, _) = self.resolution.char_grid();
        match cols {
            100 | 80 => self.render_resume_80(view),
            40 => self.render_resume_40(view),
            _ => self.render_resume_32(view),
        }
    }

    pub(crate) fn render_resume_80(&self, view: &mut View) {
        view.add(Element::Text(TextElement {
            x: 16,
            y: 48,
            text: "DMYTRO YEMELIANOV - SENIOR SYSTEMS & AI INFRASTRUCTURE ENGINEER".to_string(),
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));

        // Clickable header links (Row 4: y = 64)
        view.add(Element::Link(LinkElement::new(
            16,
            64,
            "[ Web: yemelianov.dev ]",
            "https://yemelianov.dev",
            TextStyle::new(Color::from_palette(12)),
        )));
        view.add(Element::Link(LinkElement::new(
            216,
            64,
            "[ GitHub: dmytro-yemelianov ]",
            "https://github.com/dmytro-yemelianov",
            TextStyle::new(Color::from_palette(7)),
        )));
        view.add(Element::Link(LinkElement::new(
            480,
            64,
            "[ LinkedIn ]",
            "https://linkedin.com/in/dmytro-yemelianov",
            TextStyle::new(Color::from_palette(7)),
        )));

        // Top divider (Row 5: y = 80)
        view.add(Element::Rect(RectElement {
            x: 0,
            y: 80,
            width: 640,
            height: 1,
            color: Color::from_palette(7),
            filled: true,
        }));

        let max_scroll = self.resume_max_scroll();

        // Vertical scroll window from y = 96 to y = 320 (Rows 6..20, 15 visible lines)
        for (i, &(line_text, pal_idx, bold)) in RESUME_LINES_80.iter().enumerate() {
            let line_y = 96 + ((i as i32) - (self.resume_scroll as i32)) * 16;
            if (96..=320).contains(&line_y) {
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

        // Retro scrollbar on the right edge (x = 624)
        view.add(Element::Text(TextElement {
            x: 624,
            y: 96,
            text: "▲".to_string(),
            style: TextStyle::new(Color::from_palette(7)).bold(),
        }));

        // Track bar
        view.add(Element::Rect(RectElement {
            x: 627,
            y: 112,
            width: 2,
            height: 192,
            color: Color::from_palette(7),
            filled: true,
        }));

        // Scroll thumb
        let track_h = 192.0 - 16.0;
        let thumb_ratio = if max_scroll > 0 {
            self.resume_scroll as f32 / max_scroll as f32
        } else {
            0.0
        };
        let thumb_y = 112 + (thumb_ratio * track_h) as u16;
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

        // Bottom scroll status (Row 21: y = 336)
        view.add(Element::Text(TextElement {
            x: 16,
            y: 336,
            text: format!(
                "Lines {}-{} of {} | Use [Up/Down], [k/j], [Wheel], or click track to scroll",
                self.resume_scroll + 1,
                (self.resume_scroll + 15).min(RESUME_LINES_80.len()),
                RESUME_LINES_80.len()
            ),
            style: TextStyle::new(Color::from_palette(4)),
        }));
    }

    pub(crate) fn render_resume_40(&self, view: &mut View) {
        let resume_hdr =
            horizontal_scroll("DMYTRO YEMELIANOV - SYSTEMS & AI ARCHITECT", 38, self.tick);
        view.add(Element::Text(TextElement {
            x: 4,
            y: 22,
            text: resume_hdr,
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));

        view.add(Element::Link(LinkElement::new(
            4,
            31,
            "[yemelianov.dev]",
            "https://yemelianov.dev",
            TextStyle::new(Color::from_palette(12)),
        )));
        view.add(Element::Link(LinkElement::new(
            138,
            31,
            "[github]",
            "https://github.com/dmytro-yemelianov",
            TextStyle::new(Color::from_palette(7)),
        )));
        view.add(Element::Link(LinkElement::new(
            212,
            31,
            "[linkedin]",
            "https://linkedin.com/in/dmytro-yemelianov",
            TextStyle::new(Color::from_palette(7)),
        )));

        view.add(Element::Rect(RectElement {
            x: 4,
            y: 40,
            width: 302,
            height: 1,
            color: Color::from_palette(3),
            filled: true,
        }));

        let max_scroll = self.resume_max_scroll();

        for (i, &(line_text, pal_idx, bold)) in RESUME_LINES_40.iter().enumerate() {
            let line_y = 43 + ((i as i32) - (self.resume_scroll as i32)) * 9;
            if (43..=170).contains(&line_y) {
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

        // Scrollbar at x = 310
        view.add(Element::Text(TextElement {
            x: 310,
            y: 43,
            text: "▲".to_string(),
            style: TextStyle::new(Color::from_palette(7)).bold(),
        }));

        view.add(Element::Rect(RectElement {
            x: 313,
            y: 53,
            width: 2,
            height: 110,
            color: Color::from_palette(3),
            filled: true,
        }));

        let track_h = 110.0 - 12.0;
        let thumb_ratio = if max_scroll > 0 {
            self.resume_scroll as f32 / max_scroll as f32
        } else {
            0.0
        };
        let thumb_y = 53 + (thumb_ratio * track_h) as u16;
        view.add(Element::Rect(RectElement {
            x: 310,
            y: thumb_y,
            width: 8,
            height: 12,
            color: Color::from_palette(6),
            filled: true,
        }));

        view.add(Element::Text(TextElement {
            x: 310,
            y: 165,
            text: "▼".to_string(),
            style: TextStyle::new(Color::from_palette(7)).bold(),
        }));

        view.add(Element::Text(TextElement {
            x: 4,
            y: 177,
            text: format!(
                "Lines {}-{} of {} [j/k/Wheel]",
                self.resume_scroll + 1,
                (self.resume_scroll + 14).min(RESUME_LINES_40.len()),
                RESUME_LINES_40.len()
            ),
            style: TextStyle::new(Color::from_palette(4)),
        }));
    }

    pub(crate) fn render_resume_32(&self, view: &mut View) {
        let resume_hdr =
            horizontal_scroll("DMYTRO YEMELIANOV - SYSTEMS & AI ARCHITECT", 30, self.tick);
        view.add(Element::Text(TextElement {
            x: 2,
            y: 22,
            text: resume_hdr,
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));

        view.add(Element::Link(LinkElement::new(
            2,
            31,
            "[yemelianov.dev]",
            "https://yemelianov.dev",
            TextStyle::new(Color::from_palette(12)),
        )));
        view.add(Element::Link(LinkElement::new(
            134,
            31,
            "[github]",
            "https://github.com/dmytro-yemelianov",
            TextStyle::new(Color::from_palette(7)),
        )));

        view.add(Element::Rect(RectElement {
            x: 2,
            y: 40,
            width: 240,
            height: 1,
            color: Color::from_palette(3),
            filled: true,
        }));

        let max_scroll = self.resume_max_scroll();

        for (i, &(line_text, pal_idx, bold)) in RESUME_LINES_32.iter().enumerate() {
            let line_y = 43 + ((i as i32) - (self.resume_scroll as i32)) * 9;
            if (43..=164).contains(&line_y) {
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
            self.resume_scroll as f32 / max_scroll as f32
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
            y: 170,
            text: format!(
                "Lines {}-{} of {} [k/j]",
                self.resume_scroll + 1,
                (self.resume_scroll + 13).min(RESUME_LINES_32.len()),
                RESUME_LINES_32.len()
            ),
            style: TextStyle::new(Color::from_palette(4)),
        }));
    }

    pub(crate) fn render_resume_terminal(&self, view: &mut View) {
        let cols = self.terminal_cols;
        let rows = self.terminal_rows;
        let width = cols * 8;
        let max_scroll = self.resume_max_scroll();

        // Visible content rows: row 3 to rows - 3
        let visible_rows = (rows as usize).saturating_sub(5).max(6);
        for row_idx in 0..visible_rows {
            let line_idx = self.resume_scroll + row_idx;
            if line_idx < RESUME_LINES_80.len() {
                let (line_text, pal_idx, bold) = RESUME_LINES_80[line_idx];
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
            self.resume_scroll as f32 / max_scroll as f32
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
                self.resume_scroll + 1,
                (self.resume_scroll + visible_rows).min(RESUME_LINES_80.len()),
                RESUME_LINES_80.len()
            ),
            style: TextStyle::new(Color::from_palette(4)),
        }));
    }
}
