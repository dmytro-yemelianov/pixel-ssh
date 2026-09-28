pub mod font;

use font::{FONT_8X8, FONT_HEIGHT, FONT_WIDTH};
use pixel_ssh_view::{Element, View};

pub const DEFAULT_WIDTH: u16 = 320;
pub const DEFAULT_HEIGHT: u16 = 200;

#[derive(Debug, Clone)]
pub struct Framebuffer {
    pub width: u16,
    pub height: u16,
    pub pixels: Vec<u8>,
    pub palette: [[u8; 4]; 256],
}

impl Default for Framebuffer {
    fn default() -> Self {
        Self::new(DEFAULT_WIDTH, DEFAULT_HEIGHT)
    }
}

impl Framebuffer {
    pub fn new(width: u16, height: u16) -> Self {
        let size = (width as usize) * (height as usize);
        Self {
            width,
            height,
            pixels: vec![0; size],
            palette: Self::default_palette(),
        }
    }

    pub fn default_palette() -> [[u8; 4]; 256] {
        let mut p = [[0, 0, 0, 255]; 256];

        // Specific UI palette
        p[0] = [13, 17, 23, 255];     // 0: #0d1117 (Dark background)
        p[1] = [22, 27, 34, 255];     // 1: #161b22 (Card header)
        p[2] = [33, 38, 45, 255];     // 2: #21262d (Selected item background)
        p[3] = [48, 54, 61, 255];     // 3: #30363d (Border gray)
        p[4] = [139, 148, 158, 255];  // 4: #8b949e (Muted gray)
        p[5] = [201, 209, 217, 255];  // 5: #c9d1d9 (Light text body)
        p[6] = [240, 246, 252, 255];  // 6: #f0f6fc (Bright white text)
        p[7] = [88, 166, 255, 255];   // 7: #58a6ff (Accent blue)
        p[8] = [63, 185, 80, 255];    // 8: #3fb950 (Terminal green)
        p[9] = [210, 153, 34, 255];   // 9: #d29922 (Warning yellow)
        p[10] = [248, 81, 73, 255];   // 10: #f85149 (Danger/WIP red)
        p[11] = [188, 140, 255, 255]; // 11: #bc8cff (Purple)
        p[12] = [57, 197, 187, 255];  // 12: #39c5bb (Cyan)
        p[13] = [0, 0, 0, 255];       // 13: #000000 (Pure black)
        p[14] = [255, 255, 255, 255]; // 14: #ffffff (Pure white)

        // Fill remaining entries with standard 6x6x6 color cube + grayscale ramp
        let mut idx = 16;
        for r in 0..6 {
            for g in 0..6 {
                for b in 0..6 {
                    if idx < 232 {
                        let rv = if r == 0 { 0 } else { (r * 40 + 55) as u8 };
                        let gv = if g == 0 { 0 } else { (g * 40 + 55) as u8 };
                        let bv = if b == 0 { 0 } else { (b * 40 + 55) as u8 };
                        p[idx] = [rv, gv, bv, 255];
                        idx += 1;
                    }
                }
            }
        }
        for gray in 0..24 {
            if idx < 256 {
                let v = (gray * 10 + 8) as u8;
                p[idx] = [v, v, v, 255];
                idx += 1;
            }
        }

        p
    }

    pub fn clear(&mut self, color_index: u8) {
        self.pixels.fill(color_index);
    }

    #[inline]
    pub fn set_pixel(&mut self, x: u16, y: u16, color_index: u8) {
        if x < self.width && y < self.height {
            let idx = (y as usize) * (self.width as usize) + (x as usize);
            self.pixels[idx] = color_index;
        }
    }

    pub fn fill_rect(&mut self, x: u16, y: u16, w: u16, h: u16, color_index: u8) {
        let x_end = (x + w).min(self.width);
        let y_end = (y + h).min(self.height);

        for cy in y..y_end {
            let row_start = (cy as usize) * (self.width as usize);
            for cx in x..x_end {
                self.pixels[row_start + (cx as usize)] = color_index;
            }
        }
    }

    pub fn draw_rect(&mut self, x: u16, y: u16, w: u16, h: u16, color_index: u8) {
        if w == 0 || h == 0 { return; }
        self.fill_rect(x, y, w, 1, color_index);
        self.fill_rect(x, y + h - 1, w, 1, color_index);
        self.fill_rect(x, y, 1, h, color_index);
        self.fill_rect(x + w - 1, y, 1, h, color_index);
    }

    pub fn draw_char(&mut self, x: u16, y: u16, ch: char, fg: u8, bg: Option<u8>) {
        let char_code = (ch as usize).min(127);
        let glyph = FONT_8X8[char_code];

        for row in 0..FONT_HEIGHT {
            let py = y + row;
            if py >= self.height { break; }
            let byte = glyph[row as usize];
            let row_start = (py as usize) * (self.width as usize);

            for col in 0..FONT_WIDTH {
                let px = x + col;
                if px >= self.width { break; }

                // font8x8 has bit 0 on the left in its encoding
                let is_on = (byte & (1 << col)) != 0;
                let pixel_idx = row_start + (px as usize);

                if is_on {
                    self.pixels[pixel_idx] = fg;
                } else if let Some(bg_color) = bg {
                    self.pixels[pixel_idx] = bg_color;
                }
            }
        }
    }

    pub fn draw_text(&mut self, mut x: u16, y: u16, text: &str, fg: u8, bg: Option<u8>) {
        for ch in text.chars() {
            if x + FONT_WIDTH > self.width {
                break;
            }
            self.draw_char(x, y, ch, fg, bg);
            x += FONT_WIDTH;
        }
    }

    /// Renders a platform-independent View into this indexed framebuffer.
    pub fn draw_view(&mut self, view: &View) {
        self.clear(0); // clear to background

        for element in &view.elements {
            match element {
                Element::Rect(r) => {
                    if r.filled {
                        self.fill_rect(r.x, r.y, r.width, r.height, r.color.palette_index);
                    } else {
                        self.draw_rect(r.x, r.y, r.width, r.height, r.color.palette_index);
                    }
                }
                Element::Text(t) => {
                    let bg = t.style.bg.map(|c| c.palette_index);
                    self.draw_text(t.x, t.y, &t.text, t.style.fg.palette_index, bg);
                }
                Element::Sprite(s) => {
                    let mut i = 0;
                    for sy in 0..s.height {
                        let py = s.y + sy;
                        if py >= self.height { break; }
                        for sx in 0..s.width {
                            let px = s.x + sx;
                            if i < s.data.len() && px < self.width {
                                self.set_pixel(px, py, s.data[i]);
                            }
                            i += 1;
                        }
                    }
                }
            }
        }

        // Draw cursor if visible
        if let Some(cursor) = &view.cursor {
            if cursor.visible && cursor.x < self.width && cursor.y < self.height {
                self.fill_rect(cursor.x, cursor.y, 6, 8, 8); // Green block cursor
            }
        }
    }
}
