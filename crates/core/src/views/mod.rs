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

            // Top Bar Clock: at far top right, HH:MM, no seconds, no brackets
            let clock_str = self.clock_formatted();
            let clock_x = width.saturating_sub(48);
            view.add(Element::Text(TextElement {
                x: clock_x,
                y: 0,
                text: clock_str,
                style: TextStyle::new(Color::from_palette(10)).bold(),
            }));

            // Row 1: Tab bar background
            view.add(Element::Rect(RectElement {
                x: 0,
                y: 16,
                width,
                height: 16,
                color: Color::from_palette(0),
                filled: true,
            }));

            let tabs: &[(Tab, &str, u16)] = if self.platform == Platform::Web {
                &[
                    (Tab::Projects, "[1] Projects", 8),
                    (Tab::Resume, "[2] CV", 112),
                    (Tab::About, "[3] About", 168),
                    (Tab::Contact, "[4] Contact", 248),
                ]
            } else {
                &[
                    (Tab::Projects, "[1] Projects", 8),
                    (Tab::Resume, "[2] Resume", 120),
                    (Tab::About, "[3] About", 216),
                    (Tab::Contact, "[4] Contact", 304),
                    (Tab::Help, "[5] Help", 400),
                ]
            };

            for &(tab, label, tx) in tabs {
                let active = self.current_tab == tab;
                let style = if active {
                    TextStyle::new(Color::from_palette(6))
                        .with_bg(Color::from_palette(7))
                        .bold()
                } else {
                    TextStyle::new(Color::from_palette(4))
                };
                view.add(Element::Text(TextElement {
                    x: tx,
                    y: 16,
                    text: label.to_string(),
                    style,
                }));
            }

            if self.platform == Platform::Web {
                // Modal triggers: Visuals, System, Help
                let vis_active = self.active_modal == ActiveModal::Visuals;
                let vis_style = if vis_active {
                    TextStyle::new(Color::from_palette(6)).with_bg(Color::from_palette(7)).bold()
                } else {
                    TextStyle::new(Color::from_palette(11))
                };
                view.add(Element::Link(LinkElement::new(
                    344,
                    16,
                    "[VIS]",
                    "#modal-visuals",
                    vis_style,
                )));

                let sys_active = self.active_modal == ActiveModal::System;
                let sys_style = if sys_active {
                    TextStyle::new(Color::from_palette(6)).with_bg(Color::from_palette(7)).bold()
                } else {
                    TextStyle::new(Color::from_palette(9)).bold()
                };
                view.add(Element::Link(LinkElement::new(
                    392,
                    16,
                    format!("[SYS:{}]", self.system_mode.short_name()),
                    "#modal-system",
                    sys_style,
                )));

                let hlp_active = self.active_modal == ActiveModal::Help;
                let hlp_style = if hlp_active {
                    TextStyle::new(Color::from_palette(6)).with_bg(Color::from_palette(7)).bold()
                } else {
                    TextStyle::new(Color::from_palette(8))
                };
                view.add(Element::Link(LinkElement::new(
                    480,
                    16,
                    "[?]",
                    "#modal-help",
                    hlp_style,
                )));
            } else {
                let sys_text = format!("[S] {}", self.system_mode.short_name());
                let sys_x = width.saturating_sub((sys_text.len() as u16 + 2) * 8);
                view.add(Element::Link(LinkElement::new(
                    sys_x,
                    16,
                    sys_text,
                    "#system",
                    TextStyle::new(Color::from_palette(9)).bold(),
                )));
            }

            // Row 2: Tab separator line (y = 32)
            view.add(Element::Rect(RectElement {
                x: 0,
                y: 32,
                width,
                height: 1,
                color: Color::from_palette(7),
                filled: true,
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

            // Top Bar Clock at far right
            let clock_str = self.clock_formatted();
            view.add(Element::Text(TextElement {
                x: width.saturating_sub(44),
                y: 1,
                text: clock_str,
                style: TextStyle::new(Color::from_palette(10)).bold(),
            }));

            view.add(Element::Rect(RectElement {
                x: 0,
                y: 10,
                width,
                height: 10,
                color: Color::from_palette(0),
                filled: true,
            }));

            let tabs: &[(Tab, &str, u16)] = if self.platform == Platform::Web {
                &[
                    (Tab::Projects, "1:PRJ", 2),
                    (Tab::Resume, "2:CV", 44),
                    (Tab::About, "3:ABT", 86),
                    (Tab::Contact, "4:CNT", 128),
                ]
            } else {
                &[
                    (Tab::Projects, "1:PRJ", 4),
                    (Tab::Resume, "2:RES", 52),
                    (Tab::About, "3:ABT", 100),
                    (Tab::Contact, "4:CNT", 148),
                    (Tab::Help, "5:?", 196),
                ]
            };

            for &(tab, label, tx) in tabs {
                let active = self.current_tab == tab;
                let style = if active {
                    TextStyle::new(Color::from_palette(6))
                        .with_bg(Color::from_palette(7))
                        .bold()
                } else {
                    TextStyle::new(Color::from_palette(4))
                };
                view.add(Element::Text(TextElement {
                    x: tx,
                    y: 11,
                    text: label.to_string(),
                    style,
                }));
            }

            if self.platform == Platform::Web {
                let vis_style = if self.active_modal == ActiveModal::Visuals {
                    TextStyle::new(Color::from_palette(6)).with_bg(Color::from_palette(7)).bold()
                } else {
                    TextStyle::new(Color::from_palette(11))
                };
                view.add(Element::Link(LinkElement::new(
                    170,
                    11,
                    "VIS",
                    "#modal-visuals",
                    vis_style,
                )));

                let sys_style = if self.active_modal == ActiveModal::System {
                    TextStyle::new(Color::from_palette(6)).with_bg(Color::from_palette(7)).bold()
                } else {
                    TextStyle::new(Color::from_palette(9)).bold()
                };
                view.add(Element::Link(LinkElement::new(
                    202,
                    11,
                    "SYS",
                    "#modal-system",
                    sys_style,
                )));

                let hlp_style = if self.active_modal == ActiveModal::Help {
                    TextStyle::new(Color::from_palette(6)).with_bg(Color::from_palette(7)).bold()
                } else {
                    TextStyle::new(Color::from_palette(8))
                };
                view.add(Element::Link(LinkElement::new(
                    234,
                    11,
                    "?",
                    "#modal-help",
                    hlp_style,
                )));

                let sys_text = format!("[{}]", self.system_mode.short_name());
                let sys_x = width.saturating_sub((sys_text.len() as u16 + 1) * 8);
                view.add(Element::Link(LinkElement::new(
                    sys_x,
                    11,
                    sys_text,
                    "#system",
                    TextStyle::new(Color::from_palette(9)).bold(),
                )));
            } else {
                let sys_text = format!("[S]{}", self.system_mode.short_name());
                let sys_x = width.saturating_sub((sys_text.len() as u16 + 1) * 8);
                view.add(Element::Link(LinkElement::new(
                    sys_x,
                    11,
                    sys_text,
                    "#system",
                    TextStyle::new(Color::from_palette(9)).bold(),
                )));
            }

            view.add(Element::Rect(RectElement {
                x: 0,
                y: 20,
                width,
                height: 1,
                color: Color::from_palette(3),
                filled: true,
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

            // Top Bar Clock at top right
            let clock_str = self.clock_formatted();
            view.add(Element::Text(TextElement {
                x: width.saturating_sub(44),
                y: 1,
                text: clock_str,
                style: TextStyle::new(Color::from_palette(10)).bold(),
            }));

            view.add(Element::Rect(RectElement {
                x: 0,
                y: 10,
                width,
                height: 10,
                color: Color::from_palette(0),
                filled: true,
            }));

            let tabs: &[(Tab, &str, u16)] = if self.platform == Platform::Web {
                &[
                    (Tab::Projects, "1:PR", 0),
                    (Tab::Resume, "2:CV", 36),
                    (Tab::About, "3:AB", 72),
                    (Tab::Contact, "4:CT", 108),
                ]
            } else {
                &[
                    (Tab::Projects, "1:PR", 0),
                    (Tab::Resume, "2:CV", 44),
                    (Tab::About, "3:AB", 88),
                    (Tab::Contact, "4:CT", 132),
                    (Tab::Help, "5:?", 176),
                ]
            };

            for &(tab, label, tx) in tabs {
                let active = self.current_tab == tab;
                let style = if active {
                    TextStyle::new(Color::from_palette(6))
                        .with_bg(Color::from_palette(7))
                        .bold()
                } else {
                    TextStyle::new(Color::from_palette(4))
                };
                view.add(Element::Text(TextElement {
                    x: tx,
                    y: 11,
                    text: label.to_string(),
                    style,
                }));
            }

            if self.platform == Platform::Web {
                let vis_style = if self.active_modal == ActiveModal::Visuals {
                    TextStyle::new(Color::from_palette(6)).with_bg(Color::from_palette(7)).bold()
                } else {
                    TextStyle::new(Color::from_palette(11))
                };
                view.add(Element::Link(LinkElement::new(
                    144,
                    11,
                    "VIS",
                    "#modal-visuals",
                    vis_style,
                )));

                let sys_style = if self.active_modal == ActiveModal::System {
                    TextStyle::new(Color::from_palette(6)).with_bg(Color::from_palette(7)).bold()
                } else {
                    TextStyle::new(Color::from_palette(9)).bold()
                };
                view.add(Element::Link(LinkElement::new(
                    176,
                    11,
                    "SYS",
                    "#modal-system",
                    sys_style,
                )));

                let hlp_style = if self.active_modal == ActiveModal::Help {
                    TextStyle::new(Color::from_palette(6)).with_bg(Color::from_palette(7)).bold()
                } else {
                    TextStyle::new(Color::from_palette(8))
                };
                view.add(Element::Link(LinkElement::new(
                    208,
                    11,
                    "?",
                    "#modal-help",
                    hlp_style,
                )));
            } else {
                let sys_text = format!("[{}]", self.system_mode.short_name());
                let sys_x = 224;
                view.add(Element::Link(LinkElement::new(
                    sys_x,
                    11,
                    sys_text,
                    "#system",
                    TextStyle::new(Color::from_palette(9)).bold(),
                )));
            }

            view.add(Element::Rect(RectElement {
                x: 0,
                y: 20,
                width,
                height: 1,
                color: Color::from_palette(3),
                filled: true,
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

        // Row 1: Tab bar (y = 16)
        view.add(Element::Rect(RectElement {
            x: 0,
            y: 16,
            width,
            height: 16,
            color: Color::from_palette(0),
            filled: true,
        }));

        let tabs: &[(Tab, &str, u16)] = &[
            (Tab::Projects, "[1] Projects", 8),
            (Tab::Resume, "[2] Resume", 120),
            (Tab::About, "[3] About", 216),
            (Tab::Contact, "[4] Contact", 304),
            (Tab::Help, "[5] Help", 400),
        ];

        for &(tab, label, tx) in tabs {
            let active = self.current_tab == tab;
            let style = if active {
                TextStyle::new(Color::from_palette(6))
                    .with_bg(Color::from_palette(7))
                    .bold()
            } else {
                TextStyle::new(Color::from_palette(4))
            };
            view.add(Element::Text(TextElement {
                x: tx,
                y: 16,
                text: label.to_string(),
                style,
            }));
        }

        let sys_text = format!("[S] {}", self.system_mode.short_name());
        let sys_x = width.saturating_sub((sys_text.len() as u16 + 2) * 8);
        view.add(Element::Link(LinkElement::new(
            sys_x,
            16,
            sys_text,
            "#system",
            TextStyle::new(Color::from_palette(9)).bold(),
        )));

        // Row 2: Tab separator rule (y = 32)
        view.add(Element::Rect(RectElement {
            x: 0,
            y: 32,
            width,
            height: 1,
            color: Color::from_palette(7),
            filled: true,
        }));
    }
}
