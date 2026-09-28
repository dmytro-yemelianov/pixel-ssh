//! Core application state, Tab enum, and session containers.

use crate::data::{
    project_detail_lines, ABOUT_LINES_32, ABOUT_LINES_40, ABOUT_LINES_80, PROJECTS,
    RESUME_LINES_32, RESUME_LINES_40, RESUME_LINES_80,
};
use pixel_ssh_view::{
    ActiveModal, ColorTheme, Platform, ResolutionMode, SystemMode, VisualEffects,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Projects,
    Resume,
    About,
    Visuals,
    Contact,
    Help,
}

#[derive(Debug, Clone)]
pub struct App {
    pub platform: Platform,
    pub current_tab: Tab,
    pub active_modal: ActiveModal,
    pub screensaver_active: bool,
    pub screensaver_tick: usize,
    pub selected_project: usize,
    pub show_detail: bool,
    pub status: String,
    pub scroll_offset: usize,
    pub resume_scroll: usize,
    pub about_scroll: usize,
    pub detail_scroll: usize,
    pub resolution: ResolutionMode,
    pub color_theme: ColorTheme,
    pub system_mode: SystemMode,
    pub palette_mode: SystemMode,
    pub visual_effects: VisualEffects,
    pub tick: usize,
    pub clock_time: Option<(u8, u8, u8)>,
    pub selected_fx_slider: usize,
    pub selected_detail_item: usize,
    pub mouse_pos: Option<(u16, u16)>,
    pub terminal_cols: u16,
    pub terminal_rows: u16,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self::new_web()
    }

    pub fn new_web() -> Self {
        let resolution = ResolutionMode::default();
        let color_theme = ColorTheme::default();
        let system_mode = resolution.to_system_mode();
        Self {
            platform: Platform::Web,
            current_tab: Tab::Projects,
            active_modal: ActiveModal::None,
            screensaver_active: false,
            screensaver_tick: 0,
            selected_project: 0,
            show_detail: false,
            status: String::from(
                "Ready. [1-4] Nav | [V] Visuals | [S] System | [?] Help | [Enter] Detail",
            ),
            scroll_offset: 0,
            resume_scroll: 0,
            about_scroll: 0,
            detail_scroll: 0,
            resolution,
            color_theme,
            system_mode,
            palette_mode: system_mode,
            visual_effects: VisualEffects::default(),
            tick: 0,
            clock_time: None,
            selected_fx_slider: 0,
            selected_detail_item: 0,
            mouse_pos: None,
            terminal_cols: 80,
            terminal_rows: 25,
        }
    }

    pub fn new_terminal() -> Self {
        let resolution = ResolutionMode::default();
        let color_theme = ColorTheme::default();
        let system_mode = resolution.to_system_mode();
        Self {
            platform: Platform::Terminal,
            current_tab: Tab::Projects,
            active_modal: ActiveModal::None,
            screensaver_active: false,
            screensaver_tick: 0,
            selected_project: 0,
            show_detail: false,
            status: String::from("Ready. [1-5] Tabs | [S] System | [Enter] Detail | [j/k] Scroll"),
            scroll_offset: 0,
            resume_scroll: 0,
            about_scroll: 0,
            detail_scroll: 0,
            resolution,
            color_theme,
            system_mode,
            palette_mode: system_mode,
            visual_effects: VisualEffects::clean(),
            tick: 0,
            clock_time: None,
            selected_fx_slider: 0,
            selected_detail_item: 0,
            mouse_pos: None,
            terminal_cols: 80,
            terminal_rows: 25,
        }
    }

    pub fn set_resolution(&mut self, res: ResolutionMode) {
        self.resolution = res;
        self.system_mode = res.to_system_mode();
        self.ensure_selected_project_visible();
        let (w, h) = self.resolution.resolution();
        let (c, r) = self.resolution.char_grid();
        self.status = format!("Resolution: {} ({}x{}, {}x{} cols)", res.name(), w, h, c, r);
    }

    pub fn set_color_theme(&mut self, theme: ColorTheme) {
        self.color_theme = theme;
        self.palette_mode = match theme {
            ColorTheme::Amber => SystemMode::Amber,
            ColorTheme::GreenCrt => SystemMode::GreenCrt,
            ColorTheme::Ega => SystemMode::Ega,
            ColorTheme::C64 => SystemMode::C64,
            ColorTheme::Atari => SystemMode::Atari,
            ColorTheme::ZxSpectrum => SystemMode::ZxSpectrum,
            ColorTheme::Commander | ColorTheme::VgaModern => SystemMode::Vga,
        };
        self.status = format!("Theme: {}", theme.name());
    }

    pub fn adjust_selected_slider(&mut self, delta: f32) {
        match self.selected_fx_slider {
            0 => {
                self.visual_effects.scanlines =
                    (self.visual_effects.scanlines + delta).clamp(0.0, 1.0)
            }
            1 => {
                self.visual_effects.pixel_grid =
                    (self.visual_effects.pixel_grid + delta).clamp(0.0, 1.0)
            }
            2 => {
                self.visual_effects.chromatic =
                    (self.visual_effects.chromatic + delta).clamp(0.0, 1.0)
            }
            3 => {
                self.visual_effects.afterglow =
                    (self.visual_effects.afterglow + delta).clamp(0.0, 1.0)
            }
            4 => {
                self.visual_effects.curvature =
                    (self.visual_effects.curvature + delta).clamp(0.0, 1.0)
            }
            5 => self.visual_effects.jitter = (self.visual_effects.jitter + delta).clamp(0.0, 1.0),
            6 => self.visual_effects.magnet = (self.visual_effects.magnet + delta).clamp(0.0, 1.0),
            7 => {
                self.visual_effects.antenna_hum =
                    (self.visual_effects.antenna_hum + delta).clamp(0.0, 1.0)
            }
            _ => {}
        }
    }

    pub fn set_terminal_size(&mut self, cols: u16, rows: u16) {
        self.terminal_cols = cols.max(40);
        self.terminal_rows = rows.max(20);
    }

    pub fn projects_max_visible(&self) -> usize {
        if self.platform == Platform::Terminal {
            (self.terminal_rows as usize).saturating_sub(6).max(6)
        } else {
            let (cols, _) = self.resolution.char_grid();
            match cols {
                100 => 24,
                80 => 15,
                40 => 12,
                // ZX gives each item a title row and a separate tag row. Eight
                // two-row items fit above the navigation bar and leave one row
                // for the list position and controls.
                _ => 8,
            }
        }
    }

    pub fn projects_max_scroll(&self) -> usize {
        PROJECTS.len().saturating_sub(self.projects_max_visible())
    }

    /// Keeps the current project in the visible window after a navigation or
    /// resolution change. Rendering can then use one shared row count.
    pub fn ensure_selected_project_visible(&mut self) {
        if PROJECTS.is_empty() {
            self.selected_project = 0;
            self.scroll_offset = 0;
            return;
        }

        self.selected_project = self.selected_project.min(PROJECTS.len() - 1);
        let visible = self.projects_max_visible();
        self.scroll_offset = self.scroll_offset.min(self.projects_max_scroll());

        if self.selected_project < self.scroll_offset {
            self.scroll_offset = self.selected_project;
        } else if self.selected_project >= self.scroll_offset + visible {
            self.scroll_offset = self.selected_project + 1 - visible;
        }
    }

    pub fn tick(&mut self) {
        self.tick = self.tick.wrapping_add(1);
        if self.screensaver_active {
            self.screensaver_tick = self.screensaver_tick.wrapping_add(1);
        }
    }

    pub fn set_time(&mut self, hours: u8, minutes: u8, seconds: u8) {
        self.clock_time = Some((hours, minutes, seconds));
    }

    pub fn clock_formatted(&self) -> String {
        if let Some((h, m, s)) = self.clock_time {
            let colon = if s % 2 == 0 { ":" } else { " " };
            format!("{h:02}{colon}{m:02}")
        } else {
            #[cfg(not(target_arch = "wasm32"))]
            {
                if let Ok(duration) =
                    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
                {
                    let secs = duration.as_secs();
                    let s = (secs % 60) as u8;
                    let m = ((secs / 60) % 60) as u8;
                    let h = ((secs / 3600) % 24) as u8;
                    let colon = if s % 2 == 0 { ":" } else { " " };
                    return format!("{h:02}{colon}{m:02}");
                }
            }
            let total_secs = self.tick / 10;
            let s = (total_secs % 60) as u8;
            let m = ((total_secs / 60) % 60) as u8;
            let h = ((total_secs / 3600) % 24) as u8;
            let colon = if s % 2 == 0 { ":" } else { " " };
            format!("{h:02}{colon}{m:02}")
        }
    }

    pub fn resume_max_scroll(&self) -> usize {
        if self.platform == Platform::Terminal {
            let visible = (self.terminal_rows as usize).saturating_sub(6).max(8);
            return RESUME_LINES_80.len().saturating_sub(visible);
        }
        let (cols, _) = self.resolution.char_grid();
        let (_, height) = self.resolution.resolution();
        let visible = (height / self.resolution.line_height()).saturating_sub(7) as usize;
        let len = match cols {
            100 | 80 => RESUME_LINES_80.len(),
            40 => RESUME_LINES_40.len(),
            _ => RESUME_LINES_32.len(),
        };
        len.saturating_sub(visible)
    }

    pub fn about_max_scroll(&self) -> usize {
        if self.platform == Platform::Terminal {
            let visible = (self.terminal_rows as usize).saturating_sub(6).max(8);
            return ABOUT_LINES_80.len().saturating_sub(visible);
        }
        let (cols, _) = self.resolution.char_grid();
        let (_, height) = self.resolution.resolution();
        let visible = (height / self.resolution.line_height()).saturating_sub(7) as usize;
        let len = match cols {
            100 | 80 => ABOUT_LINES_80.len(),
            40 => ABOUT_LINES_40.len(),
            _ => ABOUT_LINES_32.len(),
        };
        len.saturating_sub(visible)
    }

    pub fn detail_max_scroll(&self) -> usize {
        let (cols, _) = self.resolution.char_grid();
        let visible = if self.platform == Platform::Terminal {
            (self.terminal_rows as usize).saturating_sub(11).max(8)
        } else {
            match cols {
                100 | 80 => 13,
                40 => 16,
                _ => 15,
            }
        };
        let p = &PROJECTS[self.selected_project.min(PROJECTS.len() - 1)];
        let lines = project_detail_lines(p, cols as usize, self.tick);
        lines.len().saturating_sub(visible)
    }
}

pub struct Session {
    pub id: String,
    pub app: App,
}

impl Session {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            app: App::new_terminal(),
        }
    }
}
