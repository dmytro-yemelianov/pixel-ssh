//! Contact tab view implementation.

use pixel_ssh_view::{
    word_wrap, Color, Element, LinkElement, Platform, RectElement, TextElement, TextStyle, View,
};
use crate::state::App;

impl App {
    pub(crate) fn render_contact(&self, view: &mut View) {
        if self.platform == Platform::Terminal {
            self.render_contact_terminal(view);
            return;
        }
        let (cols, _) = self.palette_mode.char_grid();
        match cols {
            100 | 80 => self.render_contact_80(view),
            40 => self.render_contact_40(view),
            _ => self.render_contact_32(view),
        }
    }

    pub(crate) fn render_contact_80(&self, view: &mut View) {
        view.add(Element::Text(TextElement {
            x: 16,
            y: 48,
            text: "CONNECT & DIRECT CONTACT:".to_string(),
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));

        view.add(Element::Text(TextElement {
            x: 16,
            y: 64,
            text: "Click any underlined link below to open directly in a new tab:".to_string(),
            style: TextStyle::new(Color::from_palette(4)),
        }));

        // Divider (Row 5: y = 80)
        view.add(Element::Rect(RectElement {
            x: 0,
            y: 80,
            width: 640,
            height: 1,
            color: Color::from_palette(7),
            filled: true,
        }));

        // Clickable contact cards (Rows 6, 8, 10, 12, 14)
        let links = [
            ("Email:    dyemelianov@icloud.com", "mailto:dyemelianov@icloud.com", 96),
            ("GitHub:   https://github.com/dmytro-yemelianov", "https://github.com/dmytro-yemelianov", 128),
            ("LinkedIn: https://linkedin.com/in/dmytro-yemelianov", "https://linkedin.com/in/dmytro-yemelianov", 160),
            ("Website:  https://yemelianov.dev", "https://yemelianov.dev", 192),
            ("SSH:      ssh guest@yemelianov.dev (Port 2222)", "ssh://guest@yemelianov.dev:2222", 224),
        ];

        for (label, url, ly) in links {
            view.add(Element::Rect(RectElement {
                x: 0,
                y: ly,
                width: 640,
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

        // Additional information wrapped (Row 17: y = 272)
        let mut y = 272;
        let info = "Location: Vinnytsia, Ukraine / Remote worldwide\nAvailability: Full-time Senior / Lead / Architect roles in Systems, AI Infrastructure, and Agentic Engineering.";
        for line in word_wrap(info, 74) {
            view.add(Element::Text(TextElement {
                x: 16,
                y,
                text: line,
                style: TextStyle::new(Color::from_palette(5)),
            }));
            y += 16;
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
            ("Email:   dyemelianov@icloud.com", "mailto:dyemelianov@icloud.com", 47),
            ("GitHub:  github.com/dmytro-yemelianov", "https://github.com/dmytro-yemelianov", 72),
            ("LinkedIn: linkedin.com/in/dmytro-yemelianov", "https://linkedin.com/in/dmytro-yemelianov", 97),
            ("Website: https://yemelianov.dev", "https://yemelianov.dev", 122),
            ("SSH:     ssh -p 2222 guest@dev", "ssh://guest@yemelianov.dev:2222", 147),
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
            ("Email: dyemelianov@icloud.com", "mailto:dyemelianov@icloud.com", 46),
            ("GH:    github/.../yemelianov", "https://github.com/dmytro-yemelianov", 70),
            ("In:    linkedin/.../yemelianov", "https://linkedin.com/in/dmytro-yemelianov", 94),
            ("Web:   yemelianov.dev", "https://yemelianov.dev", 118),
            ("SSH:   ssh -p 2222 guest@dev", "ssh://guest@yemelianov.dev:2222", 142),
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
            ("Email:    dyemelianov@icloud.com", "mailto:dyemelianov@icloud.com", 96),
            ("GitHub:   https://github.com/dmytro-yemelianov", "https://github.com/dmytro-yemelianov", 128),
            ("LinkedIn: https://linkedin.com/in/dmytro-yemelianov", "https://linkedin.com/in/dmytro-yemelianov", 160),
            ("Website:  https://yemelianov.dev", "https://yemelianov.dev", 192),
            ("SSH:      ssh guest@yemelianov.dev (Port 2222)", "ssh://guest@yemelianov.dev:2222", 224),
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
