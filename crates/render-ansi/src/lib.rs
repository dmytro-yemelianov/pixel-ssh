use pixel_ssh_framebuffer::Framebuffer;
use pixel_ssh_view::{is_table_border_char, Element, Platform, View};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnsiCell {
    pub ch: char,
    pub fg_rgb: [u8; 3],
    pub bg_rgb: [u8; 3],
    pub bold: bool,
    pub underline: bool,
    pub link_url: Option<String>,
}

impl Default for AnsiCell {
    fn default() -> Self {
        Self {
            ch: ' ',
            fg_rgb: [201, 209, 217], // Default light gray
            bg_rgb: [13, 17, 23],    // Default dark bg
            bold: false,
            underline: false,
            link_url: None,
        }
    }
}

pub struct AnsiRenderer {
    pub cols: u16,
    pub rows: u16,
    pub grid: Vec<AnsiCell>,
    previous_grid: Option<Vec<AnsiCell>>,
    previous_cursor: Option<(u16, u16)>,
}

impl AnsiRenderer {
    pub fn new(cols: u16, rows: u16) -> Self {
        let size = (cols as usize) * (rows as usize);
        Self {
            cols,
            rows,
            grid: vec![AnsiCell::default(); size],
            previous_grid: None,
            previous_cursor: None,
        }
    }

    pub fn resize(&mut self, cols: u16, rows: u16) {
        self.cols = cols;
        self.rows = rows;
        let size = (cols as usize) * (rows as usize);
        self.grid = vec![AnsiCell::default(); size];
        self.previous_grid = None;
        self.previous_cursor = None;
    }

    pub fn force_full_redraw(&mut self) {
        self.previous_grid = None;
        self.previous_cursor = None;
    }

    pub fn clear(&mut self, bg_rgb: [u8; 3]) {
        let default_cell = AnsiCell {
            ch: ' ',
            fg_rgb: [201, 209, 217],
            bg_rgb,
            bold: false,
            underline: false,
            link_url: None,
        };
        self.grid.fill(default_cell);
    }

    /// Sanitizes untrusted text by stripping all ANSI/escape control characters.
    pub fn sanitize_text(input: &str) -> String {
        input
            .chars()
            .filter(|&c| c >= ' ' && c != '\x7f' && c != '\x1b')
            .collect()
    }

    /// Maps pixel (x, y) from the system canvas to terminal (col, row) on the active system's grid,
    /// centered nicely if the terminal dimensions exceed the system's character grid.
    pub fn pixel_to_grid(&self, x: u16, y: u16, mode: pixel_ssh_view::PaletteMode) -> (u16, u16) {
        let (target_cols, target_rows) = mode.char_grid();
        let offset_col = self.cols.saturating_sub(target_cols) / 2;
        let offset_row = self.rows.saturating_sub(target_rows) / 2;

        let col = offset_col + (x / 8);
        let row = offset_row
            + if mode.line_height() == 16 {
                // 80x25 system (VGA Modern, Amber, Green CRT)
                (y / 16).min(target_rows.saturating_sub(1))
            } else {
                // 40x25 / 32x24 system (C64, Atari, ZX Spectrum)
                if y < 10 {
                    0 // Header bar
                } else if y < 20 {
                    1 // Tab bar
                } else if y < 24 {
                    2 // Separator line
                } else if y >= (target_rows.saturating_sub(1) * 8) {
                    target_rows.saturating_sub(1) // Status bar
                } else {
                    3 + ((y.saturating_sub(24)) / 8).min(target_rows.saturating_sub(4))
                }
            };

        (col, row)
    }

