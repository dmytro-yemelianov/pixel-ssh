//! Event handling and user input processing for the application.

use pixel_ssh_view::{ActiveModal, InputEvent, Key, Platform, SystemMode, VisualEffects};
use crate::data::PROJECTS;
use crate::state::{App, Tab};

impl App {
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

                // If active modal is open, handle modal-specific keys
                if self.active_modal != ActiveModal::None {
                    match key {
                        Key::Escape => {
                            self.active_modal = ActiveModal::None;
                            return true;
                        }
                        Key::Char('1'..='9') | Key::Char('0') if self.active_modal == ActiveModal::System => {
                            let digit = match key {
                                Key::Char('0') => 9,
                                Key::Char(c) => (c as usize) - ('1' as usize),
                                _ => 0,
                            };
                            if digit < SystemMode::ALL.len() {
                                self.system_mode = SystemMode::ALL[digit];
                                self.palette_mode = self.system_mode;
                                self.active_modal = ActiveModal::None;
                                let (w, h) = self.system_mode.resolution();
                                let (c, r) = self.system_mode.char_grid();
                                self.status = format!("{}: {}x{} ({}x{})", self.system_mode.name(), w, h, c, r);
                                return true;
                            }
                        }
                        Key::Char('v') | Key::Char('V') if self.active_modal == ActiveModal::Visuals => {
                            self.active_modal = ActiveModal::None;
                            return true;
                        }
                        Key::Char('s') | Key::Char('S') if self.active_modal == ActiveModal::System => {
                            self.active_modal = ActiveModal::None;
                            return true;
                        }
                        Key::Char('?') | Key::Char('h') | Key::Char('H') if self.active_modal == ActiveModal::Help => {
                            self.active_modal = ActiveModal::None;
                            return true;
                        }
                        _ => {}
                    }
                }

