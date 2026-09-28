//! Contact tab view implementation.

use crate::state::App;
use pixel_ssh_view::{
    word_wrap, Color, Element, LinkElement, Platform, RectElement, TextElement, TextStyle, View,
};

impl App {
    pub(crate) fn render_contact(&self, view: &mut View) {
        if self.platform == Platform::Terminal {
            self.render_contact_terminal(view);
            return;
        }
        let (cols, _) = self.resolution.char_grid();
        match cols {
            100 | 80 => self.render_contact_80(view),
            40 => self.render_contact_40(view),
            _ => self.render_contact_32(view),
        }
    }

    pub(crate) fn render_contact_80(&self, view: &mut View) {
        let cols = (view.width / 8) as usize;
        let lh = self.resolution.line_height();
        let title_panel = " D:\\PORTFOLIO\\CONTACT ";
        let left_pad = cols.saturating_sub(2 + title_panel.len()) / 2;
        let right_pad = cols.saturating_sub(2 + title_panel.len() + left_pad);
        let top_border = format!(
            "╔{}{}{}╗",
            "═".repeat(left_pad),
            title_panel,
            "═".repeat(right_pad)
        );
        view.add(Element::Text(TextElement {
            x: 0,
            y: 2 * lh,
            text: top_border,
            style: TextStyle::new(Color::from_palette(3)).bold(),
        }));

        view.add(Element::Text(TextElement {
            x: 0,
            y: 3 * lh,
            text: format!("║  {:<76}║", "CONNECT & DIRECT CONTACT:"),
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));

        view.add(Element::Text(TextElement {
            x: 0,
            y: 4 * lh,
            text: format!(
                "║  {:<76}║",
                "Click any underlined link below to open directly in a new tab:"
            ),
            style: TextStyle::new(Color::from_palette(4)),
        }));

        // Divider (Row 5)
        let sep_str = format!("╟{}╢", "─".repeat(cols.saturating_sub(2)));
        view.add(Element::Text(TextElement {
            x: 0,
            y: 5 * lh,
            text: sep_str.clone(),
            style: TextStyle::new(Color::from_palette(3)),
        }));

        // Clickable contact cards (Rows 6, 8, 10, 12, 14)
        let links = [
            (
                " Email:    dyemelianov@icloud.com              [Open Mail] ",
                "mailto:dyemelianov@icloud.com",
                6,
            ),
            (
                " GitHub:   https://github.com/dmytro-yemelianov [Profile]   ",
                "https://github.com/dmytro-yemelianov",
                8,
            ),
            (
                " LinkedIn: https://linkedin.com/in/dmytro-yemelianov [Net]  ",
                "https://linkedin.com/in/dmytro-yemelianov",
                10,
            ),
            (
                " Website:  https://yemelianov.dev              [Portal]    ",
                "https://yemelianov.dev",
                12,
            ),
            (
                " SSH:      ssh guest@yemelianov.dev:2222       [Terminal]  ",
                "ssh://guest@yemelianov.dev:2222",
                14,
            ),
        ];

        for (label, url, r) in links {
            let ly = r * lh;
            view.add(Element::Rect(RectElement {
                x: 16,
                y: ly,
                width: view.width.saturating_sub(32),
                height: lh,
                color: Color::from_palette(13),
                filled: true,
            }));
            view.add(Element::Link(LinkElement::new(
                24,
                ly,
                label,
                url,
                TextStyle::new(Color::from_palette(14)).bold(),
            )));
        }

        // Side borders for rows 6..14
        for r in 6..15 {
            let ry = r * lh;
            view.add(Element::Text(TextElement {
                x: 0,
                y: ry,
                text: "║".to_string(),
                style: TextStyle::new(Color::from_palette(3)),
            }));
            view.add(Element::Text(TextElement {
                x: view.width.saturating_sub(8),
                y: ry,
                text: "║".to_string(),
                style: TextStyle::new(Color::from_palette(3)),
            }));
        }

        // Secondary Divider (Row 15)
        view.add(Element::Text(TextElement {
            x: 0,
            y: 15 * lh,
            text: sep_str,
            style: TextStyle::new(Color::from_palette(3)),
        }));

        // Side borders and content for rows 16..19
        let info = [
            "Location:     Vinnytsia, Ukraine / Remote worldwide (no relocation)",
            "Availability: Full-time Lead / Senior Architect roles in Systems & AI",
            "Specialty:    Agentic architectures, compilers, CAD/PLM, high-throughput Rust",
            "Direct PGP:   Available on request • Key ID 0x3F8A7B2C",
        ];
        for (i, line) in info.iter().enumerate() {
            let ry = (16 + i as u16) * lh;
            view.add(Element::Text(TextElement {
                x: 0,
                y: ry,
                text: "║".to_string(),
                style: TextStyle::new(Color::from_palette(3)),
            }));
            view.add(Element::Text(TextElement {
                x: 16,
                y: ry,
                text: format!("{line:<76}"),
                style: TextStyle::new(Color::from_palette(5)),
            }));
            view.add(Element::Text(TextElement {
                x: view.width.saturating_sub(8),
                y: ry,
                text: "║".to_string(),
                style: TextStyle::new(Color::from_palette(3)),
            }));
        }

        // Panel Bottom Border (Row 20)
        let bot_label = " 5 Direct Channels Online • Available ";
        let left_bot = cols.saturating_sub(2 + bot_label.len()) / 2;
        let right_bot = cols.saturating_sub(2 + bot_label.len() + left_bot);
        let bot_border = format!(
            "╚{}{}{}╝",
            "═".repeat(left_bot),
            bot_label,
            "═".repeat(right_bot)
        );
        let bot_y = 20 * lh;
        if bot_y + lh <= view.height {
            view.add(Element::Text(TextElement {
                x: 0,
                y: bot_y,
                text: bot_border,
                style: TextStyle::new(Color::from_palette(3)).bold(),
            }));
        }

        // DOS prompt path (Row 21)
        let cursor_char = if (self.tick / 4) % 2 == 0 { "█" } else { " " };
        let prompt_y = 21 * lh;
        if prompt_y + lh <= view.height {
            view.add(Element::Text(TextElement {
                x: 16,
                y: prompt_y,
                text: format!("C:\\DMYTRO\\CONTACT>{cursor_char}"),
                style: TextStyle::new(Color::from_palette(14)).bold(),
            }));
        }

        // Navigation hint (Row 22)
        let hint_y = 22 * lh;
        if hint_y + lh <= view.height {
            view.add(Element::Text(TextElement {
                x: 16,
                y: hint_y,
                text: "Click any contact channel above to open • [1-10] Navigation buttons below"
                    .to_string(),
                style: TextStyle::new(Color::from_palette(4)),
            }));
        }
    }

    pub(crate) fn render_contact_40(&self, view: &mut View) {
        view.add(Element::Text(TextElement {
            x: 4,
            y: 24,
            text: "CONNECT & DIRECT CONTACT:".to_string(),
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));

        view.add(Element::Text(TextElement {
            x: 4,
            y: 34,
            text: "Click link to open in new tab:".to_string(),
            style: TextStyle::new(Color::from_palette(4)),
        }));

        view.add(Element::Rect(RectElement {
            x: 4,
            y: 43,
            width: 312,
            height: 1,
            color: Color::from_palette(3),
            filled: true,
        }));

        let links = [
            (
                "Email:   dyemelianov@icloud.com",
                "mailto:dyemelianov@icloud.com",
                47,
            ),
            (
                "GitHub:  github.com/dmytro-yemelianov",
                "https://github.com/dmytro-yemelianov",
                72,
            ),
            (
                "LinkedIn: linkedin.com/in/dmytro-yemelianov",
                "https://linkedin.com/in/dmytro-yemelianov",
                97,
            ),
            (
                "Website: https://yemelianov.dev",
                "https://yemelianov.dev",
                122,
            ),
            (
                "SSH:     ssh -p 2222 guest@dev",
                "ssh://guest@yemelianov.dev:2222",
                147,
            ),
        ];

        for (label, url, ly) in links {
            view.add(Element::Rect(RectElement {
                x: 4,
                y: ly,
                width: 312,
                height: 18,
                color: Color::from_palette(1),
                filled: true,
            }));
            view.add(Element::Link(LinkElement::new(
                8,
                ly + 4,
                label,
                url,
                TextStyle::new(Color::from_palette(12)).bold(),
            )));
        }

        view.add(Element::Text(TextElement {
            x: 4,
            y: 173,
            text: "Vinnytsia, UA / Remote worldwide".to_string(),
            style: TextStyle::new(Color::from_palette(5)),
        }));
    }

    pub(crate) fn render_contact_32(&self, view: &mut View) {
        view.add(Element::Text(TextElement {
            x: 2,
            y: 24,
            text: "CONNECT & CONTACT:".to_string(),
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));

        view.add(Element::Text(TextElement {
            x: 2,
            y: 33,
            text: "Click link to open:".to_string(),
            style: TextStyle::new(Color::from_palette(4)),
        }));

        view.add(Element::Rect(RectElement {
            x: 2,
            y: 42,
            width: 252,
            height: 1,
            color: Color::from_palette(3),
            filled: true,
        }));

        let links = [
            (
                "Email: dyemelianov@icloud.com",
                "mailto:dyemelianov@icloud.com",
                46,
            ),
            (
                "GH:    github/.../yemelianov",
                "https://github.com/dmytro-yemelianov",
                70,
            ),
            (
                "In:    linkedin/.../yemelianov",
                "https://linkedin.com/in/dmytro-yemelianov",
                94,
            ),
            ("Web:   yemelianov.dev", "https://yemelianov.dev", 118),
            (
                "SSH:   ssh -p 2222 guest@dev",
                "ssh://guest@yemelianov.dev:2222",
                142,
            ),
        ];

        for (label, url, ly) in links {
            view.add(Element::Rect(RectElement {
                x: 2,
                y: ly,
                width: 252,
                height: 18,
                color: Color::from_palette(1),
                filled: true,
            }));
            view.add(Element::Link(LinkElement::new(
                6,
                ly + 4,
                label,
                url,
                TextStyle::new(Color::from_palette(12)).bold(),
            )));
        }

        view.add(Element::Text(TextElement {
            x: 2,
            y: 168,
            text: "Vinnytsia, UA / Remote / Relo".to_string(),
            style: TextStyle::new(Color::from_palette(5)),
        }));
    }

    pub(crate) fn render_contact_terminal(&self, view: &mut View) {
        let width = self.terminal_cols * 8;
        let card_w = width.saturating_sub(16);

        // Header at row 3 (y = 48)
        view.add(Element::Text(TextElement {
            x: 8,
            y: 48,
            text: "CONNECT & DIRECT CONTACT:".to_string(),
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));

        // Note at row 4 (y = 64)
        view.add(Element::Text(TextElement {
            x: 8,
            y: 64,
            text: "Direct interactive links and endpoints:".to_string(),
            style: TextStyle::new(Color::from_palette(4)),
        }));

        // Divider at row 5 (y = 80)
        view.add(Element::Rect(RectElement {
            x: 0,
            y: 80,
            width,
            height: 1,
            color: Color::from_palette(7),
            filled: true,
        }));

        let links = [
            (
                "Email:    dyemelianov@icloud.com",
                "mailto:dyemelianov@icloud.com",
                96,
            ),
            (
                "GitHub:   https://github.com/dmytro-yemelianov",
                "https://github.com/dmytro-yemelianov",
                128,
            ),
            (
                "LinkedIn: https://linkedin.com/in/dmytro-yemelianov",
                "https://linkedin.com/in/dmytro-yemelianov",
                160,
            ),
            (
                "Website:  https://yemelianov.dev",
                "https://yemelianov.dev",
                192,
            ),
            (
                "SSH:      ssh guest@yemelianov.dev (Port 2222)",
                "ssh://guest@yemelianov.dev:2222",
                224,
            ),
        ];

        for (label, url, ly) in links {
            view.add(Element::Rect(RectElement {
                x: 8,
                y: ly,
                width: card_w,
                height: 16,
                color: Color::from_palette(1),
                filled: true,
            }));
            view.add(Element::Link(LinkElement::new(
                16,
                ly,
                label,
                url,
                TextStyle::new(Color::from_palette(12)).bold(),
            )));
        }

        let mut y = 264;
        let info = "Location: Vinnytsia, Ukraine / Remote worldwide\nAvailability: Full-time Senior / Lead / Architect roles in Systems, AI Infrastructure, and Agentic Engineering.";
        let wrap_w = (self.terminal_cols as usize).saturating_sub(4).max(30);
        for line in word_wrap(info, wrap_w) {
            view.add(Element::Text(TextElement {
                x: 8,
                y,
                text: line,
                style: TextStyle::new(Color::from_palette(5)),
            }));
            y += 16;
        }
    }
}
