//! Event handling and user input processing for the application.

use crate::data::PROJECTS;
use crate::state::{App, Tab};
use crate::views::links::PROJECT_LINKS;
use pixel_ssh_view::{ActiveModal, InputEvent, Key, Platform, VisualEffects};

const PROJECT_LIST_PREFIX_ITEMS: usize = 3;

impl App {
    fn select_list_item(&mut self, item: usize) {
        self.selected_list_item = item.min(PROJECT_LIST_PREFIX_ITEMS + PROJECTS.len() - 1);
        if self.selected_list_item < PROJECT_LIST_PREFIX_ITEMS {
            self.scroll_offset = 0;
            return;
        }

        self.selected_project = self.selected_list_item - PROJECT_LIST_PREFIX_ITEMS;
        self.ensure_selected_project_visible();
    }

    fn activate_selected_list_item(&mut self) {
        match self.selected_list_item {
            0 => self.current_tab = Tab::Resume,
            1 => self.current_tab = Tab::Contact,
            2 => self.current_tab = Tab::About,
            _ => {
                self.selected_project = self.selected_list_item - PROJECT_LIST_PREFIX_ITEMS;
                self.show_detail = true;
                self.detail_scroll = 0;
                self.selected_detail_item = 0;
            }
        }
    }

    /// Updates the application state with a normalized input event.
    /// Returns `true` if the state changed and a re-render is required (`dirty`).
    pub fn update(&mut self, event: InputEvent) -> bool {
        match event {
            InputEvent::KeyDown(key) => {
                // If screensaver is active, ANY key press wakes and dismisses it
                if self.screensaver_active {
                    self.screensaver_active = false;
                    return true;
                }

                if matches!(key, Key::Char('s') | Key::Char('S')) {
                    self.set_resolution(self.resolution.next());
                    self.active_modal = ActiveModal::None;
                    return true;
                }

                // If active modal is open, modal captures all keys exclusively!
                if self.active_modal != ActiveModal::None {
                    match self.active_modal {
                        ActiveModal::Visuals => {
                            match key {
                                Key::Escape | Key::Char('v') | Key::Char('V') => {
                                    self.active_modal = ActiveModal::None;
                                    return true;
                                }
                                Key::Char('1') | Key::Char('a') | Key::Char('A') => {
                                    self.visual_effects = VisualEffects::clean();
                                    self.status = "Preset: Clean (Pixel-Perfect)".to_string();
                                    return true;
                                }
                                Key::Char('2') | Key::Char('w') | Key::Char('W') => {
                                    self.visual_effects = VisualEffects::crt_trinitron();
                                    self.status = "Preset: 80s Trinitron CRT".to_string();
                                    return true;
                                }
                                Key::Char('3') | Key::Char('c') | Key::Char('C') => {
                                    self.visual_effects = VisualEffects::crt_arcade();
                                    self.status = "Preset: Arcade Cabinet".to_string();
                                    return true;
                                }
                                Key::Char('4') | Key::Char('d') | Key::Char('D') => {
                                    self.visual_effects = VisualEffects::phosphor_bloom();
                                    self.status = "Preset: Phosphor Bloom".to_string();
                                    return true;
                                }
                                Key::Char('5') | Key::Char('e') | Key::Char('E') => {
                                    self.visual_effects = VisualEffects::retro_glitch();
                                    self.status = "Preset: Retro Glitch".to_string();
                                    return true;
                                }
                                Key::Char('6') | Key::Char('r') | Key::Char('R') => {
                                    self.visual_effects = VisualEffects::default();
                                    self.status = "Preset: Default CRT".to_string();
                                    return true;
                                }
                                Key::Up | Key::Char('k') => {
                                    self.selected_fx_slider = (self.selected_fx_slider + 8) % 9;
                                    return true;
                                }
                                Key::Down | Key::Char('j') => {
                                    self.selected_fx_slider = (self.selected_fx_slider + 1) % 9;
                                    return true;
                                }
                                Key::Left | Key::Char('h') => {
                                    self.adjust_selected_slider(-0.05);
                                    return true;
                                }
                                Key::Right | Key::Char('l') => {
                                    self.adjust_selected_slider(0.05);
                                    return true;
                                }
                                Key::Char('z') | Key::Char('Z') => {
                                    self.selected_fx_slider = 0;
                                    self.visual_effects.scanlines =
                                        if self.visual_effects.scanlines < 0.15 {
                                            0.35
                                        } else if self.visual_effects.scanlines < 0.50 {
                                            0.70
                                        } else {
                                            0.0
                                        };
                                    self.status = format!(
                                        "Scanlines: {:.0}%",
                                        self.visual_effects.scanlines * 100.0
                                    );
                                    return true;
                                }
                                Key::Char('x') | Key::Char('X') => {
                                    self.selected_fx_slider = 1;
                                    self.visual_effects.pixel_grid =
                                        if self.visual_effects.pixel_grid < 0.10 {
                                            0.20
                                        } else if self.visual_effects.pixel_grid < 0.30 {
                                            0.40
                                        } else {
                                            0.0
                                        };
                                    self.status = format!(
                                        "Pixel Grid: {:.0}%",
                                        self.visual_effects.pixel_grid * 100.0
                                    );
                                    return true;
                                }
                                Key::Char('m') | Key::Char('M') => {
                                    self.selected_fx_slider = 2;
                                    self.visual_effects.chromatic =
                                        if self.visual_effects.chromatic < 0.15 {
                                            0.35
                                        } else if self.visual_effects.chromatic < 0.55 {
                                            0.85
                                        } else {
                                            0.0
                                        };
                                    self.status = format!(
                                        "Chromatic: {:.0}%",
                                        self.visual_effects.chromatic * 100.0
                                    );
                                    return true;
                                }
                                Key::Char('b') | Key::Char('B') => {
                                    self.selected_fx_slider = 4;
                                    self.visual_effects.curvature =
                                        if self.visual_effects.curvature < 0.10 {
                                            0.25
                                        } else if self.visual_effects.curvature < 0.35 {
                                            0.45
                                        } else {
                                            0.0
                                        };
                                    self.status = format!(
                                        "CRT Curvature: {:.0}%",
                                        self.visual_effects.curvature * 100.0
                                    );
                                    return true;
                                }
                                Key::Char('n') | Key::Char('N') => {
                                    self.selected_fx_slider = 5;
                                    self.visual_effects.jitter =
                                        if self.visual_effects.jitter < 0.10 {
                                            0.25
                                        } else if self.visual_effects.jitter < 0.40 {
                                            0.65
                                        } else {
                                            0.0
                                        };
                                    self.status = format!(
                                        "Jitter: {:.0}%",
                                        self.visual_effects.jitter * 100.0
                                    );
                                    return true;
                                }
                                Key::Char('g') | Key::Char('G') => {
                                    self.selected_fx_slider = 6;
                                    self.visual_effects.magnet =
                                        if self.visual_effects.magnet < 0.05 {
                                            0.10
                                        } else if self.visual_effects.magnet < 0.18 {
                                            0.25
                                        } else {
                                            0.0
                                        };
                                    self.status = format!(
                                        "Point CRT Magnet Deflection: {:.0}%",
                                        self.visual_effects.magnet * 100.0
                                    );
                                    return true;
                                }
                                _ => {}
                            }
                            return true; // Modal captures and swallows all other keys
                        }
                        ActiveModal::Help => {
                            match key {
                                Key::Escape
                                | Key::Enter
                                | Key::Char('?')
                                | Key::Char('h')
                                | Key::Char('H')
                                | Key::Char('q')
                                | Key::Char('Q') => {
                                    self.active_modal = ActiveModal::None;
                                    return true;
                                }
                                _ => {}
                            }
                            return true; // Modal captures and swallows all other keys
                        }
                        ActiveModal::None => unreachable!(),
                    }
                }

                if matches!(key, Key::Char('l') | Key::Char('L'))
                    && !(self.current_tab == Tab::Projects && self.show_detail)
                {
                    self.current_tab = Tab::Links;
                    self.show_detail = false;
                    self.select_link(self.selected_link);
                    return true;
                }

                if self.current_tab == Tab::Links {
                    let selected = match key {
                        Key::Up | Key::Char('k') => Some(self.selected_link.saturating_sub(1)),
                        Key::Down | Key::Char('j') => Some(self.selected_link + 1),
                        Key::Tab => Some((self.selected_link + 1) % PROJECT_LINKS.len()),
                        Key::Home => Some(0),
                        Key::End => Some(PROJECT_LINKS.len() - 1),
                        Key::PageUp => {
                            Some(self.selected_link.saturating_sub(self.links_geometry().2))
                        }
                        Key::PageDown => Some(self.selected_link + self.links_geometry().2),
                        Key::Enter => {
                            self.link_activation =
                                Some(PROJECT_LINKS[self.selected_link].1.to_owned());
                            self.status = format!("Open: {}", PROJECT_LINKS[self.selected_link].1);
                            return true;
                        }
                        Key::Escape | Key::Char('q') | Key::Char('Q') => {
                            self.current_tab = Tab::Projects;
                            return true;
                        }
                        _ => None,
                    };
                    if let Some(selected) = selected {
                        self.select_link(selected);
                        return true;
                    }
                }

                // If on Web with an article overlay open (Tab::Resume or Tab::About):
                if self.platform == Platform::Web
                    && (self.current_tab == Tab::Resume || self.current_tab == Tab::About)
                {
                    match key {
                        Key::Escape
                        | Key::Char('q')
                        | Key::Char('Q')
                        | Key::Char('p')
                        | Key::Char('P') => {
                            self.current_tab = Tab::Projects;
                            return true;
                        }
                        Key::Char('h') | Key::Char('H') | Key::Char('?') => {
                            self.active_modal = ActiveModal::Help;
                            return true;
                        }
                        Key::Char('v') | Key::Char('V') => {
                            self.active_modal = ActiveModal::Visuals;
                            return true;
                        }

                        Key::Tab => {
                            self.current_tab = match self.current_tab {
                                Tab::Resume => Tab::About,
                                Tab::About => Tab::Contact,
                                _ => Tab::Projects,
                            };
                            return true;
                        }
                        Key::Down | Key::Char('j') => {
                            if self.current_tab == Tab::Resume {
                                self.resume_scroll =
                                    (self.resume_scroll + 1).min(self.resume_max_scroll());
                            } else {
                                self.about_scroll =
                                    (self.about_scroll + 1).min(self.about_max_scroll());
                            }
                            return true;
                        }
                        Key::Up | Key::Char('k') => {
                            if self.current_tab == Tab::Resume {
                                self.resume_scroll = self.resume_scroll.saturating_sub(1);
                            } else {
                                self.about_scroll = self.about_scroll.saturating_sub(1);
                            }
                            return true;
                        }
                        Key::PageDown | Key::Char(' ') => {
                            let step = 10;
                            if self.current_tab == Tab::Resume {
                                self.resume_scroll =
                                    (self.resume_scroll + step).min(self.resume_max_scroll());
                            } else {
                                self.about_scroll =
                                    (self.about_scroll + step).min(self.about_max_scroll());
                            }
                            return true;
                        }
                        Key::PageUp => {
                            let step = 10;
                            if self.current_tab == Tab::Resume {
                                self.resume_scroll = self.resume_scroll.saturating_sub(step);
                            } else {
                                self.about_scroll = self.about_scroll.saturating_sub(step);
                            }
                            return true;
                        }
                        Key::Home => {
                            if self.current_tab == Tab::Resume {
                                self.resume_scroll = 0;
                            } else {
                                self.about_scroll = 0;
                            }
                            return true;
                        }
                        Key::End => {
                            if self.current_tab == Tab::Resume {
                                self.resume_scroll = self.resume_max_scroll();
                            } else {
                                self.about_scroll = self.about_max_scroll();
                            }
                            return true;
                        }
                        _ => {
                            // Any other key while article is open is swallowed
                            return true;
                        }
                    }
                }

                match key {
                    // Workstation controls use their visible first letters.
                    // Digits have no global navigation assignment.
                    Key::Char('p') | Key::Char('P') => {
                        self.current_tab = Tab::Projects;
                        self.show_detail = false;
                        self.active_modal = ActiveModal::None;
                        true
                    }
                    Key::Char('H') | Key::Char('?') | Key::Char('h')
                        if !(self.current_tab == Tab::Projects && self.show_detail) =>
                    {
                        if self.platform == Platform::Web {
                            self.active_modal = if self.active_modal == ActiveModal::Help {
                                ActiveModal::None
                            } else {
                                ActiveModal::Help
                            };
                        } else {
                            self.current_tab = Tab::Help;
                            self.show_detail = false;
                        }
                        true
                    }
                    Key::Char('v') | Key::Char('V') if self.platform == Platform::Web => {
                        self.active_modal = if self.active_modal == ActiveModal::Visuals {
                            ActiveModal::None
                        } else {
                            ActiveModal::Visuals
                        };
                        true
                    }

                    // Tab key: In detail view, cycles interactive items; otherwise cycles views
                    Key::Tab => {
                        if self.current_tab == Tab::Projects && self.show_detail {
                            self.selected_detail_item =
                                (self.selected_detail_item + 1) % self.detail_item_count();
                            true
                        } else {
                            self.current_tab = match (self.platform, self.current_tab) {
                                (Platform::Web, Tab::Projects) => Tab::Resume,
                                (Platform::Web, Tab::Resume) => Tab::About,
                                (Platform::Web, Tab::About) => Tab::Contact,
                                (Platform::Web, Tab::Contact) => Tab::Links,
                                (Platform::Web, _) => Tab::Projects,

                                (Platform::Terminal, Tab::Projects) => Tab::Resume,
                                (Platform::Terminal, Tab::Resume) => Tab::About,
                                (Platform::Terminal, Tab::About) => Tab::Contact,
                                (Platform::Terminal, Tab::Visuals) => Tab::Contact,
                                (Platform::Terminal, Tab::Contact) => Tab::Help,
                                (Platform::Terminal, Tab::Help) => Tab::Projects,
                                (Platform::Terminal, Tab::Links) => Tab::Projects,
                            };
                            self.show_detail = false;
                            true
                        }
                    }

                    // Project Detail Left/Right navigation ('h' / 'l' or Left / Right)
                    Key::Char('h') | Key::Left
                        if self.current_tab == Tab::Projects && self.show_detail =>
                    {
                        if self.selected_project > 0 {
                            self.selected_project -= 1;
                        } else {
                            self.selected_project = PROJECTS.len().saturating_sub(1);
                        }
                        self.selected_list_item = self.selected_project + PROJECT_LIST_PREFIX_ITEMS;
                        self.detail_scroll = 0;
                        self.selected_detail_item = 0;
                        true
                    }
                    Key::Char('l') | Key::Right
                        if self.current_tab == Tab::Projects && self.show_detail =>
                    {
                        self.selected_project = (self.selected_project + 1) % PROJECTS.len();
                        self.selected_list_item = self.selected_project + PROJECT_LIST_PREFIX_ITEMS;
                        self.detail_scroll = 0;
                        self.selected_detail_item = 0;
                        true
                    }

                    // Visuals preset shortcuts (Web only)
                    Key::Char('a') | Key::Char('A')
                        if self.platform == Platform::Web && self.current_tab == Tab::Visuals =>
                    {
                        self.visual_effects = VisualEffects::clean();
                        self.status = "Preset: Clean (Pixel-Perfect)".to_string();
                        true
                    }
                    Key::Char('w') | Key::Char('W')
                        if self.platform == Platform::Web && self.current_tab == Tab::Visuals =>
                    {
                        self.visual_effects = VisualEffects::crt_trinitron();
                        self.status = "Preset: 80s Trinitron CRT".to_string();
                        true
                    }
                    Key::Char('c') | Key::Char('C')
                        if self.platform == Platform::Web && self.current_tab == Tab::Visuals =>
                    {
                        self.visual_effects = VisualEffects::crt_arcade();
                        self.status = "Preset: Arcade Cabinet".to_string();
                        true
                    }
                    Key::Char('d') | Key::Char('D')
                        if self.platform == Platform::Web && self.current_tab == Tab::Visuals =>
                    {
                        self.visual_effects = VisualEffects::phosphor_bloom();
                        self.status = "Preset: Phosphor Bloom".to_string();
                        true
                    }
                    Key::Char('e') | Key::Char('E')
                        if self.platform == Platform::Web && self.current_tab == Tab::Visuals =>
                    {
                        self.visual_effects = VisualEffects::retro_glitch();
                        self.status = "Preset: Retro Glitch".to_string();
                        true
                    }
                    Key::Char('r') | Key::Char('R')
                        if self.platform == Platform::Web && self.current_tab == Tab::Visuals =>
                    {
                        self.visual_effects = VisualEffects::default();
                        self.status = "Preset: Default CRT".to_string();
                        true
                    }

                    // Direct Visuals FX slider toggles (Keys Z, X, M, V, B, N, G)
                    Key::Char('z') | Key::Char('Z')
                        if self.platform == Platform::Web && self.current_tab == Tab::Visuals =>
                    {
                        self.selected_fx_slider = 0;
                        self.visual_effects.scanlines = if self.visual_effects.scanlines < 0.15 {
                            0.35
                        } else if self.visual_effects.scanlines < 0.50 {
                            0.70
                        } else {
                            0.0
                        };
                        self.status =
                            format!("Scanlines: {:.0}%", self.visual_effects.scanlines * 100.0);
                        true
                    }
                    Key::Char('x') | Key::Char('X')
                        if self.platform == Platform::Web && self.current_tab == Tab::Visuals =>
                    {
                        self.selected_fx_slider = 1;
                        self.visual_effects.pixel_grid = if self.visual_effects.pixel_grid < 0.10 {
                            0.20
                        } else if self.visual_effects.pixel_grid < 0.30 {
                            0.40
                        } else {
                            0.0
                        };
                        self.status =
                            format!("Pixel Grid: {:.0}%", self.visual_effects.pixel_grid * 100.0);
                        true
                    }
                    Key::Char('m') | Key::Char('M')
                        if self.platform == Platform::Web && self.current_tab == Tab::Visuals =>
                    {
                        self.selected_fx_slider = 2;
                        self.visual_effects.chromatic = if self.visual_effects.chromatic < 0.15 {
                            0.35
                        } else if self.visual_effects.chromatic < 0.55 {
                            0.85
                        } else {
                            0.0
                        };
                        self.status =
                            format!("Chromatic: {:.0}%", self.visual_effects.chromatic * 100.0);
                        true
                    }
                    Key::Char('v') | Key::Char('V')
                        if self.platform == Platform::Web && self.current_tab == Tab::Visuals =>
                    {
                        self.selected_fx_slider = 3;
                        self.visual_effects.afterglow = if self.visual_effects.afterglow < 0.15 {
                            0.30
                        } else if self.visual_effects.afterglow < 0.55 {
                            0.80
                        } else {
                            0.0
                        };
                        self.status =
                            format!("Afterglow: {:.0}%", self.visual_effects.afterglow * 100.0);
                        true
                    }
                    Key::Char('b') | Key::Char('B')
                        if self.platform == Platform::Web && self.current_tab == Tab::Visuals =>
                    {
                        self.selected_fx_slider = 4;
                        self.visual_effects.curvature = if self.visual_effects.curvature < 0.10 {
                            0.25
                        } else if self.visual_effects.curvature < 0.35 {
                            0.45
                        } else {
                            0.0
                        };
                        self.status = format!(
                            "CRT Curvature: {:.0}%",
                            self.visual_effects.curvature * 100.0
                        );
                        true
                    }
                    Key::Char('n') | Key::Char('N')
                        if self.platform == Platform::Web && self.current_tab == Tab::Visuals =>
                    {
                        self.selected_fx_slider = 5;
                        self.visual_effects.jitter = if self.visual_effects.jitter < 0.10 {
                            0.25
                        } else if self.visual_effects.jitter < 0.40 {
                            0.65
                        } else {
                            0.0
                        };
                        self.status = format!("Jitter: {:.0}%", self.visual_effects.jitter * 100.0);
                        true
                    }
                    Key::Char('g') | Key::Char('G')
                        if self.platform == Platform::Web && self.current_tab == Tab::Visuals =>
                    {
                        self.selected_fx_slider = 6;
                        self.visual_effects.magnet = if self.visual_effects.magnet < 0.05 {
                            0.10
                        } else if self.visual_effects.magnet < 0.18 {
                            0.25
                        } else {
                            0.0
                        };
                        self.status = format!(
                            "Point CRT Magnet Deflection: {:.0}%",
                            self.visual_effects.magnet * 100.0
                        );
                        true
                    }
                    Key::Char('h')
                        if self.platform == Platform::Web && self.current_tab == Tab::Visuals =>
                    {
                        self.selected_fx_slider = 7;
                        self.visual_effects.antenna_hum = if self.visual_effects.antenna_hum < 0.10
                        {
                            0.25
                        } else if self.visual_effects.antenna_hum < 0.40 {
                            0.60
                        } else {
                            0.0
                        };
                        self.status = format!(
                            "Sporadic Antenna Hum: {:.0}%",
                            self.visual_effects.antenna_hum * 100.0
                        );
                        true
                    }
                    Key::Char('y') | Key::Char('Y')
                        if self.platform == Platform::Web && self.current_tab == Tab::Visuals =>
                    {
                        self.selected_fx_slider = 8;
                        self.visual_effects.noise = if self.visual_effects.noise < 0.10 {
                            0.25
                        } else if self.visual_effects.noise < 0.40 {
                            0.60
                        } else {
                            0.0
                        };
                        self.status = format!(
                            "White Noise Snow: {:.0}%",
                            self.visual_effects.noise * 100.0
                        );
                        true
                    }

                    // Fine slider tuning with Left/Right or -/+ when on Visuals Tab
                    Key::Left | Key::Char('-')
                        if self.platform == Platform::Web && self.current_tab == Tab::Visuals =>
                    {
                        match self.selected_fx_slider {
                            0 => {
                                self.visual_effects.scanlines =
                                    (self.visual_effects.scanlines - 0.05).max(0.0)
                            }
                            1 => {
                                self.visual_effects.pixel_grid =
                                    (self.visual_effects.pixel_grid - 0.05).max(0.0)
                            }
                            2 => {
                                self.visual_effects.chromatic =
                                    (self.visual_effects.chromatic - 0.05).max(0.0)
                            }
                            3 => {
                                self.visual_effects.afterglow =
                                    (self.visual_effects.afterglow - 0.05).max(0.0)
                            }
                            4 => {
                                self.visual_effects.curvature =
                                    (self.visual_effects.curvature - 0.05).max(0.0)
                            }
                            5 => {
                                self.visual_effects.jitter =
                                    (self.visual_effects.jitter - 0.05).max(0.0)
                            }
                            6 => {
                                self.visual_effects.magnet =
                                    (self.visual_effects.magnet - 0.05).max(0.0)
                            }
                            7 => {
                                self.visual_effects.antenna_hum =
                                    (self.visual_effects.antenna_hum - 0.05).max(0.0)
                            }
                            _ => {
                                self.visual_effects.noise =
                                    (self.visual_effects.noise - 0.05).max(0.0)
                            }
                        }
                        true
                    }
                    Key::Right | Key::Char('+') | Key::Char('=')
                        if self.platform == Platform::Web && self.current_tab == Tab::Visuals =>
                    {
                        match self.selected_fx_slider {
                            0 => {
                                self.visual_effects.scanlines =
                                    (self.visual_effects.scanlines + 0.05).min(1.0)
                            }
                            1 => {
                                self.visual_effects.pixel_grid =
                                    (self.visual_effects.pixel_grid + 0.05).min(1.0)
                            }
                            2 => {
                                self.visual_effects.chromatic =
                                    (self.visual_effects.chromatic + 0.05).min(1.0)
                            }
                            3 => {
                                self.visual_effects.afterglow =
                                    (self.visual_effects.afterglow + 0.05).min(1.0)
                            }
                            4 => {
                                self.visual_effects.curvature =
                                    (self.visual_effects.curvature + 0.05).min(1.0)
                            }
                            5 => {
                                self.visual_effects.jitter =
                                    (self.visual_effects.jitter + 0.05).min(1.0)
                            }
                            6 => {
                                self.visual_effects.magnet =
                                    (self.visual_effects.magnet + 0.05).min(1.0)
                            }
                            7 => {
                                self.visual_effects.antenna_hum =
                                    (self.visual_effects.antenna_hum + 0.05).min(1.0)
                            }
                            _ => {
                                self.visual_effects.noise =
                                    (self.visual_effects.noise + 0.05).min(1.0)
                            }
                        }
                        true
                    }

                    Key::Up | Key::Char('k') => {
                        if self.platform == Platform::Web && self.current_tab == Tab::Visuals {
                            if self.selected_fx_slider > 0 {
                                self.selected_fx_slider -= 1;
                                return true;
                            }
                        } else if self.current_tab == Tab::Resume {
                            if self.resume_scroll > 0 {
                                self.resume_scroll -= 1;
                                return true;
                            }
                        } else if self.current_tab == Tab::About {
                            if self.about_scroll > 0 {
                                self.about_scroll -= 1;
                                return true;
                            }
                        } else if self.current_tab == Tab::Projects {
                            if self.show_detail {
                                if self.detail_scroll > 0 {
                                    self.detail_scroll -= 1;
                                    return true;
                                } else {
                                    self.selected_detail_item =
                                        self.selected_detail_item.saturating_sub(1);
                                    return true;
                                }
                            } else if self.selected_list_item > 0 {
                                self.select_list_item(self.selected_list_item - 1);
                                return true;
                            }
                        }
                        false
                    }
                    Key::Down | Key::Char('j') => {
                        if self.platform == Platform::Web && self.current_tab == Tab::Visuals {
                            if self.selected_fx_slider < 8 {
                                self.selected_fx_slider += 1;
                                return true;
                            }
                        } else if self.current_tab == Tab::Resume {
                            if self.resume_scroll < self.resume_max_scroll() {
                                self.resume_scroll += 1;
                                return true;
                            }
                        } else if self.current_tab == Tab::About {
                            if self.about_scroll < self.about_max_scroll() {
                                self.about_scroll += 1;
                                return true;
                            }
                        } else if self.current_tab == Tab::Projects {
                            if self.show_detail {
                                if self.detail_scroll < self.detail_max_scroll() {
                                    self.detail_scroll += 1;
                                    return true;
                                } else {
                                    self.selected_detail_item = (self.selected_detail_item + 1)
                                        .min(self.detail_item_count() - 1);
                                    return true;
                                }
                            } else if self.selected_list_item + 1
                                < PROJECT_LIST_PREFIX_ITEMS + PROJECTS.len()
                            {
                                self.select_list_item(self.selected_list_item + 1);
                                return true;
                            }
                        }
                        false
                    }
                    Key::PageUp => {
                        if self.current_tab == Tab::Resume {
                            self.resume_scroll = self.resume_scroll.saturating_sub(6);
                            return true;
                        } else if self.current_tab == Tab::About {
                            self.about_scroll = self.about_scroll.saturating_sub(6);
                            return true;
                        } else if self.current_tab == Tab::Projects && self.show_detail {
                            self.detail_scroll = self.detail_scroll.saturating_sub(6);
                            return true;
                        }
                        false
                    }
                    Key::PageDown => {
                        if self.current_tab == Tab::Resume {
                            self.resume_scroll =
                                (self.resume_scroll + 6).min(self.resume_max_scroll());
                            return true;
                        } else if self.current_tab == Tab::About {
                            self.about_scroll =
                                (self.about_scroll + 6).min(self.about_max_scroll());
                            return true;
                        } else if self.current_tab == Tab::Projects && self.show_detail {
                            self.detail_scroll =
                                (self.detail_scroll + 6).min(self.detail_max_scroll());
                            return true;
                        }
                        false
                    }
                    Key::Home => {
                        if self.current_tab == Tab::Resume {
                            self.resume_scroll = 0;
                            return true;
                        } else if self.current_tab == Tab::About {
                            self.about_scroll = 0;
                            return true;
                        } else if self.current_tab == Tab::Projects {
                            if self.show_detail {
                                self.detail_scroll = 0;
                            } else {
                                self.select_list_item(0);
                            }
                            return true;
                        }
                        false
                    }
                    Key::End => {
                        if self.current_tab == Tab::Resume {
                            self.resume_scroll = self.resume_max_scroll();
                            return true;
                        } else if self.current_tab == Tab::About {
                            self.about_scroll = self.about_max_scroll();
                            return true;
                        } else if self.current_tab == Tab::Projects {
                            if self.show_detail {
                                self.detail_scroll = self.detail_max_scroll();
                            } else {
                                self.select_list_item(
                                    PROJECT_LIST_PREFIX_ITEMS + PROJECTS.len() - 1,
                                );
                            }
                            return true;
                        }
                        false
                    }
                    Key::Enter => {
                        if self.current_tab == Tab::Projects {
                            if self.show_detail {
                                if self.selected_detail_item == self.detail_item_count() - 1 {
                                    self.show_detail = false;
                                    self.detail_scroll = 0;
                                } else {
                                    let project = &PROJECTS[self.selected_project];
                                    self.link_activation = if self.selected_detail_item == 0 {
                                        Some(format!(
                                            "https://github.com/dmytro-yemelianov/{}",
                                            project.slug
                                        ))
                                    } else {
                                        project.demo.map(str::to_owned)
                                    };
                                }
                            } else {
                                self.activate_selected_list_item();
                            }
                            true
                        } else {
                            false
                        }
                    }
                    Key::Escape | Key::Char('q') | Key::Char('Q') => {
                        if self.active_modal != ActiveModal::None {
                            self.active_modal = ActiveModal::None;
                            true
                        } else if self.current_tab == Tab::Resume || self.current_tab == Tab::About
                        {
                            self.current_tab = Tab::Projects;
                            true
                        } else if self.show_detail {
                            self.show_detail = false;
                            self.detail_scroll = 0;
                            true
                        } else if self.current_tab == Tab::Help {
                            self.current_tab = Tab::Projects;
                            true
                        } else {
                            self.screensaver_active = true;
                            true
                        }
                    }
                    _ => false,
                }
            }
            InputEvent::TouchDrag { dy } => {
                let list = (self.current_tab == Tab::Links
                    || (self.current_tab == Tab::Projects && !self.show_detail))
                    && self.active_modal == ActiveModal::None;
                // TouchDrag uses the wheel convention: positive means finger up.
                // Lists move the selector with the finger; articles scroll content.
                self.update(InputEvent::Wheel {
                    dx: 0,
                    dy: if list { dy.saturating_neg() } else { dy },
                })
            }
            InputEvent::Wheel { dy, .. } => {
                // If modal is active, swallow wheel event so it never leaks into background
                if self.active_modal != ActiveModal::None {
                    return true;
                }

                let delta = (dy.unsigned_abs() as usize).max(1);
                if self.current_tab == Tab::Links {
                    let selected = if dy > 0 {
                        self.selected_link.saturating_add(delta)
                    } else if dy < 0 {
                        self.selected_link.saturating_sub(delta)
                    } else {
                        return false;
                    };
                    self.select_link(selected);
                    return true;
                }
                if self.current_tab == Tab::Resume {
                    if dy > 0 {
                        self.resume_scroll =
                            (self.resume_scroll + delta).min(self.resume_max_scroll());
                    } else if dy < 0 {
                        self.resume_scroll = self.resume_scroll.saturating_sub(delta);
                    }
                    return true;
                } else if self.current_tab == Tab::About {
                    if dy > 0 {
                        self.about_scroll =
                            (self.about_scroll + delta).min(self.about_max_scroll());
                    } else if dy < 0 {
                        self.about_scroll = self.about_scroll.saturating_sub(delta);
                    }
                    return true;
                } else if self.current_tab == Tab::Projects {
                    if self.show_detail {
                        if dy > 0 {
                            self.detail_scroll =
                                (self.detail_scroll + delta).min(self.detail_max_scroll());
                        } else if dy < 0 {
                            self.detail_scroll = self.detail_scroll.saturating_sub(delta);
                        }
                        return true;
                    } else if dy > 0
                        && self.selected_list_item + 1 < PROJECT_LIST_PREFIX_ITEMS + PROJECTS.len()
                    {
                        self.select_list_item(
                            (self.selected_list_item + delta)
                                .min(PROJECT_LIST_PREFIX_ITEMS + PROJECTS.len() - 1),
                        );
                        return true;
                    } else if dy < 0 && self.selected_list_item > 0 {
                        self.select_list_item(self.selected_list_item.saturating_sub(delta));
                        return true;
                    }
                }
                false
            }
            InputEvent::PointerDown { x, y, .. } => {
                self.mouse_pos = Some((x, y));
                let (cols, _rows) = self.resolution.char_grid();
                let (width, height) = self.view_dimensions();

                // 0. If screensaver active, any click wakes and dismisses it
                if self.screensaver_active {
                    self.screensaver_active = false;
                    return true;
                }

                // 1. Hot corner clicked: launches CRT screensaver
                if x >= width.saturating_sub(64) && y <= 24 {
                    self.screensaver_active = true;
                    return true;
                }

                // 1.5. Single global bottom navigation bar clicks (interacts across all views and modals)
                let (nav_y, nav_h) = self.navigation_geometry();
                let fkey_y_range = nav_y..nav_y + nav_h;

                if fkey_y_range.contains(&y) {
                    let slot_count = 6;
                    let slot_w = width / slot_count;
                    let slot = ((x / slot_w) as usize).min(slot_count as usize - 1);
                    match slot {
                        0 => {
                            self.current_tab = Tab::Projects;
                            self.show_detail = false;
                            self.active_modal = ActiveModal::None;
                            return true;
                        }
                        1 => {
                            self.active_modal = ActiveModal::Help;
                            return true;
                        }
                        2 => {
                            self.set_resolution(self.resolution.next());
                            self.active_modal = ActiveModal::None;
                            return true;
                        }
                        3 => {
                            self.active_modal = ActiveModal::Visuals;
                            return true;
                        }
                        4 => {
                            if self.active_modal != ActiveModal::None {
                                self.active_modal = ActiveModal::None;
                            } else if self.current_tab != Tab::Projects || self.show_detail {
                                self.current_tab = Tab::Projects;
                                self.show_detail = false;
                            } else {
                                self.screensaver_active = true;
                            }
                            return true;
                        }
                        5 => {
                            self.current_tab = Tab::Links;
                            self.show_detail = false;
                            self.active_modal = ActiveModal::None;
                            self.select_link(self.selected_link);
                            return true;
                        }
                        _ => {}
                    }
                }

                // 2. Active Modal Dialog click handling: strictly captures all mouse events
                if self.active_modal != ActiveModal::None {
                    let char_h: u16 = self.resolution.line_height();
                    let geometry = self.standard_dialog_geometry();
                    let (box_x, box_y, box_w, box_h) = (
                        geometry.box_x,
                        geometry.box_y,
                        geometry.box_w,
                        geometry.box_h,
                    );

                    // Clicking outside dialog box dismisses it
                    if x < box_x || x >= box_x + box_w || y < box_y || y >= box_y + box_h {
                        self.active_modal = ActiveModal::None;
                        return true;
                    }

                    // Clicking [X] close button
                    let close_x = (box_x + box_w).saturating_sub(48);
                    if y >= box_y && y < box_y + char_h + 2 && x >= close_x {
                        self.active_modal = ActiveModal::None;
                        return true;
                    }

                    // Visuals modal presets & sliders
                    if self.active_modal == ActiveModal::Visuals {
                        if cols < 80 && y >= box_y + 12 * char_h && y < box_y + 13 * char_h {
                            let preset = ((x.saturating_sub(box_x + 8)) / 40).min(4);
                            self.visual_effects = match preset {
                                0 => VisualEffects::clean(),
                                1 => VisualEffects::crt_trinitron(),
                                2 => VisualEffects::crt_arcade(),
                                3 => VisualEffects::phosphor_bloom(),
                                _ => VisualEffects::retro_glitch(),
                            };
                            self.status = format!("Preset: {}", self.visual_effects.preset_name());
                            return true;
                        }
                        let preset_y = box_y + char_h;
                        if cols >= 80 && y >= preset_y && y < preset_y + char_h {
                            if x >= box_x + 104 && x < box_x + 160 {
                                self.visual_effects = VisualEffects::clean();
                                self.status = "Preset: Clean".to_string();
                                return true;
                            } else if x >= box_x + 176 && x < box_x + 264 {
                                self.visual_effects = VisualEffects::crt_trinitron();
                                self.status = "Preset: 80s Trinitron".to_string();
                                return true;
                            } else if x >= box_x + 280 && x < box_x + 352 {
                                self.visual_effects = VisualEffects::crt_arcade();
                                self.status = "Preset: Arcade Cabinet".to_string();
                                return true;
                            } else if x >= box_x + 368 && x < box_x + 432 {
                                self.visual_effects = VisualEffects::phosphor_bloom();
                                self.status = "Preset: Phosphor Bloom".to_string();
                                return true;
                            } else if x >= box_x + 440 && x < box_x + 512 {
                                self.visual_effects = VisualEffects::retro_glitch();
                                self.status = "Preset: Retro Glitch".to_string();
                                return true;
                            }
                        }

                        let slider_start_y = box_y + if cols < 80 { 2 } else { 3 } * char_h;
                        if y >= slider_start_y && y < slider_start_y + 9 * char_h {
                            let slider_idx = ((y - slider_start_y) / char_h) as usize;
                            if slider_idx < 9 {
                                self.selected_fx_slider = slider_idx;
                                let track_start = if cols < 80 { box_x + 64 } else { box_x + 144 };
                                let track_w = if cols == 32 {
                                    56
                                } else if cols == 40 {
                                    80
                                } else {
                                    112
                                };
                                if x >= track_start && x <= track_start + track_w {
                                    let ratio =
                                        ((x - track_start) as f32 / track_w as f32).clamp(0.0, 1.0);
                                    match slider_idx {
                                        0 => self.visual_effects.scanlines = ratio,
                                        1 => self.visual_effects.pixel_grid = ratio,
                                        2 => self.visual_effects.chromatic = ratio,
                                        3 => self.visual_effects.afterglow = ratio,
                                        4 => self.visual_effects.curvature = ratio,
                                        5 => self.visual_effects.jitter = ratio,
                                        6 => self.visual_effects.magnet = ratio,
                                        7 => self.visual_effects.antenna_hum = ratio,
                                        8 => self.visual_effects.noise = ratio,
                                        _ => unreachable!(),
                                    }
                                }
                                return true;
                            }
                        }
                        return true; // Any other click inside visuals modal is swallowed
                    }

                    // Help modal: any click inside consumed
                    if self.active_modal == ActiveModal::Help {
                        return true;
                    }

                    return true;
                }

                // 3. Article view (CV or About) on Web: strictly captures mouse clicks
                if self.platform == Platform::Web
                    && (self.current_tab == Tab::Resume || self.current_tab == Tab::About)
                {
                    let char_h = self.resolution.line_height();
                    let pad_x = if cols >= 80 {
                        16u16
                    } else if cols >= 40 {
                        8u16
                    } else {
                        0u16
                    };
                    let pad_top = char_h * 2;
                    let pad_bot = char_h * 3;
                    let box_x = pad_x;
                    let box_y = pad_top;
                    let box_w = width.saturating_sub(pad_x * 2);
                    let box_h = ((height.saturating_sub(pad_top + pad_bot)) / char_h) * char_h;

                    let close_btn_w = if cols >= 80 { 64u16 } else { 24u16 };
                    let close_x = (box_x + box_w).saturating_sub(close_btn_w + 8);
                    if x < box_x
                        || x >= box_x + box_w
                        || y < box_y
                        || y >= box_y + box_h
                        || (y < box_y + char_h && x >= close_x)
                    {
                        self.current_tab = Tab::Projects;
                        return true;
                    }

                    // Scrollbar click on the right edge
                    if x >= (box_x + box_w).saturating_sub(24) {
                        if y < box_y + box_h / 2 {
                            if self.current_tab == Tab::Resume {
                                self.resume_scroll = self.resume_scroll.saturating_sub(5);
                            } else {
                                self.about_scroll = self.about_scroll.saturating_sub(5);
                            }
                        } else if self.current_tab == Tab::Resume {
                            self.resume_scroll =
                                (self.resume_scroll + 5).min(self.resume_max_scroll());
                        } else {
                            self.about_scroll =
                                (self.about_scroll + 5).min(self.about_max_scroll());
                        }
                    }

                    // STRICT CAPTURE: Consume all other clicks inside article overlay without leaking to background!
                    return true;
                }

                // 2. Resume, About & Project Detail scrollbar interaction
                if self.current_tab == Tab::Resume
                    || self.current_tab == Tab::About
                    || (self.current_tab == Tab::Projects && self.show_detail)
                {
                    let max_scroll = if self.current_tab == Tab::Resume {
                        self.resume_max_scroll()
                    } else if self.current_tab == Tab::About {
                        self.about_max_scroll()
                    } else {
                        self.detail_max_scroll()
                    };
                    let scrollbar_x_start = width.saturating_sub(20);
                    if x >= scrollbar_x_start {
                        let is_detail_wide =
                            self.current_tab == Tab::Projects && self.show_detail && cols >= 80;
                        let (top_arrow_y, bot_arrow_y, track_top, track_bot) = if is_detail_wide {
                            let lh = self.resolution.line_height();
                            let top = 8 * lh;
                            let bottom = (8 + self.detail_wide_visible_rows() as u16 - 1) * lh;
                            (
                                top..=top + lh - 1,
                                bottom..=bottom + lh - 1,
                                (top + lh) as f32,
                                bottom as f32,
                            )
                        } else if cols == 80 && self.current_tab == Tab::Resume {
                            (96..=112, 304..=320, 112.0, 304.0)
                        } else if cols == 80 && self.current_tab == Tab::About {
                            (48..=64, 304..=320, 64.0, 304.0)
                        } else {
                            (
                                34..=50,
                                (height.saturating_sub(35))..=(height.saturating_sub(18)),
                                50.0,
                                (height.saturating_sub(35)) as f32,
                            )
                        };
                        if top_arrow_y.contains(&y) {
                            if self.current_tab == Tab::Resume {
                                self.resume_scroll = self.resume_scroll.saturating_sub(2);
                            } else if self.current_tab == Tab::About {
                                self.about_scroll = self.about_scroll.saturating_sub(2);
                            } else {
                                self.detail_scroll = self.detail_scroll.saturating_sub(2);
                            }
                            return true;
                        } else if bot_arrow_y.contains(&y) {
                            if self.current_tab == Tab::Resume {
                                self.resume_scroll = (self.resume_scroll + 2).min(max_scroll);
                            } else if self.current_tab == Tab::About {
                                self.about_scroll = (self.about_scroll + 2).min(max_scroll);
                            } else {
                                self.detail_scroll = (self.detail_scroll + 2).min(max_scroll);
                            }
                            return true;
                        } else if (y as f32) > track_top && (y as f32) < track_bot {
                            let ratio = (y as f32 - track_top) / (track_bot - track_top);
                            let new_scroll =
                                ((ratio * max_scroll as f32).round() as usize).min(max_scroll);
                            if self.current_tab == Tab::Resume {
                                self.resume_scroll = new_scroll;
                            } else if self.current_tab == Tab::About {
                                self.about_scroll = new_scroll;
                            } else {
                                self.detail_scroll = new_scroll;
                            }
                            return true;
                        }
                    }
                }

                if self.current_tab == Tab::Links {
                    let (top, lh, visible) = self.links_geometry();
                    if y >= top && y < top + visible as u16 * 2 * lh {
                        self.select_link(self.links_scroll + ((y - top) / (2 * lh)) as usize);
                        return true;
                    }
                }

                // 3. Visuals Tab interaction (presets and sliders) - Web only
                if self.platform == Platform::Web && self.current_tab == Tab::Visuals {
                    if cols == 80 {
                        // Presets row at Row 5: y = 80 (clickable y = 72..=96)
                        if (72..=96).contains(&y) {
                            if (16..112).contains(&x) {
                                self.visual_effects = VisualEffects::clean();
                                self.status = "Preset: Clean (Pixel-Perfect)".to_string();
                                return true;
                            } else if (112..208).contains(&x) {
                                self.visual_effects = VisualEffects::crt_trinitron();
                                self.status = "Preset: 80s Trinitron CRT".to_string();
                                return true;
                            } else if (208..312).contains(&x) {
                                self.visual_effects = VisualEffects::crt_arcade();
                                self.status = "Preset: Arcade Cabinet".to_string();
                                return true;
                            } else if (312..408).contains(&x) {
                                self.visual_effects = VisualEffects::phosphor_bloom();
                                self.status = "Preset: Phosphor Bloom".to_string();
                                return true;
                            } else if (408..512).contains(&x) {
                                self.visual_effects = VisualEffects::retro_glitch();
                                self.status = "Preset: Retro Glitch".to_string();
                                return true;
                            } else if (512..=630).contains(&x) {
                                self.visual_effects = VisualEffects::default();
                                self.status = "Preset: Default CRT".to_string();
                                return true;
                            }
                        }

                        // Effect rows at Rows 7..15: y = 112..=256 (9 rows with step 16)
                        if (112..256).contains(&y) {
                            let row = ((y - 112) / 16).min(8);
                            self.selected_fx_slider = row as usize;

                            let direct_val = if (248..=424).contains(&x) {
                                let ratio = (x.saturating_sub(256) as f32 / 160.0).clamp(0.0, 1.0);
                                Some((ratio * 20.0).round() / 20.0)
                            } else {
                                None
                            };

                            match row {
                                0 => {
                                    self.visual_effects.scanlines = direct_val.unwrap_or({
                                        if self.visual_effects.scanlines < 0.15 {
                                            0.35
                                        } else if self.visual_effects.scanlines < 0.50 {
                                            0.70
                                        } else {
                                            0.0
                                        }
                                    });
                                    self.status = format!(
                                        "Scanlines: {:.0}%",
                                        self.visual_effects.scanlines * 100.0
                                    );
                                    return true;
                                }
                                1 => {
                                    self.visual_effects.pixel_grid = direct_val.unwrap_or({
                                        if self.visual_effects.pixel_grid < 0.10 {
                                            0.20
                                        } else if self.visual_effects.pixel_grid < 0.30 {
                                            0.40
                                        } else {
                                            0.0
                                        }
                                    });
                                    self.status = format!(
                                        "Pixel Grid: {:.0}%",
                                        self.visual_effects.pixel_grid * 100.0
                                    );
                                    return true;
                                }
                                2 => {
                                    self.visual_effects.chromatic = direct_val.unwrap_or({
                                        if self.visual_effects.chromatic < 0.15 {
                                            0.35
                                        } else if self.visual_effects.chromatic < 0.55 {
                                            0.85
                                        } else {
                                            0.0
                                        }
                                    });
                                    self.status = format!(
                                        "Chromatic Aberration: {:.0}%",
                                        self.visual_effects.chromatic * 100.0
                                    );
                                    return true;
                                }
                                3 => {
                                    self.visual_effects.afterglow = direct_val.unwrap_or({
                                        if self.visual_effects.afterglow < 0.15 {
                                            0.30
                                        } else if self.visual_effects.afterglow < 0.55 {
                                            0.80
                                        } else {
                                            0.0
                                        }
                                    });
                                    self.status = format!(
                                        "Phosphor Afterglow: {:.0}%",
                                        self.visual_effects.afterglow * 100.0
                                    );
                                    return true;
                                }
                                4 => {
                                    self.visual_effects.curvature = direct_val.unwrap_or({
                                        if self.visual_effects.curvature < 0.10 {
                                            0.25
                                        } else if self.visual_effects.curvature < 0.35 {
                                            0.45
                                        } else {
                                            0.0
                                        }
                                    });
                                    self.status = format!(
                                        "CRT Curvature: {:.0}%",
                                        self.visual_effects.curvature * 100.0
                                    );
                                    return true;
                                }
                                5 => {
                                    self.visual_effects.jitter = direct_val.unwrap_or({
                                        if self.visual_effects.jitter < 0.10 {
                                            0.25
                                        } else if self.visual_effects.jitter < 0.40 {
                                            0.65
                                        } else {
                                            0.0
                                        }
                                    });
                                    self.status = format!(
                                        "Signal Jitter: {:.0}%",
                                        self.visual_effects.jitter * 100.0
                                    );
                                    return true;
                                }
                                6 => {
                                    self.visual_effects.magnet = direct_val.unwrap_or({
                                        if self.visual_effects.magnet < 0.05 {
                                            0.10
                                        } else if self.visual_effects.magnet < 0.18 {
                                            0.25
                                        } else {
                                            0.0
                                        }
                                    });
                                    self.status = format!(
                                        "Point CRT Magnet Deflection: {:.0}%",
                                        self.visual_effects.magnet * 100.0
                                    );
                                    return true;
                                }
                                7 => {
                                    self.visual_effects.antenna_hum = direct_val.unwrap_or({
                                        if self.visual_effects.antenna_hum < 0.10 {
                                            0.25
                                        } else if self.visual_effects.antenna_hum < 0.40 {
                                            0.60
                                        } else {
                                            0.0
                                        }
                                    });
                                    self.status = format!(
                                        "Sporadic Antenna Hum: {:.0}%",
                                        self.visual_effects.antenna_hum * 100.0
                                    );
                                    return true;
                                }
                                8 => {
                                    self.visual_effects.noise = direct_val.unwrap_or({
                                        if self.visual_effects.noise < 0.10 {
                                            0.25
                                        } else if self.visual_effects.noise < 0.40 {
                                            0.60
                                        } else {
                                            0.0
                                        }
                                    });
                                    self.status = format!(
                                        "White Noise Snow: {:.0}%",
                                        self.visual_effects.noise * 100.0
                                    );
                                    return true;
                                }
                                _ => {}
                            }
                        }
                    } else {
                        // 40 / 32 cols: Presets at y = 20..=38
                        if (20..=38).contains(&y) {
                            let step = width / 5;
                            if x < step {
                                self.visual_effects = VisualEffects::clean();
                                self.status = "Clean FX".to_string();
                                return true;
                            } else if x < step * 2 {
                                self.visual_effects = VisualEffects::crt_trinitron();
                                self.status = "CRT FX".to_string();
                                return true;
                            } else if x < step * 3 {
                                self.visual_effects = VisualEffects::crt_arcade();
                                self.status = "Arcade FX".to_string();
                                return true;
                            } else if x < step * 4 {
                                self.visual_effects = VisualEffects::phosphor_bloom();
                                self.status = "Bloom FX".to_string();
                                return true;
                            } else {
                                self.visual_effects = VisualEffects::retro_glitch();
                                self.status = "Glitch FX".to_string();
                                return true;
                            }
                        }

                        // Effect rows: 40 cols at y = 45..126 (step 9), 32 cols at y = 44..116 (step 8)
                        let start_y = if cols == 40 { 45 } else { 44 };
                        let step_y = if cols == 40 { 9 } else { 8 };
                        if y >= start_y && y < start_y + 9 * step_y {
                            let row = (y - start_y) / step_y;
                            self.selected_fx_slider = (row as usize).min(8);

                            let direct_val = if cols == 40 && (92..=180).contains(&x) {
                                let ratio = (x.saturating_sub(100) as f32 / 80.0).clamp(0.0, 1.0);
                                Some((ratio * 10.0).round() / 10.0)
                            } else if cols == 32 && (64..=136).contains(&x) {
                                let ratio = (x.saturating_sub(72) as f32 / 64.0).clamp(0.0, 1.0);
                                Some((ratio * 8.0).round() / 8.0)
                            } else {
                                None
                            };

                            match row {
                                0 => {
                                    self.visual_effects.scanlines = direct_val.unwrap_or({
                                        if self.visual_effects.scanlines < 0.2 {
                                            0.45
                                        } else {
                                            0.0
                                        }
                                    });
                                    return true;
                                }
                                1 => {
                                    self.visual_effects.pixel_grid = direct_val.unwrap_or({
                                        if self.visual_effects.pixel_grid < 0.15 {
                                            0.30
                                        } else {
                                            0.0
                                        }
                                    });
                                    return true;
                                }
                                2 => {
                                    self.visual_effects.chromatic = direct_val.unwrap_or({
                                        if self.visual_effects.chromatic < 0.2 {
                                            0.50
                                        } else {
                                            0.0
                                        }
                                    });
                                    return true;
                                }
                                3 => {
                                    self.visual_effects.afterglow = direct_val.unwrap_or({
                                        if self.visual_effects.afterglow < 0.2 {
                                            0.60
                                        } else {
                                            0.0
                                        }
                                    });
                                    return true;
                                }
                                4 => {
                                    self.visual_effects.curvature = direct_val.unwrap_or({
                                        if self.visual_effects.curvature < 0.15 {
                                            0.35
                                        } else {
                                            0.0
                                        }
                                    });
                                    return true;
                                }
                                5 => {
                                    self.visual_effects.jitter = direct_val.unwrap_or({
                                        if self.visual_effects.jitter < 0.15 {
                                            0.45
                                        } else {
                                            0.0
                                        }
                                    });
                                    return true;
                                }
                                6 => {
                                    self.visual_effects.magnet = direct_val.unwrap_or({
                                        if self.visual_effects.magnet < 0.05 {
                                            0.10
                                        } else if self.visual_effects.magnet < 0.18 {
                                            0.25
                                        } else {
                                            0.0
                                        }
                                    });
                                    return true;
                                }
                                7 => {
                                    self.visual_effects.antenna_hum = direct_val.unwrap_or({
                                        if self.visual_effects.antenna_hum < 0.15 {
                                            0.35
                                        } else {
                                            0.0
                                        }
                                    });
                                    return true;
                                }
                                8 => {
                                    self.visual_effects.noise = direct_val.unwrap_or({
                                        if self.visual_effects.noise < 0.15 {
                                            0.35
                                        } else {
                                            0.0
                                        }
                                    });
                                    return true;
                                }
                                _ => {}
                            }
                        }
                    }
                }

                // 4. Project list interaction
                if self.current_tab == Tab::Projects {
                    if self.show_detail {
                        let back_y = if cols >= 80 {
                            let y = (8 + self.detail_wide_visible_rows() as u16)
                                * self.resolution.line_height();
                            y..=y + self.resolution.line_height() - 1
                        } else {
                            (height.saturating_sub(30))..=(height.saturating_sub(8))
                        };
                        if back_y.contains(&y) {
                            self.show_detail = false;
                            return true;
                        }
                    } else {
                        let list_top = if cols >= 80 {
                            3 * self.resolution.line_height()
                        } else {
                            24
                        };
                        let project_row_h = if cols >= 80 {
                            2 * self.resolution.line_height()
                        } else {
                            self.compact_project_pitch(self.scroll_offset, cols)
                        };
                        let prefix_row_h = if cols >= 80 {
                            self.resolution.line_height()
                        } else if cols <= 40 {
                            8
                        } else {
                            project_row_h
                        };
                        let prefix_rows = usize::from(self.scroll_offset == 0) * 4;
                        if self.scroll_offset == 0 && y >= list_top {
                            let prefix_y_end = list_top + prefix_rows as u16 * prefix_row_h;
                            if y < prefix_y_end {
                                let row = ((y - list_top) / prefix_row_h) as usize;
                                if row < PROJECT_LIST_PREFIX_ITEMS {
                                    if self.selected_list_item == row {
                                        self.activate_selected_list_item();
                                    } else {
                                        self.select_list_item(row);
                                    }
                                }
                                return true;
                            }
                        }
                        let project_top = list_top + prefix_rows as u16 * prefix_row_h;
                        let list_bot = project_top
                            + self.projects_visible_at(self.scroll_offset) as u16 * project_row_h;
                        if y >= project_top && y < list_bot {
                            let row_idx = ((y - project_top) / project_row_h) as usize;
                            let project_idx = self.scroll_offset + row_idx;
                            if project_idx < PROJECTS.len() {
                                let list_item = project_idx + PROJECT_LIST_PREFIX_ITEMS;
                                if self.selected_list_item == list_item {
                                    self.activate_selected_list_item();
                                } else {
                                    self.select_list_item(list_item);
                                }
                                return true;
                            }
                        }
                    }
                }
                false
            }
            InputEvent::PointerMove { x, y } => {
                let changed = self.mouse_pos != Some((x, y));
                self.mouse_pos = Some((x, y));

                // Hot corner: top-right corner initiates CRT screensaver!
                let (width, _height) = self.system_mode.resolution();
                let in_hot_corner = x >= width.saturating_sub(64) && y <= 24;
                if in_hot_corner {
                    if !self.screensaver_active {
                        self.screensaver_active = true;
                        return true;
                    }
                } else if self.screensaver_active {
                    self.screensaver_active = false;
                    return true;
                }

                changed
            }
            InputEvent::PointerLeave => {
                let changed = self.mouse_pos.is_some() || self.screensaver_active;
                self.mouse_pos = None;
                self.screensaver_active = false;
                changed
            }
            InputEvent::Resize { .. } => true,
            _ => false,
        }
    }
}