    /// Renders a platform-independent View into an ANSI string stream.
    pub fn render_view(&mut self, view: &View) -> String {
        let palette = Framebuffer::palette_for_palette(view.color_palette);
        let default_bg = [palette[0][0], palette[0][1], palette[0][2]];
        self.clear(default_bg);

        let is_terminal = view.platform == Platform::Terminal;
        let (target_cols, _target_rows) = view.resolution.char_grid();
        let offset_col = if is_terminal {
            0
        } else {
            self.cols.saturating_sub(target_cols) / 2
        };

        let border_entry = palette[7];
        let border_rgb = [border_entry[0], border_entry[1], border_entry[2]];

        for elem in &view.elements {
            match elem {
                Element::Rect(r) => {
                    let pal_rgb = palette[r.color.palette_index as usize];
                    let rgb = [pal_rgb[0], pal_rgb[1], pal_rgb[2]];

                    // Check if it's a thin horizontal separator rule
                    if r.height <= 2 {
                        let (rx_start, ry) = if is_terminal {
                            (
                                (r.x / 8).min(self.cols.saturating_sub(1)),
                                (r.y / 16).min(self.rows.saturating_sub(1)),
                            )
                        } else {
                            self.pixel_to_grid(r.x, r.y, view.font_mode)
                        };
                        let rx_end = if is_terminal {
                            if r.width >= view.width || r.width >= 600 {
                                self.cols
                            } else {
                                (r.x + r.width).div_ceil(8).min(self.cols)
                            }
                        } else {
                            offset_col
                                + (r.x + r.width)
                                    .div_ceil(8)
                                    .min(self.cols.saturating_sub(offset_col))
                        };
                        let line_rgb = if is_terminal { border_rgb } else { rgb };
                        if ry < self.rows {
                            for x in rx_start..rx_end.min(self.cols) {
                                let idx = (ry as usize) * (self.cols as usize) + (x as usize);
                                if idx < self.grid.len() && self.grid[idx].ch == ' ' {
                                    self.grid[idx].ch = '─';
                                    self.grid[idx].fg_rgb = line_rgb;
                                }
                            }
                        }
                    } else {
                        // Background rect (header, tabs, active card, or status bar)
                        let (rx_start, ry_start) = if is_terminal {
                            (
                                (r.x / 8).min(self.cols.saturating_sub(1)),
                                (r.y / 16).min(self.rows.saturating_sub(1)),
                            )
                        } else {
                            self.pixel_to_grid(r.x, r.y, view.font_mode)
                        };
                        let (rx_end, ry_end) = if is_terminal {
                            let end_col = if r.width >= view.width || r.width >= 600 {
                                self.cols
                            } else {
                                (r.x + r.width).div_ceil(8).min(self.cols)
                            };
                            let end_row = if r.y + r.height >= view.height {
                                self.rows.saturating_sub(1)
                            } else {
                                (r.y + r.height)
                                    .div_ceil(16)
                                    .saturating_sub(1)
                                    .min(self.rows.saturating_sub(1))
                            };
                            (end_col, end_row)
                        } else {
                            self.pixel_to_grid(r.x + r.width, r.y + r.height, view.font_mode)
                        };

                        for y in ry_start..=ry_end.min(self.rows.saturating_sub(1)) {
                            for x in rx_start..rx_end.min(self.cols) {
                                let idx = (y as usize) * (self.cols as usize) + (x as usize);
                                if idx < self.grid.len() {
                                    self.grid[idx].bg_rgb = rgb;
                                }
                            }
                        }
                    }
                }
                Element::Text(t) => {
                    let sanitized = Self::sanitize_text(&t.text);
                    let (mut start_col, row) = if is_terminal {
                        (
                            (t.x / 8).min(self.cols.saturating_sub(1)),
                            (t.y / 16).min(self.rows.saturating_sub(1)),
                        )
                    } else {
                        self.pixel_to_grid(t.x, t.y, view.font_mode)
                    };

                    let fg_entry = palette[t.style.fg.palette_index as usize];
                    let fg_rgb = [fg_entry[0], fg_entry[1], fg_entry[2]];

                    let bg_rgb = t.style.bg.map(|c| {
                        let e = palette[c.palette_index as usize];
                        [e[0], e[1], e[2]]
                    });

                    if row < self.rows {
                        for ch in sanitized.chars() {
                            if start_col >= self.cols {
                                break;
                            }
                            let idx = (row as usize) * (self.cols as usize) + (start_col as usize);
                            if idx < self.grid.len() {
                                self.grid[idx].ch = ch;
                                if is_table_border_char(ch) {
                                    self.grid[idx].fg_rgb = border_rgb;
                                } else {
                                    self.grid[idx].fg_rgb = fg_rgb;
                                }
                                if let Some(bg) = bg_rgb {
                                    self.grid[idx].bg_rgb = bg;
                                }
                                self.grid[idx].bold = t.style.bold;
                            }
                            start_col += 1;
                        }
                    }
                }
                Element::Link(l) => {
                    let sanitized = Self::sanitize_text(&l.text);
                    let (mut start_col, row) = if is_terminal {
                        (
                            (l.x / 8).min(self.cols.saturating_sub(1)),
                            (l.y / 16).min(self.rows.saturating_sub(1)),
                        )
                    } else {
                        self.pixel_to_grid(l.x, l.y, view.font_mode)
                    };

                    let fg_entry = palette[l.style.fg.palette_index as usize];
                    let fg_rgb = [fg_entry[0], fg_entry[1], fg_entry[2]];

                    let bg_rgb = l.style.bg.map(|c| {
                        let e = palette[c.palette_index as usize];
                        [e[0], e[1], e[2]]
                    });

                    if row < self.rows {
                        for ch in sanitized.chars() {
                            if start_col >= self.cols {
                                break;
                            }
                            let idx = (row as usize) * (self.cols as usize) + (start_col as usize);
                            if idx < self.grid.len() {
                                self.grid[idx].ch = ch;
                                if is_table_border_char(ch) {
                                    self.grid[idx].fg_rgb = border_rgb;
                                } else {
                                    self.grid[idx].fg_rgb = fg_rgb;
                                }
                                if let Some(bg) = bg_rgb {
                                    self.grid[idx].bg_rgb = bg;
                                }
                                self.grid[idx].bold = l.style.bold;
                                self.grid[idx].underline = true;
                                self.grid[idx].link_url = Some(l.url.clone());
                            }
                            start_col += 1;
                        }
                    }
                }
                Element::Sprite(s) => {
                    let (start_col, start_row) = if is_terminal {
                        (
                            (s.x / 8).min(self.cols.saturating_sub(1)),
                            (s.y / 16).min(self.rows.saturating_sub(1)),
                        )
                    } else {
                        self.pixel_to_grid(s.x, s.y, view.font_mode)
                    };

                    let sprite_cols = (s.width / 8).max(1);
                    let sprite_rows = (s.height / 16).max(1);

                    for r in 0..sprite_rows {
                        let cell_row = start_row + r;
                        if cell_row >= self.rows {
                            break;
                        }
                        for c in 0..sprite_cols {
                            let cell_col = start_col + c;
                            if cell_col >= self.cols {
                                break;
                            }

                            let top_py = (r * 16) + 4;
                            let bot_py = (r * 16) + 12;
                            let px = (c * 8) + 4;

                            if (top_py as usize) < (s.height as usize)
                                && (bot_py as usize) < (s.height as usize)
                                && (px as usize) < (s.width as usize)
                            {
                                let top_idx =
                                    s.data[(top_py as usize) * (s.width as usize) + (px as usize)];
                                let bot_idx =
                                    s.data[(bot_py as usize) * (s.width as usize) + (px as usize)];

                                let top_entry = palette[top_idx as usize];
                                let bot_entry = palette[bot_idx as usize];

                                let grid_idx = (cell_row as usize) * (self.cols as usize)
                                    + (cell_col as usize);
                                if grid_idx < self.grid.len() {
                                    self.grid[grid_idx].ch = '▀';
                                    self.grid[grid_idx].fg_rgb =
                                        [top_entry[0], top_entry[1], top_entry[2]];
                                    self.grid[grid_idx].bg_rgb =
                                        [bot_entry[0], bot_entry[1], bot_entry[2]];
                                    self.grid[grid_idx].bold = false;
                                }
                            }
                        }
                    }
                }
            }
        }

        let cursor_position = view
            .cursor
            .as_ref()
            .filter(|cursor| cursor.visible)
            .map(|cursor| {
                if is_terminal {
                    (
                        (cursor.x / 8).min(self.cols.saturating_sub(1)),
                        (cursor.y / 16).min(self.rows.saturating_sub(1)),
                    )
                } else {
                    self.pixel_to_grid(cursor.x, cursor.y, view.font_mode)
                }
            });
        let cursor_changed = cursor_position != self.previous_cursor;
        self.previous_cursor = cursor_position;

        // Emit only changed cells after the initial frame. This keeps the animated
        // clock and marquee from retransmitting an entire terminal screen.
        if let Some(previous) = self.previous_grid.replace(self.grid.clone()) {
            if previous.len() == self.grid.len() {
                let mut changed = String::new();
                for (index, (before, cell)) in previous.iter().zip(&self.grid).enumerate() {
                    if before == cell {
                        continue;
                    }
                    let row = index / self.cols as usize + 1;
                    let col = index % self.cols as usize + 1;
                    changed.push_str(&format!("\x1b[{row};{col}H"));
                    changed.push_str("\x1b[0;");
                    if cell.bold {
                        changed.push_str("1;");
                    }
                    if cell.underline {
                        changed.push_str("4;");
                    }
                    changed.push_str(&format!(
                        "38;2;{};{};{};48;2;{};{};{}m",
                        cell.fg_rgb[0],
                        cell.fg_rgb[1],
                        cell.fg_rgb[2],
                        cell.bg_rgb[0],
                        cell.bg_rgb[1],
                        cell.bg_rgb[2]
                    ));
                    if let Some(url) = &cell.link_url {
                        changed.push_str(&format!("\x1b]8;;{url}\x1b\\"));
                    }
                    changed.push(cell.ch);
                    if cell.link_url.is_some() {
                        changed.push_str("\x1b]8;;\x1b\\");
                    }
                }
                if !changed.is_empty() || cursor_changed {
                    changed.push_str("\x1b[0m");
                    if let Some((cx, cy)) = cursor_position {
                        changed.push_str(&format!("\x1b[{};{}H\x1b[?25h", cy + 1, cx + 1));
                    } else if cursor_changed {
                        changed.push_str("\x1b[?25l");
                    }
                }
                return changed;
            }
        }

        // Initial frame, or a forced redraw after a dropped queued update.
        let mut out = String::with_capacity((self.cols as usize) * (self.rows as usize) * 8);
        out.push_str("\x1b[?25l"); // Hide cursor during draw
        out.push_str("\x1b[H"); // Move cursor to (1,1)

        let mut current_fg = [0u8, 0, 0];
        let mut current_bg = [0u8, 0, 0];
        let mut current_bold = false;
        let mut current_underline = false;
        let mut current_url: Option<String> = None;
        let mut first = true;

        for r in 0..self.rows {
            for c in 0..self.cols {
                let idx = (r as usize) * (self.cols as usize) + (c as usize);
                let cell = &self.grid[idx];

                // Check attribute changes
                let need_sgr = first
                    || cell.fg_rgb != current_fg
                    || cell.bg_rgb != current_bg
                    || cell.bold != current_bold
                    || cell.underline != current_underline;

                if need_sgr {
                    out.push_str("\x1b[0;");
                    if cell.bold {
                        out.push_str("1;");
                    }
                    if cell.underline {
                        out.push_str("4;");
                    }
                    out.push_str(&format!(
                        "38;2;{};{};{};48;2;{};{};{}m",
                        cell.fg_rgb[0],
                        cell.fg_rgb[1],
                        cell.fg_rgb[2],
                        cell.bg_rgb[0],
                        cell.bg_rgb[1],
                        cell.bg_rgb[2]
                    ));
                    current_fg = cell.fg_rgb;
                    current_bg = cell.bg_rgb;
                    current_bold = cell.bold;
                    current_underline = cell.underline;
                    first = false;
                }

                // Check hyperlink change (OSC 8)
                if cell.link_url != current_url {
                    if let Some(url) = &cell.link_url {
                        out.push_str(&format!("\x1b]8;;{url}\x1b\\"));
                    } else {
                        out.push_str("\x1b]8;;\x1b\\");
                    }
                    current_url = cell.link_url.clone();
                }

                out.push(cell.ch);
            }

            // Close any open OSC 8 link at the end of the line
            if current_url.is_some() {
                out.push_str("\x1b]8;;\x1b\\");
                current_url = None;
            }

            if r + 1 < self.rows {
                out.push_str("\r\n");
            }
        }

        out.push_str("\x1b[0m"); // Reset all attributes

        // Position cursor if requested
        if let Some(cursor) = &view.cursor {
            if cursor.visible {
                let (cx, cy) = if is_terminal {
                    (
                        (cursor.x / 8).min(self.cols.saturating_sub(1)),
                        (cursor.y / 16).min(self.rows.saturating_sub(1)),
                    )
                } else {
                    self.pixel_to_grid(cursor.x, cursor.y, view.font_mode)
                };
                out.push_str(&format!(
                    "\x1b[{};{}H\x1b[?25h",
                    (cy + 1).min(self.rows),
                    (cx + 1).min(self.cols)
                ));
            }
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pixel_ssh_view::{Color, ColorTheme, PaletteMode, TextStyle};

    #[test]
    fn test_pixel_to_grid_mapping_vga() {
        let renderer = AnsiRenderer::new(80, 25);
        assert_eq!(renderer.pixel_to_grid(16, 0, PaletteMode::Vga), (2, 0)); // Header
        assert_eq!(renderer.pixel_to_grid(16, 16, PaletteMode::Vga), (2, 1)); // Tab bar
        assert_eq!(renderer.pixel_to_grid(16, 32, PaletteMode::Vga), (2, 2)); // Separator line
        assert_eq!(renderer.pixel_to_grid(16, 48, PaletteMode::Vga), (2, 3)); // First content row
        assert_eq!(renderer.pixel_to_grid(16, 64, PaletteMode::Vga), (2, 4)); // Second content row
        assert_eq!(renderer.pixel_to_grid(16, 384, PaletteMode::Vga), (2, 24)); // Status bar
    }

    #[test]
    fn test_pixel_to_grid_mapping_c64() {
        let renderer = AnsiRenderer::new(80, 25);
        // C64 is 40 cols centered in 80: offset_col = 20
        assert_eq!(renderer.pixel_to_grid(0, 0, PaletteMode::C64), (20, 0)); // Header
        assert_eq!(renderer.pixel_to_grid(0, 10, PaletteMode::C64), (20, 1)); // Tab bar
        assert_eq!(renderer.pixel_to_grid(0, 24, PaletteMode::C64), (20, 3)); // Content
    }

    #[test]
    fn test_pixel_to_grid_mapping_zx() {
        let renderer = AnsiRenderer::new(80, 24);
        // ZX is 32 cols centered in 80: offset_col = 24
        assert_eq!(
            renderer.pixel_to_grid(0, 0, PaletteMode::ZxSpectrum),
            (24, 0)
        );
        assert_eq!(
            renderer.pixel_to_grid(0, 10, PaletteMode::ZxSpectrum),
            (24, 1)
        );
    }

    #[test]
    fn test_centering_in_larger_terminal() {
        let renderer = AnsiRenderer::new(120, 35);
        assert_eq!(renderer.pixel_to_grid(16, 0, PaletteMode::Vga), (22, 5));
        assert_eq!(renderer.pixel_to_grid(16, 16, PaletteMode::Vga), (22, 6));
        assert_eq!(renderer.pixel_to_grid(16, 384, PaletteMode::Vga), (22, 29));
    }

    #[test]
    fn test_render_view_output() {
        let mut renderer = AnsiRenderer::new(80, 25);
        let mut view = View::new(640, 400);
        view.font_mode = PaletteMode::Vga;
        view.add(Element::Text(pixel_ssh_view::TextElement {
            x: 16,
            y: 4,
            text: "TEST TITLE".to_string(),
            style: TextStyle::new(Color::from_palette(6)),
        }));

        let out = renderer.render_view(&view);
        assert!(out.contains("TEST TITLE"));
        assert!(out.contains("\x1b[?25l"));
    }

    #[test]
    fn test_ansi_renderer_sends_only_changed_cells() {
        let mut renderer = AnsiRenderer::new(80, 25);
        let mut view = View::new(640, 400);
        view.platform = Platform::Terminal;
        view.add(Element::Text(pixel_ssh_view::TextElement {
            x: 8,
            y: 0,
            text: "A".to_string(),
            style: TextStyle::new(Color::from_palette(6)),
        }));
        let full = renderer.render_view(&view);
        assert!(full.len() > 1000);
        assert!(renderer.render_view(&view).is_empty());

        if let Element::Text(text) = &mut view.elements[0] {
            text.text = "B".to_string();
        }
        let changed = renderer.render_view(&view);
        assert!(changed.contains("\x1b[1;2H"));
        assert!(changed.contains('B'));
        assert!(changed.len() < full.len() / 10);

        renderer.force_full_redraw();
        assert!(renderer.render_view(&view).len() > 1000);
    }

    #[test]
    fn test_terminal_mode_fullscreen_expansion() {
        let cols = 120;
        let rows = 40;
        let mut renderer = AnsiRenderer::new(cols, rows);
        let mut view = View::new(cols * 8, rows * 16);
        view.platform = Platform::Terminal;
        view.font_mode = PaletteMode::Vga;

        // Add a horizontal rule spanning the entire view
        view.add(Element::Rect(pixel_ssh_view::RectElement {
            x: 0,
            y: 32, // row 2
            width: cols * 8,
            height: 1,
            color: Color::from_palette(7),
            filled: true,
        }));

        renderer.render_view(&view);

        // Row 2 should be filled with '─' from col 0 to cols - 1
        for col in 0..cols {
            let idx = (2 * cols + col) as usize;
            assert_eq!(renderer.grid[idx].ch, '─', "Expected '─' at col {col}");
        }
    }

    #[test]
    fn test_render_sprite_to_ansi_grid() {
        let cols = 80;
        let rows = 25;
        let mut renderer = AnsiRenderer::new(cols, rows);
        let mut view = View::new(cols * 8, rows * 16);
        view.platform = Platform::Terminal;
        view.font_mode = PaletteMode::Vga;
        view.color_palette = ColorTheme::VgaModern;

        // Add a 16x16 sprite at x=0, y=0
        let data = vec![6u8; 16 * 16]; // white
        view.add(Element::Sprite(pixel_ssh_view::SpriteElement {
            x: 0,
            y: 0,
            width: 16,
            height: 16,
            data,
        }));

        renderer.render_view(&view);

        // Terminal cell (0, 0) should have '▀' half block
        assert_eq!(renderer.grid[0].ch, '▀');
        let white_rgb = [240, 246, 252]; // palette 6 in VGA
        assert_eq!(renderer.grid[0].fg_rgb, white_rgb);
        assert_eq!(renderer.grid[0].bg_rgb, white_rgb);
    }

    #[test]
    fn test_table_border_chars_have_uniform_color() {
        let mut renderer = AnsiRenderer::new(80, 25);
        let mut view = View::new(640, 400);
        view.platform = Platform::Terminal;
        view.font_mode = PaletteMode::Vga;

        // Even if text element has bold white color (palette 6), table border chars MUST use border_rgb (palette 7)
        view.add(Element::Text(pixel_ssh_view::TextElement {
            x: 0,
            y: 48, // row 3
            text: "│ Cell text │".to_string(),
            style: TextStyle::new(Color::from_palette(6)),
        }));

        renderer.render_view(&view);

        let pal = Framebuffer::palette_for_palette(view.color_palette);
        let border_rgb = [pal[7][0], pal[7][1], pal[7][2]];
        let text_rgb = [pal[6][0], pal[6][1], pal[6][2]];

        let idx_pipe_0 = 3 * 80;
        let idx_pipe_1 = 3 * 80 + 12;
        let idx_char = 3 * 80 + 2; // 'C'

        assert_eq!(renderer.grid[idx_pipe_0].ch, '│');
        assert_eq!(renderer.grid[idx_pipe_0].fg_rgb, border_rgb);

        assert_eq!(renderer.grid[idx_pipe_1].ch, '│');
        assert_eq!(renderer.grid[idx_pipe_1].fg_rgb, border_rgb);

        assert_eq!(renderer.grid[idx_char].ch, 'C');
        assert_eq!(renderer.grid[idx_char].fg_rgb, text_rgb);
    }
}
