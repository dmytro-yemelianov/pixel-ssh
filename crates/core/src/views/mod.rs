//! Top-level view rendering, navigation bar, clock, and status bar.

pub mod about;
pub mod contact;
pub mod help;
pub mod modals;
pub mod projects;
pub mod resume;
pub mod visuals;

use crate::state::{App, Tab};
use pixel_ssh_view::{
    horizontal_scroll, ActiveModal, Color, Cursor, Element, LinkElement, Platform, RectElement,
    TextElement, TextStyle, View,
};

impl App {
    pub fn render(&self) -> View {
        let (width, height) = self.view_dimensions();
        let (cols, _rows) = if self.platform == Platform::Terminal {
            (self.terminal_cols, self.terminal_rows)
        } else {
            (width / 8, height / self.resolution.line_height())
        };
        let mut view = View::new(width, height);
        view.resolution = self.resolution;
        view.color_palette = self.color_palette;
        view.interface_theme = self.interface_theme;
        view.font_mode = self.interface_theme.font_mode();
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
            let title_color = match (self.tick / 8) % 4 {
                0 => 7,
                1 => 15,
                2 => 14,
                _ => 7,
            };
            let title_w = ((width.saturating_sub(260)) / 8) as usize;
            let title = horizontal_scroll(
                "DMYTRO YEMELIANOV - SYSTEMS & AI ARCHITECT",
                title_w.max(20),
                self.tick,
            );
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

            let title_color = match (self.tick / 8) % 4 {
                0 => 7,
                1 => 15,
                2 => 14,
                _ => 7,
            };
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

            let title_color = match (self.tick / 8) % 4 {
                0 => 7,
                1 => 15,
                2 => 14,
                _ => 7,
            };
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

        // Active Modal dialog overlay (Visuals, Help)
        if self.active_modal != ActiveModal::None {
            match self.active_modal {
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
                format!(
                    "DIALOG ACTIVE: {:?} • Click [X] or press [ESC] to dismiss",
                    self.active_modal
                )
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
                "P/H/S/V/Q j/k:Nav"
            } else {
                "P/H/S/V/Q j/k:Scroll Enter"
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
            } else {
                "P/H/S/V/Q j/k:Nav"
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

        let title_color = match (self.tick / 8) % 4 {
            0 => 7,
            1 => 15,
            2 => 14,
            _ => 7,
        };
        let title_w = (cols as usize).saturating_sub(38).max(24);
        let title = horizontal_scroll(
            "DMYTRO YEMELIANOV - SYSTEMS & AI ARCHITECT",
            title_w,
            self.tick,
        );
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
    pub(crate) fn render_bottom_navigation(
        &self,
        view: &mut View,
        width: u16,
        height: u16,
        cols: u16,
    ) {
        let compact = cols < 80;
        let nav_y = if self.platform == Platform::Terminal {
            (self.terminal_rows.saturating_sub(2)) * 16
        } else if compact {
            height.saturating_sub(20)
        } else {
            height.saturating_sub(32)
        };
        let row_h = if compact { 10 } else { 16 };
        let items = [
            (
                if compact { "Prj" } else { "Projects" },
                self.current_tab == Tab::Projects
                    && !self.show_detail
                    && self.active_modal == ActiveModal::None,
            ),
            (
                if compact { "Hlp" } else { "Help" },
                self.active_modal == ActiveModal::Help || self.current_tab == Tab::Help,
            ),
            (if compact { "Sys" } else { "System" }, false),
            (
                if compact { "Vis" } else { "Visuals" },
                self.active_modal == ActiveModal::Visuals,
            ),
            (if compact { "Qut" } else { "Quit" }, false),
        ];
        view.add(Element::Rect(RectElement {
            x: 0,
            y: nav_y,
            width,
            height: row_h,
            color: Color::from_palette(13),
            filled: true,
        }));
        let slot_w = width / items.len() as u16;
        for (index, &(label, active)) in items.iter().enumerate() {
            let x = index as u16 * slot_w;
            let bg = if active {
                Color::from_palette(6)
            } else {
                Color::from_palette(1)
            };
            let fg = if active {
                Color::from_palette(0)
            } else {
                Color::from_palette(5)
            };
            view.add(Element::Rect(RectElement {
                x: x + 1,
                y: nav_y,
                width: slot_w.saturating_sub(2),
                height: row_h,
                color: bg,
                filled: true,
            }));
            let (hotkey, rest) = label.split_at(1);
            view.add(Element::Text(TextElement {
                x: x + 3,
                y: nav_y,
                text: hotkey.to_string(),
                style: TextStyle::new(Color::from_palette(if active { 10 } else { 12 }))
                    .bold()
                    .underline(),
            }));
            view.add(Element::Text(TextElement {
                x: x + 11,
                y: nav_y,
                text: rest.to_string(),
                style: TextStyle::new(fg).bold(),
            }));
        }
    }
}
