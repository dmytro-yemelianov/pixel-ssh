//! Core application state, Tab enum, and session containers.

use crate::data::{
    project_detail_lines, ABOUT_LINES_32, ABOUT_LINES_40, ABOUT_LINES_80, PROJECTS,
    RESUME_LINES_32, RESUME_LINES_40, RESUME_LINES_80,
};
use pixel_ssh_view::{
    ActiveModal, ColorPalette, InterfaceTheme, Platform, ResolutionMode, SystemMode, VisualEffects,
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
    /// Focused row in the Projects list: CV, Contacts, About, then projects.
    pub selected_list_item: usize,
    pub selected_project: usize,
    pub show_detail: bool,
    pub status: String,
    pub scroll_offset: usize,
    pub resume_scroll: usize,
    pub about_scroll: usize,
    pub detail_scroll: usize,
    pub resolution: ResolutionMode,
    /// Native RGB palette of the selected display system.
    pub color_palette: ColorPalette,
    /// Native glyph treatment of the selected display system.
    pub interface_theme: InterfaceTheme,
    /// Geometry compatibility field. It always follows `resolution`.
    pub system_mode: SystemMode,
    /// Font/cursor compatibility field. It always follows `interface_theme`.
    pub palette_mode: SystemMode,
    pub visual_effects: VisualEffects,
    pub tick: usize,
    pub clock_time: Option<(u8, u8, u8)>,
    pub selected_fx_slider: usize,
    pub selected_detail_item: usize,
    pub mouse_pos: Option<(u16, u16)>,
    pub terminal_cols: u16,
    pub terminal_rows: u16,
    /// Extra logical rows for a tall browser viewport; display system width stays native.
    pub web_height: Option<u16>,
    pub(crate) link_activation: Option<String>,
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
        let color_palette = ColorPalette::default();
        let interface_theme = InterfaceTheme::default();
        let system_mode = resolution.to_system_mode();
        Self {
            platform: Platform::Web,
            current_tab: Tab::Projects,
            active_modal: ActiveModal::None,
            screensaver_active: false,
            screensaver_tick: 0,
            selected_list_item: 0,
            selected_project: 0,
            show_detail: false,
            status: String::from("Ready. P Projects | H Help | S Cycle system | V Visuals"),
            scroll_offset: 0,
            resume_scroll: 0,
            about_scroll: 0,
            detail_scroll: 0,
            resolution,
            color_palette,
            interface_theme,
            system_mode,
            palette_mode: interface_theme.font_mode(),
            visual_effects: VisualEffects::default(),
            tick: 0,
            clock_time: None,
            selected_fx_slider: 0,
            selected_detail_item: 0,
            mouse_pos: None,
            terminal_cols: 80,
            terminal_rows: 25,
            web_height: None,
            link_activation: None,
        }
    }

    pub fn new_terminal() -> Self {
        let resolution = ResolutionMode::default();
        let color_palette = ColorPalette::default();
        let interface_theme = InterfaceTheme::default();
        let system_mode = resolution.to_system_mode();
        Self {
            platform: Platform::Terminal,
            current_tab: Tab::Projects,
            active_modal: ActiveModal::None,
            screensaver_active: false,
            screensaver_tick: 0,
            selected_list_item: 0,
            selected_project: 0,
            show_detail: false,
            status: String::from("Ready. P Projects | H Help | S Cycle system | j/k Scroll"),
            scroll_offset: 0,
            resume_scroll: 0,
            about_scroll: 0,
            detail_scroll: 0,
            resolution,
            color_palette,
            interface_theme,
            system_mode,
            palette_mode: interface_theme.font_mode(),
            visual_effects: VisualEffects::clean(),
            tick: 0,
            clock_time: None,
            selected_fx_slider: 0,
            selected_detail_item: 0,
            mouse_pos: None,
            terminal_cols: 80,
            terminal_rows: 25,
            web_height: None,
            link_activation: None,
        }
    }

    pub fn set_resolution(&mut self, res: ResolutionMode) {
        self.resolution = res;
        self.system_mode = res.to_system_mode();
        self.color_palette = match res {
            ResolutionMode::Svga | ResolutionMode::Sga | ResolutionMode::Vga => {
                ColorPalette::VgaModern
            }
            ResolutionMode::Ega | ResolutionMode::Cga => ColorPalette::Ega,
            ResolutionMode::C64 => ColorPalette::C64,
            ResolutionMode::Atari => ColorPalette::Atari,
            ResolutionMode::ZxSpectrum => ColorPalette::ZxSpectrum,
        };
        self.interface_theme = match res {
            ResolutionMode::Svga | ResolutionMode::Sga | ResolutionMode::Vga => {
                InterfaceTheme::Modern
            }
            ResolutionMode::Ega | ResolutionMode::Cga => InterfaceTheme::Ega,
            ResolutionMode::C64 => InterfaceTheme::C64,
            ResolutionMode::Atari => InterfaceTheme::Atari,
            ResolutionMode::ZxSpectrum => InterfaceTheme::ZxSpectrum,
        };
        self.palette_mode = self.interface_theme.font_mode();
        self.ensure_selected_project_visible();
        let (w, h) = self.resolution.resolution();
        let (c, r) = self.resolution.char_grid();
        self.status = format!("System: {} ({}x{}, {}x{} cols)", res.name(), w, h, c, r);
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
            8 => self.visual_effects.noise = (self.visual_effects.noise + delta).clamp(0.0, 1.0),
            _ => {}
        }
    }

    pub fn set_terminal_size(&mut self, cols: u16, rows: u16) {
        self.terminal_cols = cols.max(40);
        self.terminal_rows = rows.max(20);
    }

    pub fn view_dimensions(&self) -> (u16, u16) {
        if self.platform == Platform::Terminal {
            (self.terminal_cols * 8, self.terminal_rows * 16)
        } else {
            let (width, native_height) = self.resolution.resolution();
            (
                width,
                self.web_height.unwrap_or(native_height).max(native_height),
            )
        }
    }

    pub fn set_web_height(&mut self, height: Option<u16>) {
        if self.web_height != height {
            self.web_height = height;
            self.ensure_selected_project_visible();
            self.detail_scroll = self.detail_scroll.min(self.detail_max_scroll());
            self.resume_scroll = self.resume_scroll.min(self.resume_max_scroll());
            self.about_scroll = self.about_scroll.min(self.about_max_scroll());
        }
    }

    pub(crate) fn compact_project_pitch(&self, offset: usize, cols: u16) -> u16 {
        let (_, height) = self.view_dimensions();
        let summary_y = height.saturating_sub(if cols == 40 { 32 } else { 40 });
        let project_top = 24 + if offset == 0 { 32 } else { 0 };
        let visible = self.projects_visible_at(offset).max(1) as u16;
        (summary_y.saturating_sub(project_top) / visible).max(16)
    }

    pub fn projects_max_visible(&self) -> usize {
        if self.platform == Platform::Terminal {
            (self.terminal_rows as usize).saturating_sub(6).max(6)
        } else {
            let (cols, _) = self.resolution.char_grid();
            let (_, height) = self.view_dimensions();
            let rows = height / self.resolution.line_height();
            match cols {
                100 | 80 => (rows as usize).saturating_sub(6) / 2,
                40 => 9,
                // ZX gives each item a title row and a separate tag row. Eight
                // two-row items fit above the navigation bar and leave one row
                // for the list position and controls.
                _ => 8,
            }
        }
    }

    /// Number of project items that fit with the optional profile links above them.
    pub(crate) fn projects_visible_at(&self, offset: usize) -> usize {
        let prefix_rows = if offset == 0 { 4 } else { 0 };
        if self.platform == Platform::Web {
            let (cols, _) = self.resolution.char_grid();
            let (_, height) = self.view_dimensions();
            let rows = height / self.resolution.line_height();
            if cols >= 80 {
                // Rows 0-2 hold the header; rows -3..-1 hold guidance and navigation.
                return (rows as usize)
                    .saturating_sub(6 + prefix_rows)
                    .checked_div(2)
                    .unwrap_or(0)
                    .max(1);
            }
            if cols == 40 {
                let top = 24 + if offset == 0 { 32 } else { 0 };
                return ((height.saturating_sub(32 + top) / 16) as usize).clamp(1, PROJECTS.len());
            }
            if cols == 32 {
                let top = 24 + if offset == 0 { 32 } else { 0 };
                return ((height.saturating_sub(40 + top) / 16) as usize).clamp(1, PROJECTS.len());
            }
        }
        self.projects_max_visible()
            .saturating_sub(prefix_rows)
            .max(1)
    }

    pub fn projects_max_scroll(&self) -> usize {
        PROJECTS.len().saturating_sub(self.projects_visible_at(1))
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
        self.scroll_offset = self.scroll_offset.min(self.projects_max_scroll());

        if self.selected_project < self.scroll_offset {
            self.scroll_offset = self.selected_project;
        }
        if self.scroll_offset == 0 && self.selected_project >= self.projects_visible_at(0) {
            self.scroll_offset = 1;
        }
        let visible = self.projects_visible_at(self.scroll_offset);
        if self.selected_project >= self.scroll_offset + visible {
            self.scroll_offset = (self.selected_project + 1)
                .saturating_sub(self.projects_visible_at(1))
                .max(1);
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
        let (_, height) = self.view_dimensions();
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
        let (_, height) = self.view_dimensions();
        let visible = (height / self.resolution.line_height()).saturating_sub(7) as usize;
        let len = match cols {
            100 | 80 => ABOUT_LINES_80.len(),
            40 => ABOUT_LINES_40.len(),
            _ => ABOUT_LINES_32.len(),
        };
        len.saturating_sub(visible)
    }

    /// Focusable links on the open project detail page: the repository, the
    /// live demo where the wide web layout shows one, then Return.
    pub fn detail_item_count(&self) -> usize {
        let wide_web = self.platform != Platform::Terminal && self.resolution.char_grid().0 >= 80;
        if wide_web && PROJECTS[self.selected_project].demo.is_some() {
            3
        } else {
            2
        }
    }

    pub fn detail_max_scroll(&self) -> usize {
        let cols = self.detail_content_columns();
        let visible = if self.platform == Platform::Terminal {
            self.terminal_detail_visible_rows()
        } else {
            match cols {
                100 | 80 => self.detail_wide_visible_rows(),
                40 | 32 => (self.view_dimensions().1.saturating_sub(64) / 8) as usize,
                _ => 15,
            }
        };
        let p = &PROJECTS[self.selected_project.min(PROJECTS.len() - 1)];
        let lines = project_detail_lines(p, cols as usize, self.tick);
        lines.len().saturating_sub(visible)
    }

    pub(crate) fn detail_content_columns(&self) -> u16 {
        if self.platform == Platform::Terminal {
            self.terminal_cols.min(80)
        } else {
            self.resolution.char_grid().0
        }
    }

    pub(crate) fn terminal_detail_visible_rows(&self) -> usize {
        (self.terminal_rows as usize).saturating_sub(11).max(6)
    }

    /// Consumes a link requested by keyboard activation. The host opens it.
    pub fn take_link_activation(&mut self) -> Option<String> {
        self.link_activation.take()
    }

    /// Content rows between the fixed detail header and the bottom menu.
    pub(crate) fn detail_wide_visible_rows(&self) -> usize {
        let (_, height) = self.view_dimensions();
        let nav_y = height.saturating_sub(32);
        (nav_y / self.resolution.line_height())
            .saturating_sub(10)
            .max(1) as usize
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
