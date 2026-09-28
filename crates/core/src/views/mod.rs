//! Top-level view rendering, navigation bar, clock, and status bar.

pub mod about;
pub mod contact;
pub mod help;
pub mod modals;
pub mod projects;
pub mod resume;
pub mod visuals;

use pixel_ssh_view::{
    horizontal_scroll, ActiveModal, Color, Cursor, Element, LinkElement, Platform, RectElement,
    TextElement, TextStyle, View,
};
use crate::state::{App, Tab};

impl App {
    pub fn render(&self) -> View {
        let (width, height) = if self.platform == Platform::Terminal {
            (self.terminal_cols * 8, self.terminal_rows * 16)
        } else {
            self.system_mode.resolution()
        };
        let (cols, _rows) = if self.platform == Platform::Terminal {
            (self.terminal_cols, self.terminal_rows)
        } else {
            self.system_mode.char_grid()
        };
        let mut view = View::new(width, height);
        view.resolution = self.resolution;
        view.color_theme = self.color_theme;
        view.palette_mode = self.palette_mode;
        view.system_mode = self.system_mode;
        view.visual_effects = self.visual_effects;
        view.platform = self.platform;
        view.mouse_pos = self.mouse_pos;

        // 0. Screensaver hot corner animation (if active, full screen starfield warp)
        if self.screensaver_active {
            self.render_screensaver(&mut view, width, height);
            return view;
        }

        if self.platform == Platform::Terminal {
            self.render_chrome_terminal(&mut view, width, height, cols);
        } else if cols >= 80 {
            // Row 0: Header background (palette index 1 = dark card)
            view.add(Element::Rect(RectElement {
                x: 0,
                y: 0,
                width,
                height: 16,
                color: Color::from_palette(1),
                filled: true,
            }));

            // Title text with subtle color cycle / pulse
            let title_color = match (self.tick / 8) % 4 { 0 => 7, 1 => 15, 2 => 14, _ => 7 };
            let title_w = ((width.saturating_sub(260)) / 8) as usize;
            let title = horizontal_scroll("DMYTRO YEMELIANOV - SYSTEMS & AI ARCHITECT", title_w.max(20), self.tick);
            view.add(Element::Text(TextElement {
                x: 8,
                y: 0,
                text: title,
                style: TextStyle::new(Color::from_palette(title_color)).bold(),
            }));

            // Header link to personal website
            view.add(Element::Link(LinkElement::new(
                width.saturating_sub(188),
                0,
                "[ yemelianov.dev ]",
                "https://yemelianov.dev",
                TextStyle::new(Color::from_palette(12)),
            )));

            // Top Bar Clock: at far top right, Volkov Commander solid badge
            let clock_str = format!(" {} ", self.clock_formatted());
            let clock_x = width.saturating_sub(60);
            view.add(Element::Rect(RectElement {
                x: clock_x,
                y: 0,
                width: 60,
                height: 16,
                color: Color::from_palette(13),
                filled: true,
            }));
            view.add(Element::Rect(RectElement {
                x: clock_x,
                y: 0,
                width: 60,
                height: 16,
                color: Color::from_palette(3),
                filled: false,
            }));
            view.add(Element::Text(TextElement {
                x: clock_x + 2,
                y: 0,
                text: clock_str,
                style: TextStyle::new(Color::from_palette(6)).bold(),
            }));
        } else if cols == 40 {
            // 40 columns (Commodore 64 / Atari 800 / CGA)
            view.add(Element::Rect(RectElement {
                x: 0,
                y: 0,
                width,
                height: 10,
                color: Color::from_palette(1),
                filled: true,
            }));

            let title_color = match (self.tick / 8) % 4 { 0 => 7, 1 => 15, 2 => 14, _ => 7 };
            let hdr_title = horizontal_scroll("DMYTRO YEMELIANOV", 18, self.tick);
            view.add(Element::Text(TextElement {
                x: 4,
                y: 1,
                text: hdr_title,
                style: TextStyle::new(Color::from_palette(title_color)).bold(),
            }));

            // Link [dev]
            view.add(Element::Link(LinkElement::new(
                width.saturating_sub(92),
                1,
                "[dev]",
                "https://yemelianov.dev",
                TextStyle::new(Color::from_palette(12)),
            )));

            // Top Bar Clock at far right with solid badge
            let clock_str = self.clock_formatted();
            let clock_x = width.saturating_sub(48);
            view.add(Element::Rect(RectElement {
                x: clock_x.saturating_sub(2),
                y: 0,
                width: 48,
                height: 10,
                color: Color::from_palette(13),
                filled: true,
            }));
            view.add(Element::Rect(RectElement {
                x: clock_x.saturating_sub(2),
                y: 0,
                width: 48,
                height: 10,
                color: Color::from_palette(3),
                filled: false,
            }));
            view.add(Element::Text(TextElement {
                x: clock_x + 1,
                y: 1,
                text: clock_str,
                style: TextStyle::new(Color::from_palette(6)).bold(),
            }));
        } else {
            // 32 columns (ZX Spectrum)
            view.add(Element::Rect(RectElement {
                x: 0,
                y: 0,
                width,
                height: 10,
                color: Color::from_palette(1),
                filled: true,
            }));

            let title_color = match (self.tick / 8) % 4 { 0 => 7, 1 => 15, 2 => 14, _ => 7 };
            let hdr_title = horizontal_scroll("DMYTRO YEMELIANOV", 13, self.tick);
            view.add(Element::Text(TextElement {
                x: 2,
                y: 1,
                text: hdr_title,
                style: TextStyle::new(Color::from_palette(title_color)).bold(),
            }));

            // Link [dev]
            view.add(Element::Link(LinkElement::new(
                width.saturating_sub(92),
                1,
                "[dev]",
                "https://yemelianov.dev",
                TextStyle::new(Color::from_palette(12)),
            )));

            // Top Bar Clock at top right with solid badge
            let clock_str = self.clock_formatted();
            let clock_x = width.saturating_sub(48);
            view.add(Element::Rect(RectElement {
                x: clock_x.saturating_sub(2),
                y: 0,
                width: 48,
                height: 10,
                color: Color::from_palette(13),
                filled: true,
            }));
            view.add(Element::Rect(RectElement {
                x: clock_x.saturating_sub(2),
                y: 0,
                width: 48,
                height: 10,
                color: Color::from_palette(3),
                filled: false,
            }));
            view.add(Element::Text(TextElement {
                x: clock_x + 1,
                y: 1,
                text: clock_str,
                style: TextStyle::new(Color::from_palette(6)).bold(),
            }));
        }
        // Content Area
        if self.platform == Platform::Web {
            if self.current_tab == Tab::Contact {
                self.render_contact(&mut view);
            } else {
                self.render_projects(&mut view);
            }

            // About and CV (Resume) are reader articles overlaying on top of Projects
            if self.current_tab == Tab::Resume || self.current_tab == Tab::About {
                self.render_article_overlay(&mut view, width, height, cols);
            }
        } else {
            match self.current_tab {
                Tab::Projects => self.render_projects(&mut view),
                Tab::Resume => self.render_resume(&mut view),
                Tab::About => self.render_about(&mut view),
                Tab::Visuals => self.render_about(&mut view),
                Tab::Contact => self.render_contact(&mut view),
                Tab::Help => self.render_help(&mut view),
            }
        }

        // Active Modal dialog overlay (System, Visuals, Help)
        if self.active_modal != ActiveModal::None {
            match self.active_modal {
                ActiveModal::System => self.render_system_modal(&mut view, width, height, cols),
                ActiveModal::Visuals => self.render_visuals_modal(&mut view, width, height, cols),
                ActiveModal::Help => self.render_help_modal(&mut view, width, height, cols),
                ActiveModal::None => {}
            }
        }

        // Single Global Bottom Navigation Bar (across all views and modals)
        self.render_bottom_navigation(&mut view, width, height, cols);

        // Footer status bar
        if self.platform == Platform::Terminal {
            let footer_y = (self.terminal_rows.saturating_sub(1)) * 16;
            view.add(Element::Rect(RectElement {
                x: 0,
                y: footer_y,
                width,
                height: 16,
                color: Color::from_palette(1),
                filled: true,
            }));

            view.add(Element::Text(TextElement {
                x: 8,
                y: footer_y,
                text: self.status.clone(),
                style: TextStyle::new(Color::from_palette(8)),
            }));

            view.cursor = Some(Cursor {
                x: width.saturating_sub(16),
                y: footer_y,
                visible: true,
            });
        } else if cols >= 80 {
            let footer_y = height.saturating_sub(16);
            view.add(Element::Rect(RectElement {
                x: 0,
                y: footer_y,
                width,
                height: 16,
                color: Color::from_palette(1),
                filled: true,
            }));

            let status_msg = if self.active_modal != ActiveModal::None {
                format!("DIALOG ACTIVE: {:?} • Click [X] or press [ESC] to dismiss", self.active_modal)
            } else if self.current_tab == Tab::Resume || self.current_tab == Tab::About {
                "ARTICLE VIEW: [j/k/Wheel] Scroll • [ESC] Return to Projects".to_string()
            } else {
                self.status.clone()
            };

            view.add(Element::Text(TextElement {
                x: 8,
                y: footer_y,
                text: status_msg,
                style: TextStyle::new(Color::from_palette(8)),
            }));

            view.cursor = Some(Cursor {
                x: width.saturating_sub(16),
                y: footer_y,
                visible: true,
            });
        } else if cols == 40 {
            let footer_y = height.saturating_sub(10);
            view.add(Element::Rect(RectElement {
                x: 0,
                y: footer_y,
                width,
                height: 10,
                color: Color::from_palette(1),
                filled: true,
            }));

            let footer_text = if self.active_modal != ActiveModal::None {
                "[ESC] Close Dialog"
            } else if self.current_tab == Tab::Resume || self.current_tab == Tab::About {
                "[j/k] Scroll [ESC] Return"
            } else if self.platform == Platform::Web {
                "[1-4]Nav [VIS] [SYS] [j/k] [Enter]"
            } else {
                "[1-5]Tabs [S]Sys [j/k]Scroll [Enter]"
            };

            view.add(Element::Text(TextElement {
                x: 4,
                y: footer_y + 1,
                text: footer_text.to_string(),
                style: TextStyle::new(Color::from_palette(8)),
            }));

            view.cursor = Some(Cursor {
                x: width.saturating_sub(12),
                y: footer_y + 1,
                visible: true,
            });
        } else {
            // 32 cols (ZX Spectrum)
            let footer_y = height.saturating_sub(10);
            view.add(Element::Rect(RectElement {
                x: 0,
                y: footer_y,
                width,
                height: 10,
                color: Color::from_palette(1),
                filled: true,
            }));

            let footer_text = if self.active_modal != ActiveModal::None {
                "ESC: Close Dialog"
            } else if self.current_tab == Tab::Resume || self.current_tab == Tab::About {
                "j/k: Scroll ESC: Exit"
            } else if self.platform == Platform::Web {
                "1-4:Tab VIS SYS k/j:Nav"
            } else {
                "1-5:Tab S:Sys k/j:Nav ?:Help"
            };
            view.add(Element::Text(TextElement {
                x: 2,
                y: footer_y + 1,
                text: footer_text.to_string(),
                style: TextStyle::new(Color::from_palette(8)),
            }));

            view.cursor = Some(Cursor {
                x: width.saturating_sub(10),
                y: footer_y + 1,
                visible: true,
            });
        }

        view
    }