                match key {
                    // Universal workstation switching (Keys 1..=4)
                    Key::Char('1') => {
                        self.current_tab = Tab::Projects;
                        self.show_detail = false;
                        self.active_modal = ActiveModal::None;
                        true
                    }
                    Key::Char('2') => {
                        self.current_tab = Tab::Resume;
                        self.show_detail = false;
                        self.active_modal = ActiveModal::None;
                        true
                    }
                    Key::Char('3') => {
                        self.current_tab = Tab::About;
                        self.show_detail = false;
                        self.active_modal = ActiveModal::None;
                        true
                    }
                    Key::Char('4') => {
                        self.current_tab = Tab::Contact;
                        self.show_detail = false;
                        self.active_modal = ActiveModal::None;
                        true
                    }
                    Key::Char('5') => {
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
                    Key::Char('6') => {
                        if self.platform == Platform::Web {
                            self.active_modal = if self.active_modal == ActiveModal::Visuals {
                                ActiveModal::None
                            } else {
                                ActiveModal::Visuals
                            };
                            true
                        } else {
                            false
                        }
                    }

                    // Visuals modal toggle shortcut ('v' / 'V') on Web
                    Key::Char('v') | Key::Char('V') if self.platform == Platform::Web => {
                        self.active_modal = if self.active_modal == ActiveModal::Visuals {
                            ActiveModal::None
                        } else {
                            ActiveModal::Visuals
                        };
                        true
                    }

                    // Help toggle shortcut ('?')
                    Key::Char('?') => {
                        if self.platform == Platform::Web {
                            self.active_modal = if self.active_modal == ActiveModal::Help {
                                ActiveModal::None
                            } else {
                                ActiveModal::Help
                            };
                        } else {
                            if self.current_tab == Tab::Help {
                                self.current_tab = Tab::Projects;
                            } else {
                                self.current_tab = Tab::Help;
                                self.show_detail = false;
                            }
                        }
                        true
                    }

                    // System Mode selector ('s' / 'S')
                    Key::Char('s') | Key::Char('S') | Key::Char('p') | Key::Char('P') => {
                        if self.platform == Platform::Web {
                            self.active_modal = if self.active_modal == ActiveModal::System {
                                ActiveModal::None
                            } else {
                                ActiveModal::System
                            };
                            true
                        } else {
                            self.system_mode = self.system_mode.next();
                            self.palette_mode = self.system_mode;
                            let (w, h) = self.system_mode.resolution();
                            let (c, r) = self.system_mode.char_grid();
                            self.status = format!("{}: {}x{} ({}x{})", self.system_mode.name(), w, h, c, r);
                            true
                        }
                    }

                    // Tab key: In detail view, cycles interactive items; otherwise cycles views
                    Key::Tab => {
                        if self.current_tab == Tab::Projects && self.show_detail {
                            self.selected_detail_item = (self.selected_detail_item + 1) % 2;
                            true
                        } else {
                            self.current_tab = match (self.platform, self.current_tab) {
                                (Platform::Web, Tab::Projects) => Tab::Resume,
                                (Platform::Web, Tab::Resume) => Tab::About,
                                (Platform::Web, Tab::About) => Tab::Contact,
                                (Platform::Web, Tab::Contact) => Tab::Projects,
                                (Platform::Web, _) => Tab::Projects,

                                (Platform::Terminal, Tab::Projects) => Tab::Resume,
                                (Platform::Terminal, Tab::Resume) => Tab::About,
                                (Platform::Terminal, Tab::About) => Tab::Contact,
                                (Platform::Terminal, Tab::Visuals) => Tab::Contact,
                                (Platform::Terminal, Tab::Contact) => Tab::Help,
                                (Platform::Terminal, Tab::Help) => Tab::Projects,
                            };
                            self.show_detail = false;
                            true
                        }
                    }

                // Project Detail Left/Right navigation ('h' / 'l' or Left / Right)
                Key::Char('h') | Key::Left if self.current_tab == Tab::Projects && self.show_detail => {
                    if self.selected_project > 0 {
                        self.selected_project -= 1;
                    } else {
                        self.selected_project = PROJECTS.len().saturating_sub(1);
                    }
                    self.detail_scroll = 0;
                    self.selected_detail_item = 0;
                    true
                }
                Key::Char('l') | Key::Right if self.current_tab == Tab::Projects && self.show_detail => {
                    self.selected_project = (self.selected_project + 1) % PROJECTS.len();
                    self.detail_scroll = 0;
                    self.selected_detail_item = 0;
                    true
                }

                // Visuals preset shortcuts (Web only)
                Key::Char('a') | Key::Char('A') if self.platform == Platform::Web && self.current_tab == Tab::Visuals => {
                    self.visual_effects = VisualEffects::clean();
                    self.status = "Preset: Clean (Pixel-Perfect)".to_string();
                    true
                }
                Key::Char('w') | Key::Char('W') if self.platform == Platform::Web && self.current_tab == Tab::Visuals => {
                    self.visual_effects = VisualEffects::crt_trinitron();
                    self.status = "Preset: 80s Trinitron CRT".to_string();
                    true
                }
                Key::Char('c') | Key::Char('C') if self.platform == Platform::Web && self.current_tab == Tab::Visuals => {
                    self.visual_effects = VisualEffects::crt_arcade();
                    self.status = "Preset: Arcade Cabinet".to_string();
                    true
                }
                Key::Char('d') | Key::Char('D') if self.platform == Platform::Web && self.current_tab == Tab::Visuals => {
                    self.visual_effects = VisualEffects::phosphor_bloom();
                    self.status = "Preset: Phosphor Bloom".to_string();
                    true
                }
                Key::Char('e') | Key::Char('E') if self.platform == Platform::Web && self.current_tab == Tab::Visuals => {
                    self.visual_effects = VisualEffects::retro_glitch();
                    self.status = "Preset: Retro Glitch".to_string();
                    true
                }
                Key::Char('r') | Key::Char('R') if self.platform == Platform::Web && self.current_tab == Tab::Visuals => {
                    self.visual_effects = VisualEffects::default();
                    self.status = "Preset: Default CRT".to_string();
                    true
                }

                // Direct Visuals FX slider toggles (Keys Z, X, M, V, B, N, G)
                Key::Char('z') | Key::Char('Z') if self.platform == Platform::Web && self.current_tab == Tab::Visuals => {
                    self.selected_fx_slider = 0;
                    self.visual_effects.scanlines = if self.visual_effects.scanlines < 0.15 { 0.35 } else if self.visual_effects.scanlines < 0.50 { 0.70 } else { 0.0 };
                    self.status = format!("Scanlines: {:.0}%", self.visual_effects.scanlines * 100.0);
                    true
                }
                Key::Char('x') | Key::Char('X') if self.platform == Platform::Web && self.current_tab == Tab::Visuals => {
                    self.selected_fx_slider = 1;
                    self.visual_effects.pixel_grid = if self.visual_effects.pixel_grid < 0.10 { 0.20 } else if self.visual_effects.pixel_grid < 0.30 { 0.40 } else { 0.0 };
                    self.status = format!("Pixel Grid: {:.0}%", self.visual_effects.pixel_grid * 100.0);
                    true
                }
                Key::Char('m') | Key::Char('M') if self.platform == Platform::Web && self.current_tab == Tab::Visuals => {
                    self.selected_fx_slider = 2;
                    self.visual_effects.chromatic = if self.visual_effects.chromatic < 0.15 { 0.35 } else if self.visual_effects.chromatic < 0.55 { 0.85 } else { 0.0 };
                    self.status = format!("Chromatic: {:.0}%", self.visual_effects.chromatic * 100.0);
                    true
                }
                Key::Char('v') | Key::Char('V') if self.platform == Platform::Web && self.current_tab == Tab::Visuals => {
                    self.selected_fx_slider = 3;
                    self.visual_effects.afterglow = if self.visual_effects.afterglow < 0.15 { 0.30 } else if self.visual_effects.afterglow < 0.55 { 0.80 } else { 0.0 };
                    self.status = format!("Afterglow: {:.0}%", self.visual_effects.afterglow * 100.0);
                    true
                }
                Key::Char('b') | Key::Char('B') if self.platform == Platform::Web && self.current_tab == Tab::Visuals => {
                    self.selected_fx_slider = 4;
                    self.visual_effects.curvature = if self.visual_effects.curvature < 0.10 { 0.25 } else if self.visual_effects.curvature < 0.35 { 0.45 } else { 0.0 };
                    self.status = format!("CRT Curvature: {:.0}%", self.visual_effects.curvature * 100.0);
                    true
                }
                Key::Char('n') | Key::Char('N') if self.platform == Platform::Web && self.current_tab == Tab::Visuals => {
                    self.selected_fx_slider = 5;
                    self.visual_effects.jitter = if self.visual_effects.jitter < 0.10 { 0.25 } else if self.visual_effects.jitter < 0.40 { 0.65 } else { 0.0 };
                    self.status = format!("Jitter: {:.0}%", self.visual_effects.jitter * 100.0);
                    true
                }
                Key::Char('g') | Key::Char('G') if self.platform == Platform::Web && self.current_tab == Tab::Visuals => {
                    self.selected_fx_slider = 6;
                    self.visual_effects.magnet = if self.visual_effects.magnet < 0.05 { 0.10 } else if self.visual_effects.magnet < 0.18 { 0.25 } else { 0.0 };
                    self.status = format!("Point CRT Magnet Deflection: {:.0}%", self.visual_effects.magnet * 100.0);
                    true
                }
                Key::Char('h') | Key::Char('H') if self.platform == Platform::Web && self.current_tab == Tab::Visuals => {
                    self.selected_fx_slider = 7;
                    self.visual_effects.antenna_hum = if self.visual_effects.antenna_hum < 0.10 { 0.25 } else if self.visual_effects.antenna_hum < 0.40 { 0.60 } else { 0.0 };
                    self.status = format!("Sporadic Antenna Hum: {:.0}%", self.visual_effects.antenna_hum * 100.0);
                    true
                }
                Key::Char('y') | Key::Char('Y') if self.platform == Platform::Web && self.current_tab == Tab::Visuals => {
                    self.selected_fx_slider = 8;
                    self.visual_effects.noise = if self.visual_effects.noise < 0.10 { 0.25 } else if self.visual_effects.noise < 0.40 { 0.60 } else { 0.0 };
                    self.status = format!("White Noise Snow: {:.0}%", self.visual_effects.noise * 100.0);
                    true
                }

                // Fine slider tuning with Left/Right or -/+ when on Visuals Tab
                Key::Left | Key::Char('-') if self.platform == Platform::Web && self.current_tab == Tab::Visuals => {
                    match self.selected_fx_slider {
                        0 => self.visual_effects.scanlines = (self.visual_effects.scanlines - 0.05).max(0.0),
                        1 => self.visual_effects.pixel_grid = (self.visual_effects.pixel_grid - 0.05).max(0.0),
                        2 => self.visual_effects.chromatic = (self.visual_effects.chromatic - 0.05).max(0.0),
                        3 => self.visual_effects.afterglow = (self.visual_effects.afterglow - 0.05).max(0.0),
                        4 => self.visual_effects.curvature = (self.visual_effects.curvature - 0.05).max(0.0),
                        5 => self.visual_effects.jitter = (self.visual_effects.jitter - 0.05).max(0.0),
                        6 => self.visual_effects.magnet = (self.visual_effects.magnet - 0.05).max(0.0),
                        7 => self.visual_effects.antenna_hum = (self.visual_effects.antenna_hum - 0.05).max(0.0),
                        _ => self.visual_effects.noise = (self.visual_effects.noise - 0.05).max(0.0),
                    }
                    true
                }
                Key::Right | Key::Char('+') | Key::Char('=') if self.platform == Platform::Web && self.current_tab == Tab::Visuals => {
                    match self.selected_fx_slider {
                        0 => self.visual_effects.scanlines = (self.visual_effects.scanlines + 0.05).min(1.0),
                        1 => self.visual_effects.pixel_grid = (self.visual_effects.pixel_grid + 0.05).min(1.0),
                        2 => self.visual_effects.chromatic = (self.visual_effects.chromatic + 0.05).min(1.0),
                        3 => self.visual_effects.afterglow = (self.visual_effects.afterglow + 0.05).min(1.0),
                        4 => self.visual_effects.curvature = (self.visual_effects.curvature + 0.05).min(1.0),
                        5 => self.visual_effects.jitter = (self.visual_effects.jitter + 0.05).min(1.0),
                        6 => self.visual_effects.magnet = (self.visual_effects.magnet + 0.05).min(1.0),
                        7 => self.visual_effects.antenna_hum = (self.visual_effects.antenna_hum + 0.05).min(1.0),
                        _ => self.visual_effects.noise = (self.visual_effects.noise + 0.05).min(1.0),
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
                                self.selected_detail_item = self.selected_detail_item.saturating_sub(1);
                                return true;
                            }
                        } else if self.selected_project > 0 {
                            self.selected_project -= 1;
                            if self.selected_project < self.scroll_offset {
                                self.scroll_offset = self.selected_project;
                            }
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
                                self.selected_detail_item = (self.selected_detail_item + 1).min(1);
                                return true;
                            }
                        } else if self.selected_project + 1 < PROJECTS.len() {
                            self.selected_project += 1;
                            let max_vis = self.projects_max_visible();
                            if self.selected_project >= self.scroll_offset + max_vis {
                                self.scroll_offset = self.selected_project.saturating_sub(max_vis - 1);
                            }
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
                        self.resume_scroll = (self.resume_scroll + 6).min(self.resume_max_scroll());
                        return true;
                    } else if self.current_tab == Tab::About {
                        self.about_scroll = (self.about_scroll + 6).min(self.about_max_scroll());
                        return true;
                    } else if self.current_tab == Tab::Projects && self.show_detail {
                        self.detail_scroll = (self.detail_scroll + 6).min(self.detail_max_scroll());
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
                            self.selected_project = 0;
                            self.scroll_offset = 0;
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
                            self.selected_project = PROJECTS.len().saturating_sub(1);
                            let max_vis = self.projects_max_visible();
                            self.scroll_offset = PROJECTS.len().saturating_sub(max_vis);
                        }
                        return true;
                    }
                    false
                }
                Key::Enter => {
                    if self.current_tab == Tab::Projects {
                        if self.show_detail && self.selected_detail_item == 1 {
                            self.show_detail = false;
                            self.detail_scroll = 0;
                        } else {
                            self.show_detail = !self.show_detail;
                            self.detail_scroll = 0;
                            self.selected_detail_item = 0;
                        }
                        true
                    } else {
                        false
                    }
                }
                Key::Escape | Key::Char('q') => {
                    if self.active_modal != ActiveModal::None {
                        self.active_modal = ActiveModal::None;
                        true
                    } else if self.current_tab == Tab::Resume || self.current_tab == Tab::About {
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
                        false
                    }
                }
                _ => false,
            }
        }
            InputEvent::Wheel { dy, .. } => {
                let delta = (dy.abs() as usize).max(1);
                if self.current_tab == Tab::Resume {
                    if dy > 0 {
                        self.resume_scroll = (self.resume_scroll + delta).min(self.resume_max_scroll());
                    } else if dy < 0 {
                        self.resume_scroll = self.resume_scroll.saturating_sub(delta);
                    }
                    return true;
                } else if self.current_tab == Tab::About {
                    if dy > 0 {
                        self.about_scroll = (self.about_scroll + delta).min(self.about_max_scroll());
                    } else if dy < 0 {
                        self.about_scroll = self.about_scroll.saturating_sub(delta);
                    }
                    return true;
                } else if self.current_tab == Tab::Projects {
                    if self.show_detail {
                        if dy > 0 {
                            self.detail_scroll = (self.detail_scroll + delta).min(self.detail_max_scroll());
                        } else if dy < 0 {
                            self.detail_scroll = self.detail_scroll.saturating_sub(delta);
                        }
                        return true;
                    } else if dy > 0 && self.selected_project + 1 < PROJECTS.len() {
                        self.selected_project += 1;
                        if self.selected_project >= self.scroll_offset + 12 {
                            self.scroll_offset = self.selected_project - 11;
                        }
                        return true;
                    } else if dy < 0 && self.selected_project > 0 {
                        self.selected_project -= 1;
                        if self.selected_project < self.scroll_offset {
                            self.scroll_offset = self.selected_project;
                        }
                        return true;
                    }
                }
                false
            }
            InputEvent::PointerDown { x, y, .. } => {
                self.mouse_pos = Some((x, y));
                let (cols, _rows) = self.system_mode.char_grid();
                let (width, height) = self.system_mode.resolution();

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

                // 2. Active Modal Dialog click handling
                if self.active_modal != ActiveModal::None {
                    let char_h: u16 = if height <= 200 { 8 } else { 16 };
                    let box_w = if cols >= 80 { 540u16.min(width.saturating_sub(32)) } else { width.saturating_sub(16) };
                    let box_h = (13 * char_h + 16).min(height.saturating_sub(24));
                    let box_x = (width.saturating_sub(box_w)) / 2;
                    let box_y = (height.saturating_sub(box_h)) / 2;

                    // Clicking outside dialog box dismisses it
                    if x < box_x || x >= box_x + box_w || y < box_y || y >= box_y + box_h {
                        self.active_modal = ActiveModal::None;
                        return true;
                    }

                    // Clicking [X] close button
                    let close_x = (box_x + box_w).saturating_sub(44);
                    if y >= box_y && y < box_y + char_h + 2 && x >= close_x {
                        self.active_modal = ActiveModal::None;
                        return true;
                    }

                    // System modal row selection
                    if self.active_modal == ActiveModal::System {
                        let row_start_y = box_y + char_h + 4;
                        if y >= row_start_y {
                            let row_idx = ((y - row_start_y) / char_h) as usize;
                            if row_idx < SystemMode::ALL.len() {
                                self.system_mode = SystemMode::ALL[row_idx];
                                self.palette_mode = self.system_mode;
                                self.active_modal = ActiveModal::None;
                                let (w, h) = self.system_mode.resolution();
                                let (c, r) = self.system_mode.char_grid();
                                self.status = format!("{}: {}x{} ({}x{})", self.system_mode.name(), w, h, c, r);
                                return true;
                            }
                        }
                    }

                    // Visuals modal presets
                    if self.active_modal == ActiveModal::Visuals {
                        let preset_y = box_y + char_h + 4;
                        if y >= preset_y && y < preset_y + char_h + 2 {
                            if x >= box_x + 80 && x < box_x + 135 {
                                self.visual_effects = VisualEffects::clean();
                                self.status = "Preset: Clean".to_string();
                                return true;
                            } else if x >= box_x + 140 && x < box_x + 225 {
                                self.visual_effects = VisualEffects::crt_trinitron();
                                self.status = "Preset: 80s Trinitron".to_string();
                                return true;
                            } else if x >= box_x + 230 && x < box_x + 300 {
                                self.visual_effects = VisualEffects::crt_arcade();
                                self.status = "Preset: Arcade Cabinet".to_string();
                                return true;
                            } else if x >= box_x + 305 && x < box_x + 360 {
                                self.visual_effects = VisualEffects::phosphor_bloom();
                                self.status = "Preset: Phosphor Bloom".to_string();
                                return true;
                            } else if x >= box_x + 365 && x < box_x + 430 {
                                self.visual_effects = VisualEffects::retro_glitch();
                                self.status = "Preset: Retro Glitch".to_string();
                                return true;
                            }
                        }
                    }

                    return true;
                }

                // 3. Article view (CV or About) on Web:
                if self.platform == Platform::Web && (self.current_tab == Tab::Resume || self.current_tab == Tab::About) {
                    let char_h = self.palette_mode.line_height();
                    let pad_x = if cols >= 80 { 16u16 } else { 4u16 };
                    let pad_top = char_h * 2;
                    let pad_bot = char_h * 2;
                    let box_x = pad_x;
                    let box_y = pad_top;
                    let box_w = width.saturating_sub(pad_x * 2);
                    let box_h = ((height.saturating_sub(pad_top + pad_bot)) / char_h) * char_h;

                    let close_btn_w = if cols >= 80 { 64u16 } else { 24u16 };
                    let close_x = (box_x + box_w).saturating_sub(close_btn_w + 8);
                    if (y >= box_y && y < box_y + char_h && x >= close_x)
                        || x < box_x || x >= box_x + box_w || y < box_y || y >= box_y + box_h
                    {
                        self.current_tab = Tab::Projects;
                        return true;
                    }
                }

                // 4. Tab bar clicks & Modal triggers
                let tab_y_range = if cols >= 80 { 16..=32 } else { 10..=22 };
                if tab_y_range.contains(&y) {
                    if self.platform == Platform::Web {
                        if cols >= 80 {
                            if x >= 8 && x < 112 {
                                self.current_tab = Tab::Projects;
                                self.show_detail = false;
                                self.active_modal = ActiveModal::None;
                                return true;
                            } else if x >= 112 && x < 168 {
                                self.current_tab = Tab::Resume;
                                self.show_detail = false;
                                self.active_modal = ActiveModal::None;
                                return true;
                            } else if x >= 168 && x < 248 {
                                self.current_tab = Tab::About;
                                self.show_detail = false;
                                self.active_modal = ActiveModal::None;
                                return true;
                            } else if x >= 248 && x < 340 {
                                self.current_tab = Tab::Contact;
                                self.show_detail = false;
                                self.active_modal = ActiveModal::None;
                                return true;
                            } else if x >= 340 && x < 388 {
                                self.active_modal = if self.active_modal == ActiveModal::Visuals { ActiveModal::None } else { ActiveModal::Visuals };
                                return true;
                            } else if x >= 388 && x < 476 {
                                self.active_modal = if self.active_modal == ActiveModal::System { ActiveModal::None } else { ActiveModal::System };
                                return true;
                            } else if x >= 476 && x < 520 {
                                self.active_modal = if self.active_modal == ActiveModal::Help { ActiveModal::None } else { ActiveModal::Help };
                                return true;
                            } else if x >= width.saturating_sub(80) {
                                self.active_modal = if self.active_modal == ActiveModal::System { ActiveModal::None } else { ActiveModal::System };
                                return true;
                            }
                        } else if cols == 40 {
                            if x < 44 {
                                self.current_tab = Tab::Projects;
                                self.show_detail = false;
                                self.active_modal = ActiveModal::None;
                                return true;
                            } else if x < 86 {
                                self.current_tab = Tab::Resume;
                                self.show_detail = false;
                                self.active_modal = ActiveModal::None;
                                return true;
                            } else if x < 128 {
                                self.current_tab = Tab::About;
                                self.show_detail = false;
                                self.active_modal = ActiveModal::None;
                                return true;
                            } else if x < 170 {
                                self.current_tab = Tab::Contact;
                                self.show_detail = false;
                                self.active_modal = ActiveModal::None;
                                return true;
                            } else if x < 202 {
                                self.active_modal = if self.active_modal == ActiveModal::Visuals { ActiveModal::None } else { ActiveModal::Visuals };
                                return true;
                            } else if x < 234 {
                                self.active_modal = if self.active_modal == ActiveModal::System { ActiveModal::None } else { ActiveModal::System };
                                return true;
                            } else if x < 256 {
                                self.active_modal = if self.active_modal == ActiveModal::Help { ActiveModal::None } else { ActiveModal::Help };
                                return true;
                            } else {
                                self.active_modal = if self.active_modal == ActiveModal::System { ActiveModal::None } else { ActiveModal::System };
                                return true;
                            }
                        } else {
                            // 32 cols
                            if x < 36 {
                                self.current_tab = Tab::Projects;
                                self.show_detail = false;
                                self.active_modal = ActiveModal::None;
                                return true;
                            } else if x < 72 {
                                self.current_tab = Tab::Resume;
                                self.show_detail = false;
                                self.active_modal = ActiveModal::None;
                                return true;
                            } else if x < 108 {
                                self.current_tab = Tab::About;
                                self.show_detail = false;
                                self.active_modal = ActiveModal::None;
                                return true;
                            } else if x < 144 {
                                self.current_tab = Tab::Contact;
                                self.show_detail = false;
                                self.active_modal = ActiveModal::None;
                                return true;
                            } else if x < 176 {
                                self.active_modal = if self.active_modal == ActiveModal::Visuals { ActiveModal::None } else { ActiveModal::Visuals };
                                return true;
                            } else if x < 208 {
                                self.active_modal = if self.active_modal == ActiveModal::System { ActiveModal::None } else { ActiveModal::System };
                                return true;
                            } else {
                                self.active_modal = if self.active_modal == ActiveModal::Help { ActiveModal::None } else { ActiveModal::Help };
                                return true;
                            }
                        }
                    } else {
                        // Platform::Terminal
                        if cols == 80 {
                            if x < 120 {
                                self.current_tab = Tab::Projects;
                                self.show_detail = false;
                                return true;
                            } else if x < 224 {
                                self.current_tab = Tab::Resume;
                                self.show_detail = false;
                                return true;
                            } else if x < 328 {
                                self.current_tab = Tab::About;
                                self.show_detail = false;
                                return true;
                            } else if x < 436 {
                                self.current_tab = Tab::Contact;
                                self.show_detail = false;
                                return true;
                            } else if x < 528 {
                                self.current_tab = Tab::Help;
                                self.show_detail = false;
                                return true;
                            } else if x >= 528 && x <= width {
                                self.system_mode = self.system_mode.next();
                                self.palette_mode = self.system_mode;
                                let (w, h) = self.system_mode.resolution();
                                let (c, r) = self.system_mode.char_grid();
                                self.status = format!("{}: {}x{} ({}x{})", self.system_mode.name(), w, h, c, r);
                                return true;
                            }
                        } else if cols == 40 {
                            if x < 50 {
                                self.current_tab = Tab::Projects;
                                self.show_detail = false;
                                return true;
                            } else if x < 98 {
                                self.current_tab = Tab::Resume;
                                self.show_detail = false;
                                return true;
                            } else if x < 146 {
                                self.current_tab = Tab::About;
                                self.show_detail = false;
                                return true;
                            } else if x < 194 {
                                self.current_tab = Tab::Contact;
                                self.show_detail = false;
                                return true;
                            } else if x < 240 {
                                self.current_tab = Tab::Help;
                                self.show_detail = false;
                                return true;
                            } else {
                                self.system_mode = self.system_mode.next();
                                self.palette_mode = self.system_mode;
                                let (w, h) = self.system_mode.resolution();
                                let (c, r) = self.system_mode.char_grid();
                                self.status = format!("{}: {}x{} ({}x{})", self.system_mode.name(), w, h, c, r);
                                return true;
                            }
                        } else {
                            // 32 cols (ZX Spectrum Terminal)
                            if x < 44 {
                                self.current_tab = Tab::Projects;
                                self.show_detail = false;
                                return true;
                            } else if x < 88 {
                                self.current_tab = Tab::Resume;
                                self.show_detail = false;
                                return true;
                            } else if x < 132 {
                                self.current_tab = Tab::About;
                                self.show_detail = false;
                                return true;
                            } else if x < 176 {
                                self.current_tab = Tab::Contact;
                                self.show_detail = false;
                                return true;
                            } else if x < 224 {
                                self.current_tab = Tab::Help;
                                self.show_detail = false;
                                return true;
                            } else {
                                self.system_mode = self.system_mode.next();
                                self.palette_mode = self.system_mode;
                                let (w, h) = self.system_mode.resolution();
                                let (c, r) = self.system_mode.char_grid();
                                self.status = format!("{}: {}x{} ({}x{})", self.system_mode.name(), w, h, c, r);
                                return true;
                            }
                        }
                    }
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
                        let is_detail_80 = self.current_tab == Tab::Projects && self.show_detail && cols == 80;
                        let (top_arrow_y, bot_arrow_y, track_top, track_bot) = if is_detail_80 {
                            (128..=144, 304..=320, 144.0, 304.0)
                        } else if cols == 80 && self.current_tab == Tab::Resume {
                            (96..=112, 304..=320, 112.0, 304.0)
                        } else if cols == 80 && self.current_tab == Tab::About {
                            (48..=64, 304..=320, 64.0, 304.0)
                        } else {
                            (34..=50, (height.saturating_sub(35))..=(height.saturating_sub(18)), 50.0, (height.saturating_sub(35)) as f32)
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
                            let new_scroll = ((ratio * max_scroll as f32).round() as usize).min(max_scroll);
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

                // 3. Visuals Tab interaction (presets and sliders) - Web only
                if self.platform == Platform::Web && self.current_tab == Tab::Visuals {
                    if cols == 80 {
                        // Presets row at Row 5: y = 80 (clickable y = 72..=96)
                        if y >= 72 && y <= 96 {
                            if x >= 16 && x < 112 {
                                self.visual_effects = VisualEffects::clean();
                                self.status = "Preset: Clean (Pixel-Perfect)".to_string();
                                return true;
                            } else if x >= 112 && x < 208 {
                                self.visual_effects = VisualEffects::crt_trinitron();
                                self.status = "Preset: 80s Trinitron CRT".to_string();
                                return true;
                            } else if x >= 208 && x < 312 {
                                self.visual_effects = VisualEffects::crt_arcade();
                                self.status = "Preset: Arcade Cabinet".to_string();
                                return true;
                            } else if x >= 312 && x < 408 {
                                self.visual_effects = VisualEffects::phosphor_bloom();
                                self.status = "Preset: Phosphor Bloom".to_string();
                                return true;
                            } else if x >= 408 && x < 512 {
                                self.visual_effects = VisualEffects::retro_glitch();
                                self.status = "Preset: Retro Glitch".to_string();
                                return true;
                            } else if x >= 512 && x <= 630 {
                                self.visual_effects = VisualEffects::default();
                                self.status = "Preset: Default CRT".to_string();
                                return true;
                            }
                        }

                        // Effect rows at Rows 7..15: y = 112..=256 (9 rows with step 16)
                        if y >= 112 && y < 256 {
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
                                    self.visual_effects.scanlines = direct_val.unwrap_or_else(|| {
                                        if self.visual_effects.scanlines < 0.15 { 0.35 } else if self.visual_effects.scanlines < 0.50 { 0.70 } else { 0.0 }
                                    });
                                    self.status = format!("Scanlines: {:.0}%", self.visual_effects.scanlines * 100.0);
                                    return true;
                                }
                                1 => {
                                    self.visual_effects.pixel_grid = direct_val.unwrap_or_else(|| {
                                        if self.visual_effects.pixel_grid < 0.10 { 0.20 } else if self.visual_effects.pixel_grid < 0.30 { 0.40 } else { 0.0 }
                                    });
                                    self.status = format!("Pixel Grid: {:.0}%", self.visual_effects.pixel_grid * 100.0);
                                    return true;
                                }
                                2 => {
                                    self.visual_effects.chromatic = direct_val.unwrap_or_else(|| {
                                        if self.visual_effects.chromatic < 0.15 { 0.35 } else if self.visual_effects.chromatic < 0.55 { 0.85 } else { 0.0 }
                                    });
                                    self.status = format!("Chromatic Aberration: {:.0}%", self.visual_effects.chromatic * 100.0);
                                    return true;
                                }
                                3 => {
                                    self.visual_effects.afterglow = direct_val.unwrap_or_else(|| {
                                        if self.visual_effects.afterglow < 0.15 { 0.30 } else if self.visual_effects.afterglow < 0.55 { 0.80 } else { 0.0 }
                                    });
                                    self.status = format!("Phosphor Afterglow: {:.0}%", self.visual_effects.afterglow * 100.0);
                                    return true;
                                }
                                4 => {
                                    self.visual_effects.curvature = direct_val.unwrap_or_else(|| {
                                        if self.visual_effects.curvature < 0.10 { 0.25 } else if self.visual_effects.curvature < 0.35 { 0.45 } else { 0.0 }
                                    });
                                    self.status = format!("CRT Curvature: {:.0}%", self.visual_effects.curvature * 100.0);
                                    return true;
                                }
                                5 => {
                                    self.visual_effects.jitter = direct_val.unwrap_or_else(|| {
                                        if self.visual_effects.jitter < 0.10 { 0.25 } else if self.visual_effects.jitter < 0.40 { 0.65 } else { 0.0 }
                                    });
                                    self.status = format!("Signal Jitter: {:.0}%", self.visual_effects.jitter * 100.0);
                                    return true;
                                }
                                6 => {
                                    self.visual_effects.magnet = direct_val.unwrap_or_else(|| {
                                        if self.visual_effects.magnet < 0.05 { 0.10 } else if self.visual_effects.magnet < 0.18 { 0.25 } else { 0.0 }
                                    });
                                    self.status = format!("Point CRT Magnet Deflection: {:.0}%", self.visual_effects.magnet * 100.0);
                                    return true;
                                }
                                7 => {
                                    self.visual_effects.antenna_hum = direct_val.unwrap_or_else(|| {
                                        if self.visual_effects.antenna_hum < 0.10 { 0.25 } else if self.visual_effects.antenna_hum < 0.40 { 0.60 } else { 0.0 }
                                    });
                                    self.status = format!("Sporadic Antenna Hum: {:.0}%", self.visual_effects.antenna_hum * 100.0);
                                    return true;
                                }
                                8 => {
                                    self.visual_effects.noise = direct_val.unwrap_or_else(|| {
                                        if self.visual_effects.noise < 0.10 { 0.25 } else if self.visual_effects.noise < 0.40 { 0.60 } else { 0.0 }
                                    });
                                    self.status = format!("White Noise Snow: {:.0}%", self.visual_effects.noise * 100.0);
                                    return true;
                                }
                                _ => {}
                            }
                        }
                    } else {
                        // 40 / 32 cols: Presets at y = 20..=38
                        if y >= 20 && y <= 38 {
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
                                    self.visual_effects.scanlines = direct_val.unwrap_or_else(|| {
                                        if self.visual_effects.scanlines < 0.2 { 0.45 } else { 0.0 }
                                    });
                                    return true;
                                }
                                1 => {
                                    self.visual_effects.pixel_grid = direct_val.unwrap_or_else(|| {
                                        if self.visual_effects.pixel_grid < 0.15 { 0.30 } else { 0.0 }
                                    });
                                    return true;
                                }
                                2 => {
                                    self.visual_effects.chromatic = direct_val.unwrap_or_else(|| {
                                        if self.visual_effects.chromatic < 0.2 { 0.50 } else { 0.0 }
                                    });
                                    return true;
                                }
                                3 => {
                                    self.visual_effects.afterglow = direct_val.unwrap_or_else(|| {
                                        if self.visual_effects.afterglow < 0.2 { 0.60 } else { 0.0 }
                                    });
                                    return true;
                                }
                                4 => {
                                    self.visual_effects.curvature = direct_val.unwrap_or_else(|| {
                                        if self.visual_effects.curvature < 0.15 { 0.35 } else { 0.0 }
                                    });
                                    return true;
                                }
                                5 => {
                                    self.visual_effects.jitter = direct_val.unwrap_or_else(|| {
                                        if self.visual_effects.jitter < 0.15 { 0.45 } else { 0.0 }
                                    });
                                    return true;
                                }
                                6 => {
                                    self.visual_effects.magnet = direct_val.unwrap_or_else(|| {
                                        if self.visual_effects.magnet < 0.05 { 0.10 } else if self.visual_effects.magnet < 0.18 { 0.25 } else { 0.0 }
                                    });
                                    return true;
                                }
                                7 => {
                                    self.visual_effects.antenna_hum = direct_val.unwrap_or_else(|| {
                                        if self.visual_effects.antenna_hum < 0.15 { 0.35 } else { 0.0 }
                                    });
                                    return true;
                                }
                                8 => {
                                    self.visual_effects.noise = direct_val.unwrap_or_else(|| {
                                        if self.visual_effects.noise < 0.15 { 0.35 } else { 0.0 }
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
                        let back_y = if cols == 80 { 336..=352 } else { (height.saturating_sub(30))..=(height.saturating_sub(8)) };
                        if back_y.contains(&y) {
                            self.show_detail = false;
                            return true;
                        }
                    } else {
                        let list_top = if cols == 80 { 48 } else { 24 };
                        let row_h = if cols == 80 { 16 } else if cols == 40 { 12 } else { 11 };
                        if y >= list_top && y < height.saturating_sub(16) {
                            let row_idx = ((y - list_top) / row_h) as usize;
                            let project_idx = self.scroll_offset + row_idx;
                            if project_idx < PROJECTS.len() {
                                if self.selected_project == project_idx {
                                    self.show_detail = true;
                                } else {
                                    self.selected_project = project_idx;
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
