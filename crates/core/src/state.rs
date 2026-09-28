//! Core application state, Tab enum, and session containers.

use pixel_ssh_view::{ActiveModal, Platform, SystemMode, VisualEffects};
use crate::data::{
    project_detail_lines, ABOUT_LINES_32, ABOUT_LINES_40, ABOUT_LINES_80,
    PROJECTS, RESUME_LINES_32, RESUME_LINES_40, RESUME_LINES_80,
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
        let system_mode = SystemMode::default();
        Self {
            platform: Platform::Web,
            current_tab: Tab::Projects,
            active_modal: ActiveModal::None,
            screensaver_active: false,
            screensaver_tick: 0,
            selected_project: 0,
            show_detail: false,
            status: String::from("Ready. [1-4] Nav | [V] Visuals | [S] System | [?] Help | [Enter] Detail"),
            scroll_offset: 0,
            resume_scroll: 0,
            about_scroll: 0,
            detail_scroll: 0,
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
        let system_mode = SystemMode::default();
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

    pub fn set_terminal_size(&mut self, cols: u16, rows: u16) {
        self.terminal_cols = cols.max(40);
        self.terminal_rows = rows.max(10);
    }

    pub fn projects_max_visible(&self) -> usize {
        if self.platform == Platform::Terminal {
            (self.terminal_rows as usize).saturating_sub(5).max(6)
        } else {
            let (cols, _) = self.palette_mode.char_grid();
            match cols {
                100 => 28,
                80 => 18,
                40 => 12,
                _ => 10,
            }
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
            format!("{:02}{}{:02}", h, colon, m)
        } else {
            #[cfg(not(target_arch = "wasm32"))]
            {
                if let Ok(duration) = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
                    let secs = duration.as_secs();
                    let s = (secs % 60) as u8;
                    let m = ((secs / 60) % 60) as u8;
                    let h = ((secs / 3600) % 24) as u8;
                    let colon = if s % 2 == 0 { ":" } else { " " };
                    return format!("{:02}{}{:02}", h, colon, m);
                }
            }
            let total_secs = self.tick / 10;
            let s = (total_secs % 60) as u8;
            let m = ((total_secs / 60) % 60) as u8;
            let h = ((total_secs / 3600) % 24) as u8;
            let colon = if s % 2 == 0 { ":" } else { " " };
            format!("{:02}{}{:02}", h, colon, m)
        }
    }

    pub fn resume_max_scroll(&self) -> usize {
        if self.platform == Platform::Terminal {
            let visible = (self.terminal_rows as usize).saturating_sub(6).max(8);
            return RESUME_LINES_80.len().saturating_sub(visible);
        }
        let (cols, _) = self.palette_mode.char_grid();
        match cols {
            80 => RESUME_LINES_80.len().saturating_sub(15),
            40 => RESUME_LINES_40.len().saturating_sub(14),
            _ => RESUME_LINES_32.len().saturating_sub(13),
        }
    }

    pub fn about_max_scroll(&self) -> usize {
        if self.platform == Platform::Terminal {
            let visible = (self.terminal_rows as usize).saturating_sub(6).max(8);
            return ABOUT_LINES_80.len().saturating_sub(visible);
        }
        let (cols, _) = self.palette_mode.char_grid();
        match cols {
            80 => ABOUT_LINES_80.len().saturating_sub(18),
            40 => ABOUT_LINES_40.len().saturating_sub(19),
            _ => ABOUT_LINES_32.len().saturating_sub(18),
        }
    }

    pub fn detail_max_scroll(&self) -> usize {
        let (cols, _) = self.palette_mode.char_grid();
        let visible = if self.platform == Platform::Terminal {
            (self.terminal_rows as usize).saturating_sub(11).max(8)
        } else {
            match cols {
                80 => 13,
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