    fn render_chrome_terminal(&self, view: &mut View, width: u16, _height: u16, cols: u16) {
        // Row 0: Top Header (y = 0)
        view.add(Element::Rect(RectElement {
            x: 0,
            y: 0,
            width,
            height: 16,
            color: Color::from_palette(1),
            filled: true,
        }));

        let title_color = match (self.tick / 8) % 4 { 0 => 7, 1 => 15, 2 => 14, _ => 7 };
        let title_w = (cols as usize).saturating_sub(38).max(24);
        let title = horizontal_scroll("DMYTRO YEMELIANOV - SYSTEMS & AI ARCHITECT", title_w, self.tick);
        view.add(Element::Text(TextElement {
            x: 8,
            y: 0,
            text: title,
            style: TextStyle::new(Color::from_palette(title_color)).bold(),
        }));

        // Link at far right
        let link_x = width.saturating_sub(132);
        view.add(Element::Link(LinkElement::new(
            link_x,
            0,
            "[ dev ]",
            "https://yemelianov.dev",
            TextStyle::new(Color::from_palette(12)),
        )));

        // Clock at far top right: HH:MM, no brackets, no seconds
        let clock_str = self.clock_formatted();
        let clock_x = width.saturating_sub(48);
        view.add(Element::Text(TextElement {
            x: clock_x,
            y: 0,
            text: clock_str,
            style: TextStyle::new(Color::from_palette(10)).bold(),
        }));

        // Top bar only: title, dev link, and clock. Navigation is at the bottom.
    }

