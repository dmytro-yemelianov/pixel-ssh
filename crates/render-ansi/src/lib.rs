use pixel_ssh_framebuffer::Framebuffer;
use pixel_ssh_view::{Element, View};

#[derive(Debug, Clone)]
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
}

impl AnsiRenderer {
    pub fn new(cols: u16, rows: u16) -> Self {
        let size = (cols as usize) * (rows as usize);
        Self {
            cols,
            rows,
            grid: vec![AnsiCell::default(); size],
        }
    }

    pub fn resize(&mut self, cols: u16, rows: u16) {
        self.cols = cols;
        self.rows = rows;
        let size = (cols as usize) * (rows as usize);
        self.grid = vec![AnsiCell::default(); size];
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
                if y < 24 {
                    0 // Header bar
                } else if y < 46 {
                    1 // Tab bar
                } else if y < 54 {
                    2 // Tab separator line
                } else if y >= 374 {
                    24 // Status footer bar
                } else {
                    3 + ((y.saturating_sub(54)) / 16).min(20)
                }
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
        let palette = Framebuffer::palette_for_mode(view.palette_mode);
        let default_bg = [palette[0][0], palette[0][1], palette[0][2]];
        self.clear(default_bg);

        let (target_cols, _target_rows) = view.palette_mode.char_grid();
        let offset_col = self.cols.saturating_sub(target_cols) / 2;

        for elem in &view.elements {
            match elem {
                Element::Rect(r) => {
                    let pal_rgb = palette[r.color.palette_index as usize];
                    let rgb = [pal_rgb[0], pal_rgb[1], pal_rgb[2]];

                    // Check if it's a thin horizontal separator rule
                    if r.height <= 2 {
                        let (rx_start, ry) = self.pixel_to_grid(r.x, r.y, view.palette_mode);
                        let rx_end = offset_col + ((r.x + r.width + 7) / 8);
                        if ry < self.rows {
                            for x in rx_start..rx_end.min(self.cols) {
                                let idx = (ry as usize) * (self.cols as usize) + (x as usize);
                                if idx < self.grid.len() && self.grid[idx].ch == ' ' {
                                    self.grid[idx].ch = '─';
                                    self.grid[idx].fg_rgb = rgb;
                                }
                            }
                        }
                    } else {
                        // Background rect (header, tabs, active card, or status bar)
                        let (rx_start, ry_start) = self.pixel_to_grid(r.x, r.y, view.palette_mode);
                        let (rx_end, ry_end) =
                            self.pixel_to_grid(r.x + r.width, r.y + r.height, view.palette_mode);

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
                    let (mut start_col, row) = self.pixel_to_grid(t.x, t.y, view.palette_mode);

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
                                self.grid[idx].fg_rgb = fg_rgb;
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
                    let (mut start_col, row) = self.pixel_to_grid(l.x, l.y, view.palette_mode);

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
                                self.grid[idx].fg_rgb = fg_rgb;
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
                Element::Sprite(_) => {}
            }
        }

        // Generate optimized ANSI string output with cursor home and differential 24-bit color
        let mut out = String::with_capacity((self.cols as usize) * (self.rows as usize) * 8);
        out.push_str("\x1b[?25l"); // Hide cursor during draw
        out.push_str("\x1b[H");    // Move cursor to (1,1)

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
                        cell.fg_rgb[0], cell.fg_rgb[1], cell.fg_rgb[2],
                        cell.bg_rgb[0], cell.bg_rgb[1], cell.bg_rgb[2]
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
                        out.push_str(&format!("\x1b]8;;{}\x1b\\", url));
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
                let (cx, cy) = self.pixel_to_grid(cursor.x, cursor.y, view.palette_mode);
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
    use pixel_ssh_view::{Color, PaletteMode, TextStyle};

    #[test]
    fn test_pixel_to_grid_mapping_vga() {
        let renderer = AnsiRenderer::new(80, 25);
        assert_eq!(renderer.pixel_to_grid(16, 4, PaletteMode::Vga), (2, 0)); // Header
        assert_eq!(renderer.pixel_to_grid(16, 28, PaletteMode::Vga), (2, 1)); // Tab bar
        assert_eq!(renderer.pixel_to_grid(16, 49, PaletteMode::Vga), (2, 2)); // Separator line
        assert_eq!(renderer.pixel_to_grid(16, 58, PaletteMode::Vga), (2, 3)); // First content row
        assert_eq!(renderer.pixel_to_grid(16, 74, PaletteMode::Vga), (2, 4)); // Second content row
        assert_eq!(renderer.pixel_to_grid(16, 382, PaletteMode::Vga), (2, 24)); // Status bar
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
        assert_eq!(renderer.pixel_to_grid(0, 0, PaletteMode::ZxSpectrum), (24, 0));
        assert_eq!(renderer.pixel_to_grid(0, 10, PaletteMode::ZxSpectrum), (24, 1));
    }

    #[test]
    fn test_centering_in_larger_terminal() {
        let renderer = AnsiRenderer::new(120, 35);
        assert_eq!(renderer.pixel_to_grid(16, 4, PaletteMode::Vga), (22, 5));
        assert_eq!(renderer.pixel_to_grid(16, 28, PaletteMode::Vga), (22, 6));
        assert_eq!(renderer.pixel_to_grid(16, 382, PaletteMode::Vga), (22, 29));
    }

    #[test]
    fn test_render_view_output() {
        let mut renderer = AnsiRenderer::new(80, 25);
        let mut view = View::new(640, 400);
        view.palette_mode = PaletteMode::Vga;
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
}

