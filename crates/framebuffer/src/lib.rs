pub mod font;

use font::{CP437_FONT_8X8, FONT_HEIGHT, FONT_WIDTH, unicode_to_cp437};
use pixel_ssh_view::{Element, PaletteMode, View};

pub const DEFAULT_WIDTH: u16 = 640;
pub const DEFAULT_HEIGHT: u16 = 400;

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
        Self::palette_for_mode(PaletteMode::default())
    }

    pub fn palette_for_mode(mode: PaletteMode) -> [[u8; 4]; 256] {
        let mut p = [[0, 0, 0, 255]; 256];

        match mode {
            PaletteMode::Vga => {
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
            }
            PaletteMode::ZxSpectrum => {
                p[0] = [0, 0, 0, 255];        // Black bg
                p[1] = [0, 0, 192, 255];      // Blue header
                p[2] = [0, 0, 255, 255];      // Bright blue selection
                p[3] = [0, 192, 192, 255];    // Cyan border
                p[4] = [192, 192, 192, 255];  // White / muted
                p[5] = [255, 255, 0, 255];    // Bright yellow body text (iconic Spectrum look)
                p[6] = [255, 255, 255, 255];  // Bright white
                p[7] = [0, 255, 255, 255];    // Bright cyan accent
                p[8] = [0, 255, 0, 255];      // Bright green
                p[9] = [255, 255, 0, 255];    // Bright yellow
                p[10] = [255, 0, 0, 255];     // Bright red
                p[11] = [255, 0, 255, 255];   // Bright magenta
                p[12] = [0, 255, 255, 255];   // Bright cyan
                p[13] = [0, 0, 0, 255];
                p[14] = [255, 255, 255, 255];
            }
            PaletteMode::C64 => {
                p[0] = [64, 49, 141, 255];    // C64 deep blue background (#40318D)
                p[1] = [108, 94, 181, 255];   // Header #6C5EB5
                p[2] = [134, 122, 222, 255];  // Selected item #867ADE
                p[3] = [134, 122, 222, 255];  // Border #867ADE
                p[4] = [187, 187, 187, 255];  // Light gray #BBBBBB
                p[5] = [170, 255, 238, 255];  // Light cyan #AAFFEE body text
                p[6] = [255, 255, 255, 255];  // White
                p[7] = [238, 238, 119, 255];  // Yellow #EEEE77
                p[8] = [170, 255, 102, 255];  // Light green #AAFF66
                p[9] = [238, 238, 119, 255];  // Yellow
                p[10] = [255, 119, 119, 255]; // Light red #FF7777
                p[11] = [204, 68, 204, 255];  // Purple #CC44CC
                p[12] = [170, 255, 238, 255]; // Light cyan #AAFFEE
                p[13] = [0, 0, 0, 255];
                p[14] = [255, 255, 255, 255];
            }
            PaletteMode::Atari => {
                p[0] = [24, 14, 4, 255];      // Deep dark umber
                p[1] = [56, 36, 12, 255];     // Dark bronze
                p[2] = [90, 60, 20, 255];     // Warm leather
                p[3] = [138, 96, 32, 255];    // Copper border
                p[4] = [184, 144, 80, 255];   // Gold-tan muted
                p[5] = [230, 200, 128, 255];  // Warm wheat body
                p[6] = [255, 240, 176, 255];  // Luminous gold
                p[7] = [0, 216, 200, 255];    // Atari turquoise/teal
                p[8] = [96, 208, 64, 255];    // Retro phosphor lime
                p[9] = [255, 208, 32, 255];   // Sunburst yellow
                p[10] = [232, 64, 48, 255];   // Burnt crimson
                p[11] = [192, 80, 208, 255];  // Atari orchid
                p[12] = [48, 224, 224, 255];  // Bright teal
                p[13] = [0, 0, 0, 255];
                p[14] = [255, 255, 255, 255];
            }
            PaletteMode::Amber => {
                p[0] = [18, 9, 0, 255];       // Phosphor dark
                p[1] = [42, 21, 0, 255];      // Header shade
                p[2] = [72, 36, 0, 255];      // Selection shade
                p[3] = [110, 55, 0, 255];     // Border
                p[4] = [160, 80, 0, 255];     // Muted amber
                p[5] = [214, 122, 0, 255];    // Warm amber body
                p[6] = [255, 170, 0, 255];    // Full intensity beam
                p[7] = [255, 195, 50, 255];   // Luminous amber glow
                p[8] = [240, 150, 0, 255];
                p[9] = [255, 180, 0, 255];
                p[10] = [180, 60, 0, 255];
                p[11] = [210, 100, 0, 255];
                p[12] = [255, 195, 50, 255];
                p[13] = [0, 0, 0, 255];
                p[14] = [255, 200, 50, 255];
            }
            PaletteMode::GreenCrt => {
                p[0] = [3, 16, 5, 255];       // Deep tube green-black
                p[1] = [8, 38, 13, 255];      // Header shade
                p[2] = [15, 66, 24, 255];     // Selection shade
                p[3] = [25, 99, 37, 255];     // Border
                p[4] = [40, 140, 58, 255];    // Muted phosphor
                p[5] = [56, 201, 84, 255];    // P1 phosphor green body
                p[6] = [85, 255, 119, 255];   // Full intensity green
                p[7] = [120, 255, 170, 255];  // Peak highlight
                p[8] = [70, 255, 100, 255];
                p[9] = [160, 255, 80, 255];
                p[10] = [40, 120, 30, 255];
                p[11] = [50, 170, 70, 255];
                p[12] = [120, 255, 170, 255];
                p[13] = [0, 0, 0, 255];
                p[14] = [160, 255, 180, 255];
            }
        }

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

    pub fn set_palette_mode(&mut self, mode: PaletteMode) {
        self.palette = Self::palette_for_mode(mode);
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
        let code = unicode_to_cp437(ch);
        let glyph = CP437_FONT_8X8[code as usize];

        for row in 0..FONT_HEIGHT {
            let py = y + row;
            if py >= self.height { break; }
            let byte = glyph[row as usize];
            let row_start = (py as usize) * (self.width as usize);

            for col in 0..FONT_WIDTH {
                let px = x + col;
                if px >= self.width { break; }

                // IBM EGA MSB-left: bit 7 is leftmost
                let is_on = (byte & (0x80 >> col)) != 0;
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

    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.pixels.resize((width as usize) * (height as usize), 0);
    }

    /// Renders a platform-independent View into this indexed framebuffer.
    pub fn draw_view(&mut self, view: &View) {
        if self.width != view.width || self.height != view.height {
            self.resize(view.width, view.height);
        }
        self.set_palette_mode(view.palette_mode);
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
                Element::Link(l) => {
                    let bg = l.style.bg.map(|c| c.palette_index);
                    self.draw_text(l.x, l.y, &l.text, l.style.fg.palette_index, bg);
                    if l.style.underline && l.y + 8 < self.height {
                        self.fill_rect(l.x, l.y + 8, l.width, 1, l.style.fg.palette_index);
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
