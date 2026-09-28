//! Help and keyboard / touch reference tab view implementation.

use crate::state::App;
use pixel_ssh_view::{Color, Element, Platform, RectElement, TextElement, TextStyle, View};

impl App {
    pub(crate) fn render_help(&self, view: &mut View) {
        if self.platform == Platform::Terminal {
            self.render_help_terminal(view);
            return;
        }
        let (cols, _) = self.resolution.char_grid();
        match cols {
            100 | 80 => self.render_help_80(view),
            40 => self.render_help_40(view),
            _ => self.render_help_32(view),
        }
    }

    pub(crate) fn render_help_80(&self, view: &mut View) {
        view.add(Element::Text(TextElement {
            x: 16,
            y: 48,
            text: "INTERACTION GUIDE & PLATFORM ARCHITECTURE:".to_string(),
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));

        view.add(Element::Rect(RectElement {
            x: 0,
            y: 64,
            width: 640,
            height: 1,
            color: Color::from_palette(7),
            filled: true,
        }));

        let shortcuts = vec![
            "[P/H/S/V/Q] Projects, Help, cycle System, Visuals, Quit",
            "[V] Visuals  [Tab] Cycle views  [Up/Down or j/k] Navigate",
            "[Enter] Select / details  [Esc] Back  [PgUp/PgDn] Page scroll",
            "[h/l] Previous/next project in detail; h opens Help on the list",
        ];

        let mouse_items = vec![
            "Menu controls:   Click Prj, Hlp, Sys, Vis, or Qut",
            "Resume / About:   Click ▲ / ▼ or drag/click vertical scrollbar track",
            "Wheel Navigation: Scroll up and down through content lines and projects",
            "Project Rows:     Click project row to select; click again for details",
            "Hyperlinks:       Click underlined links to open repo, email, or profile",
        ];

        let access_items = if self.platform == Platform::Web {
            vec![
                "SSH Server:       ssh -p 2222 guest@yemelianov.dev (or localhost:2222)",
                "Full Screen ANSI: Dynamic terminal reflow, 24-bit TrueColor stream",
            ]
        } else {
            vec![
                "Browser URL:      https://yemelianov.dev",
                "WebGL2 Shaders:   Scanlines, bloom, aberration, phosphor glow, snow",
            ]
        };

        let sections = [
            ("KEYBOARD SHORTCUTS:", shortcuts),
            ("MOUSE & TOUCH INTERACTION:", mouse_items),
            ("PLATFORM ARCHITECTURE & ACCESS:", access_items),
        ];

        let mut y = 80;
        for (header, lines) in sections {
            view.add(Element::Text(TextElement {
                x: 16,
                y,
                text: header.to_string(),
                style: TextStyle::new(Color::from_palette(7)).bold(),
            }));
            y += 16;
            for line in lines {
                view.add(Element::Text(TextElement {
                    x: 24,
                    y,
                    text: line.to_string(),
                    style: TextStyle::new(Color::from_palette(5)),
                }));
                y += 16;
            }
            y += 16;
        }
    }

    pub(crate) fn render_help_40(&self, view: &mut View) {
        view.add(Element::Text(TextElement {
            x: 4,
            y: 24,
            text: "GUIDE & CONTROLS:".to_string(),
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));

        view.add(Element::Rect(RectElement {
            x: 4,
            y: 33,
            width: 312,
            height: 1,
            color: Color::from_palette(3),
            filled: true,
        }));

        let mut lines = vec![
            ("KEYBOARD SHORTCUTS:", 7, true),
            ("P Prj H Hlp S Sys V Vis", 5, false),
            ("Q Quit; S cycles systems", 5, false),
            ("[h/l] Prev/Next in detail", 5, false),
            ("[k/j] Scroll [Enter] Detail", 5, false),
            ("[Esc] Return to list", 5, false),
        ];

        if self.platform == Platform::Web {
            lines.push(("[V] Visual effects + presets", 5, false));
        }

        lines.extend_from_slice(&[
            ("", 5, false),
            ("MOUSE & TOUCH:", 7, true),
            ("Click tabs, [S]Sys, or rows", 5, false),
            ("Click scrollbar or use wheel", 5, false),
            ("Click links to open directly", 5, false),
            ("", 5, false),
        ]);

        if self.platform == Platform::Web {
            lines.extend_from_slice(&[
                ("TERMINAL SSH ACCESS:", 7, true),
                ("ssh -p 2222 guest@yemelianov.dev", 6, false),
                ("Direct ANSI terminal stream", 4, false),
            ]);
        } else {
            lines.extend_from_slice(&[
                ("WEB INTERFACE:", 7, true),
                ("https://yemelianov.dev", 6, false),
                ("Pixel-first WebGL2 client", 4, false),
            ]);
        }

        let mut y = 38;
        for (line_text, pal_idx, bold) in lines {
            if !line_text.is_empty() {
                let mut style = TextStyle::new(Color::from_palette(pal_idx));
                if bold {
                    style = style.bold();
                }
                view.add(Element::Text(TextElement {
                    x: 4,
                    y,
                    text: line_text.to_string(),
                    style,
                }));
            }
            y += 10;
        }
    }

    pub(crate) fn render_help_32(&self, view: &mut View) {
        view.add(Element::Text(TextElement {
            x: 2,
            y: 24,
            text: "CONTROLS & HELP:".to_string(),
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));

        view.add(Element::Rect(RectElement {
            x: 2,
            y: 33,
            width: 252,
            height: 1,
            color: Color::from_palette(3),
            filled: true,
        }));

        let tab_line = "P Prj H Hlp S Sys V Vis";

        let mut lines = vec![
            ("KEYBOARD CONTROLS:", 7, true),
            (tab_line, 5, false),
            ("Q Quit; S cycles systems", 5, false),
            ("h/l: Prev/Next in detail", 5, false),
            ("k/j: Scroll Enter: Detail", 5, false),
            ("Esc: Return to list", 5, false),
            ("", 5, false),
            ("MOUSE & TOUCH:", 7, true),
            ("Click menu or project rows", 5, false),
            ("Click scrollbar or use wheel", 5, false),
            ("Click links to open directly", 5, false),
            ("", 5, false),
        ];

        if self.platform == Platform::Web {
            lines.extend_from_slice(&[
                ("TERMINAL SSH ACCESS:", 7, true),
                ("ssh -p 2222", 6, false),
                ("guest@yemelianov.dev", 6, false),
            ]);
        } else {
            lines.extend_from_slice(&[
                ("WEB INTERFACE:", 7, true),
                ("yemelianov.dev", 6, false),
                ("WASM + WebGL2 shaders", 6, false),
            ]);
        }

        let mut y = 38;
        for (line_text, pal_idx, bold) in lines {
            if !line_text.is_empty() {
                let mut style = TextStyle::new(Color::from_palette(pal_idx));
                if bold {
                    style = style.bold();
                }
                view.add(Element::Text(TextElement {
                    x: 2,
                    y,
                    text: line_text.to_string(),
                    style,
                }));
            }
            y += 8;
        }
    }

    pub(crate) fn render_help_terminal(&self, view: &mut View) {
        let width = self.terminal_cols * 8;

        // Row 3 (y = 48)
        view.add(Element::Text(TextElement {
            x: 8,
            y: 48,
            text: "INTERACTION GUIDE & PLATFORM ARCHITECTURE:".to_string(),
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));

        // Row 4 (y = 64): Divider
        view.add(Element::Rect(RectElement {
            x: 0,
            y: 64,
            width,
            height: 1,
            color: Color::from_palette(7),
            filled: true,
        }));

        let sections = [
            ("KEYBOARD SHORTCUTS:", vec![
                "[P/H/S/V/Q]      Projects, Help, cycle System, Visuals, Quit",
                "[?]                Toggle this Help Reference from any tab",
                "[Tab]              Cycle forward to next tab / switch links in detail",
                "[S]               Cycle display systems with native font and colors",
                "[Up/Down] or [k/j] Navigate project list OR vertically scroll content",
                "[PgUp / PgDn]      Fast scroll Resume & About content up / down",
                "[Home / End]       Jump directly to the start or end of lists and text",
                "[Enter]            Open detailed project view OR trigger focused link",
                "[Esc] or [q]       Close project detail / return to list",
                "[Ctrl+C]           Terminate session and cleanly return to your host shell",
            ]),
            ("TERMINAL HOST INTEGRATION:", vec![
                "Full Screen Reflow: Dynamically adapts to your host terminal window dimensions on resize",
                "TrueColor ANSI:     Emits 24-bit RGB differential sequences into the terminal alternate buffer",
                "Web Version:        Live at https://yemelianov.dev with authentic retro CRT WebGL2 shaders",
            ]),
        ];

        let mut y = 80;
        for (header, lines) in sections {
            view.add(Element::Text(TextElement {
                x: 8,
                y,
                text: header.to_string(),
                style: TextStyle::new(Color::from_palette(7)).bold(),
            }));
            y += 16;
            for line in lines {
                view.add(Element::Text(TextElement {
                    x: 16,
                    y,
                    text: line.to_string(),
                    style: TextStyle::new(Color::from_palette(5)),
                }));
                y += 16;
            }
            y += 16;
        }
    }
}