    /// Single global bottom navigation bar rendered identically across all views and modals.
    pub(crate) fn render_bottom_navigation(&self, view: &mut View, width: u16, height: u16, cols: u16) {
        if cols >= 80 {
            let nav_y = if self.platform == Platform::Terminal {
                (self.terminal_rows.saturating_sub(2)) * 16
            } else {
                height.saturating_sub(32)
            };

            // Full background across the navigation row
            view.add(Element::Rect(RectElement {
                x: 0,
                y: nav_y,
                width,
                height: 16,
                color: Color::from_palette(13), // authentic black background
                filled: true,
            }));

            let is_web = self.platform == Platform::Web;
            let active_prj = self.current_tab == Tab::Projects && !self.show_detail && self.active_modal == ActiveModal::None;
            let active_cv = self.current_tab == Tab::Resume && self.active_modal == ActiveModal::None;
            let active_abt = self.current_tab == Tab::About && self.active_modal == ActiveModal::None;
            let active_cnt = self.current_tab == Tab::Contact && self.active_modal == ActiveModal::None;
            let active_hlp = self.active_modal == ActiveModal::Help || (self.current_tab == Tab::Help && !is_web);
            let active_vis = self.active_modal == ActiveModal::Visuals;
            let active_res = self.active_modal == ActiveModal::System;
            let active_thm = false;
            let active_view = self.show_detail && self.current_tab == Tab::Projects;
            let active_quit = false;

            let items: [(usize, &str, bool); 10] = [
                (1, "Prj", active_prj),
                (2, if is_web { "CV" } else { "Resume" }, active_cv),
                (3, "About", active_abt),
                (4, "Contct", active_cnt),
                (5, "Help", active_hlp),
                (6, if is_web { "Visual" } else { "Sys" }, active_vis),
                (7, "Res", active_res),
                (8, "Theme", active_thm),
                (9, "View", active_view),
                (10, "Quit", active_quit),
            ];

            let slot_w = width / 10;
            for (i, &(key_num, label, is_active)) in items.iter().enumerate() {
                let slot_x = (i as u16) * slot_w;
                let num_str = format!("{}", key_num);
                let num_w = (num_str.len() as u16) * 8;

                // Number text (bright white or yellow)
                view.add(Element::Text(TextElement {
                    x: slot_x + 2,
                    y: nav_y,
                    text: num_str,
                    style: TextStyle::new(Color::from_palette(14)).bold(),
                }));

                // Button badge background
                let badge_x = slot_x + num_w + 3;
                let badge_w = slot_w.saturating_sub(num_w + 5);
                let badge_bg = if is_active {
                    Color::from_palette(6) // Volkov bright yellow
                } else {
                    Color::from_palette(3) // Volkov cyan
                };
                let badge_fg = if is_active {
                    Color::from_palette(0) // dark navy blue
                } else {
                    Color::from_palette(13) // black
                };

                view.add(Element::Rect(RectElement {
                    x: badge_x,
                    y: nav_y,
                    width: badge_w,
                    height: 16,
                    color: badge_bg,
                    filled: true,
                }));

                view.add(Element::Text(TextElement {
                    x: badge_x + 2,
                    y: nav_y,
                    text: label.to_string(),
                    style: TextStyle::new(badge_fg).bold(),
                }));
            }
        } else if cols == 40 {
            let nav_y = height.saturating_sub(20);
            view.add(Element::Rect(RectElement {
                x: 0,
                y: nav_y,
                width,
                height: 10,
                color: Color::from_palette(13),
                filled: true,
            }));

            let active_prj = self.current_tab == Tab::Projects && !self.show_detail && self.active_modal == ActiveModal::None;
            let active_cv = self.current_tab == Tab::Resume && self.active_modal == ActiveModal::None;
            let active_abt = self.current_tab == Tab::About && self.active_modal == ActiveModal::None;
            let active_cnt = self.current_tab == Tab::Contact && self.active_modal == ActiveModal::None;
            let active_hlp = self.active_modal == ActiveModal::Help;
            let active_sys = self.active_modal == ActiveModal::System;

            let items = [
                (1, "PRJ", active_prj),
                (2, "CV", active_cv),
                (3, "ABT", active_abt),
                (4, "CNT", active_cnt),
                (5, "?", active_hlp),
                (6, "SYS", active_sys),
            ];

            let slot_w = width / 6;
            for (i, &(k, lbl, active)) in items.iter().enumerate() {
                let sx = (i as u16) * slot_w;
                let bg = if active { Color::from_palette(6) } else { Color::from_palette(3) };
                let fg = if active { Color::from_palette(0) } else { Color::from_palette(13) };
                view.add(Element::Rect(RectElement {
                    x: sx + 10,
                    y: nav_y,
                    width: slot_w.saturating_sub(12),
                    height: 9,
                    color: bg,
                    filled: true,
                }));
                view.add(Element::Text(TextElement {
                    x: sx + 2,
                    y: nav_y,
                    text: format!("{}", k),
                    style: TextStyle::new(Color::from_palette(14)).bold(),
                }));
                view.add(Element::Text(TextElement {
                    x: sx + 12,
                    y: nav_y,
                    text: lbl.to_string(),
                    style: TextStyle::new(fg).bold(),
                }));
            }
        } else {
            // 32 columns (ZX Spectrum)
            let nav_y = height.saturating_sub(20);
            view.add(Element::Rect(RectElement {
                x: 0,
                y: nav_y,
                width,
                height: 10,
                color: Color::from_palette(13),
                filled: true,
            }));

            let active_prj = self.current_tab == Tab::Projects && !self.show_detail && self.active_modal == ActiveModal::None;
            let active_cv = self.current_tab == Tab::Resume && self.active_modal == ActiveModal::None;
            let active_abt = self.current_tab == Tab::About && self.active_modal == ActiveModal::None;
            let active_cnt = self.current_tab == Tab::Contact && self.active_modal == ActiveModal::None;
            let active_hlp = self.active_modal == ActiveModal::Help;
            let active_sys = self.active_modal == ActiveModal::System;

            let items = [
                (1, "PR", active_prj),
                (2, "CV", active_cv),
                (3, "AB", active_abt),
                (4, "CT", active_cnt),
                (5, "?", active_hlp),
                (6, "SY", active_sys),
            ];

            let slot_w = width / 6;
            for (i, &(k, lbl, active)) in items.iter().enumerate() {
                let sx = (i as u16) * slot_w;
                let bg = if active { Color::from_palette(6) } else { Color::from_palette(3) };
                let fg = if active { Color::from_palette(0) } else { Color::from_palette(13) };
                view.add(Element::Rect(RectElement {
                    x: sx + 9,
                    y: nav_y,
                    width: slot_w.saturating_sub(11),
                    height: 9,
                    color: bg,
                    filled: true,
                }));
                view.add(Element::Text(TextElement {
                    x: sx + 1,
                    y: nav_y,
                    text: format!("{}", k),
                    style: TextStyle::new(Color::from_palette(14)).bold(),
                }));
                view.add(Element::Text(TextElement {
                    x: sx + 11,
                    y: nav_y,
                    text: lbl.to_string(),
                    style: TextStyle::new(fg).bold(),
                }));
            }
        }
    }
}
