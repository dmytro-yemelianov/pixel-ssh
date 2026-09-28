use pixel_ssh_view::{Element, View};

#[derive(Debug, Clone, Copy)]
pub struct AnsiCell {
    pub ch: char,
    pub fg_index: u8,
    pub bg_index: u8,
    pub bold: bool,
}

impl Default for AnsiCell {
    fn default() -> Self {
        Self {
            ch: ' ',
            fg_index: 7, // Default white/light gray
            bg_index: 0, // Default black
            bold: false,
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

    pub fn clear(&mut self) {
        self.grid.fill(AnsiCell::default());
    }

    /// Sanitizes untrusted text by stripping all ANSI/escape control characters.
    pub fn sanitize_text(input: &str) -> String {
        input
            .chars()
            .filter(|&c| c >= ' ' && c != '\x7f' && c != '\x1b')
            .collect()
    }

    /// Maps palette index 0..15 to standard terminal colors or 256-color codes.
    pub fn color_to_ansi_fg(idx: u8) -> u8 {
        match idx {
            0 => 234,  // dark bg
            1 => 235,  // card
            2 => 237,  // sel card
            3 => 240,  // border
            4 => 245,  // muted
            5 => 252,  // body text
            6 => 255,  // bright white
            7 => 75,   // blue
            8 => 77,   // green
            9 => 178,  // yellow
            10 => 203, // red
            11 => 141, // purple
            12 => 44,  // cyan
            13 => 0,   // black
            14 => 15,  // white
            other => other,
        }
    }

    pub fn color_to_ansi_bg(idx: u8) -> u8 {
        Self::color_to_ansi_fg(idx)
    }

    /// Renders a platform-independent View into an ANSI string stream.
    pub fn render_view(&mut self, view: &View) -> String {
        self.clear();

        // Logical scale: View is 320x200.
        // Screen text cols: 40 cols (320 / 8) -> scaled to terminal cols (e.g. 80 cols)
        let col_scale = self.cols as f32 / view.width as f32;
        let row_scale = self.rows as f32 / view.height as f32;

        for elem in &view.elements {
            match elem {
                Element::Rect(r) => {
                    let rx_start = ((r.x as f32) * col_scale) as u16;
                    let ry_start = ((r.y as f32) * row_scale) as u16;
                    let rx_end = (((r.x + r.width) as f32) * col_scale) as u16;
                    let ry_end = (((r.y + r.height) as f32) * row_scale) as u16;

                    let bg = Self::color_to_ansi_bg(r.color.palette_index);

                    for y in ry_start..ry_end.min(self.rows) {
                        for x in rx_start..rx_end.min(self.cols) {
                            let idx = (y as usize) * (self.cols as usize) + (x as usize);
                            if idx < self.grid.len() {
                                self.grid[idx].bg_index = bg;
                            }
                        }
                    }
                }
                Element::Text(t) => {
                    let sanitized = Self::sanitize_text(&t.text);
                    let mut start_col = ((t.x as f32) * col_scale) as u16;
                    let row = ((t.y as f32) * row_scale) as u16;

                    let fg = Self::color_to_ansi_fg(t.style.fg.palette_index);
                    let bg = t.style.bg.map(|c| Self::color_to_ansi_bg(c.palette_index));

                    for ch in sanitized.chars() {
                        if start_col >= self.cols || row >= self.rows {
                            break;
                        }
                        let idx = (row as usize) * (self.cols as usize) + (start_col as usize);
                        if idx < self.grid.len() {
                            self.grid[idx].ch = ch;
                            self.grid[idx].fg_index = fg;
                            if let Some(bg_color) = bg {
                                self.grid[idx].bg_index = bg_color;
                            }
                            self.grid[idx].bold = t.style.bold;
                        }
                        start_col += 1;
                    }
                }
                Element::Sprite(_) => {}
            }
        }

        // Generate optimized ANSI string output with cursor home and differential formatting
        let mut out = String::with_capacity((self.cols as usize) * (self.rows as usize) * 4);
        out.push_str("\x1b[?25l"); // Hide cursor during draw
        out.push_str("\x1b[H");    // Move cursor to (1,1)

        let mut current_fg = 255;
        let mut current_bg = 255;
        let mut current_bold = false;

        for r in 0..self.rows {
            for c in 0..self.cols {
                let idx = (r as usize) * (self.cols as usize) + (c as usize);
                let cell = self.grid[idx];

                if cell.fg_index != current_fg || cell.bg_index != current_bg || cell.bold != current_bold {
                    out.push_str("\x1b[0;");
                    if cell.bold {
                        out.push_str("1;");
                    }
                    out.push_str(&format!("38;5;{};48;5;{}m", cell.fg_index, cell.bg_index));
                    current_fg = cell.fg_index;
                    current_bg = cell.bg_index;
                    current_bold = cell.bold;
                }
                out.push(cell.ch);
            }
            if r + 1 < self.rows {
                out.push_str("\r\n");
            }
        }

        out.push_str("\x1b[0m"); // Reset attributes

        // Position cursor if requested
        if let Some(cursor) = &view.cursor {
            if cursor.visible {
                let cx = ((cursor.x as f32) * col_scale) as u16 + 1;
                let cy = ((cursor.y as f32) * row_scale) as u16 + 1;
                out.push_str(&format!("\x1b[{};{}H\x1b[?25h", cy.min(self.rows), cx.min(self.cols)));
            }
        }

        out
    }
}
