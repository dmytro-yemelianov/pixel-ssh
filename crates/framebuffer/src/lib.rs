pub mod font;

use font::{font_for_mode, font_8x16_for_mode, FONT_HEIGHT, FONT_WIDTH, unicode_to_cp437};
use pixel_ssh_view::{is_table_border_char, ColorTheme, Element, PaletteMode, View};

pub const DEFAULT_WIDTH: u16 = 640;
pub const DEFAULT_HEIGHT: u16 = 400;

#[derive(Debug, Clone)]
pub struct Framebuffer {
    pub width: u16,
    pub height: u16,
    pub pixels: Vec<u8>,
    pub palette: [[u8; 4]; 256],
    pub font_mode: PaletteMode,
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
            font_mode: PaletteMode::default(),
        }
    }

    pub fn default_palette() -> [[u8; 4]; 256] {
        Self::palette_for_theme(ColorTheme::default())
    }

    pub fn palette_for_mode(mode: PaletteMode) -> [[u8; 4]; 256] {
        Self::palette_for_theme(mode.to_theme())
    }

    pub fn palette_for_theme(theme: ColorTheme) -> [[u8; 4]; 256] {
        let mut p = [[0, 0, 0, 255]; 256];

        match theme {
            ColorTheme::Commander => {
                // Volkov Commander / Norton Commander classic DOS palette
                p[0] = [0, 0, 168, 255];     // 0: #0000a8 (Iconic Commander navy blue background)
                p[1] = [0, 168, 168, 255];   // 1: #00aaaa (Cyan header bar / accent)
                p[2] = [0, 168, 168, 255];   // 2: #00aaaa (Selected item cyan background bar)
                p[3] = [0, 168, 168, 255];   // 3: #00aaaa (Cyan table / panel border)
                p[4] = [85, 255, 255, 255];  // 4: #55ffff (Bright cyan muted/secondary)
                p[5] = [255, 255, 255, 255]; // 5: #ffffff (Bright white text body)
                p[6] = [255, 255, 85, 255];  // 6: #ffff55 (Bright yellow titles and headers)
                p[7] = [85, 255, 255, 255];  // 7: #55ffff (Bright cyan links/accents)
                p[8] = [85, 255, 85, 255];   // 8: #55ff55 (Bright green)
                p[9] = [255, 255, 85, 255];  // 9: #ffff55 (Bright yellow warning)
                p[10] = [255, 85, 85, 255];  // 10: #ff5555 (Bright red danger)
                p[11] = [255, 85, 255, 255]; // 11: #ff55ff (Bright magenta)
                p[12] = [0, 168, 168, 255];  // 12: #00aaaa (Dark cyan)
                p[13] = [0, 0, 0, 255];      // 13: #000000 (Pure black, shadows & clock background)
                p[14] = [255, 255, 255, 255];// 14: #ffffff (Pure white)
                p[15] = [168, 168, 168, 255];// 15: #aaaaaa (Dialog box light gray)

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
            }
            ColorTheme::VgaModern => {
                p[0] = [13, 17, 23, 255];     // 0: #0d1117 (Dark background)
                p[1] = [22, 27, 34, 255];     // 1: #161b22 (Card header)
                p[2] = [33, 38, 45, 255];     // 2: #21262d (Selected item background)
                p[3] = [88, 96, 105, 255];    // 3: #586069 (Visible medium border gray)
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
                p[15] = [80, 80, 80, 255];

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
            }
            ColorTheme::Ega => {
                // Classic IBM EGA / CGA 16-color RGBI hardware palette
                p[0] = [0, 0, 0, 255];        // Black
                p[1] = [0, 0, 170, 255];      // Blue
                p[2] = [0, 170, 0, 255];      // Green
                p[3] = [0, 170, 170, 255];    // Cyan border
                p[4] = [170, 170, 170, 255];  // Light Gray / Muted
                p[5] = [255, 255, 255, 255];  // White body text
                p[6] = [255, 255, 255, 255];  // Bright white
                p[7] = [85, 255, 255, 255];   // Bright Cyan accent
                p[8] = [85, 255, 85, 255];    // Bright Green
                p[9] = [255, 255, 85, 255];   // Bright Yellow
                p[10] = [255, 85, 85, 255];   // Bright Red
                p[11] = [255, 85, 255, 255];  // Bright Magenta
                p[12] = [85, 255, 255, 255];  // Bright Cyan
                p[13] = [0, 0, 0, 255];       // Pure black
                p[14] = [255, 255, 255, 255]; // Pure white
                p[15] = [85, 85, 85, 255];     // Dark Gray

                for idx in 16..256 {
                    p[idx] = p[idx % 16];
                }
            }
            ColorTheme::ZxSpectrum => {
                // Official Sinclair ZX Spectrum 16-color palette (8 normal + 8 bright)
                p[0] = [0, 0, 0, 255];        // Black bg
                p[1] = [0, 0, 192, 255];      // Blue header / dither base
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
                p[13] = [0, 0, 0, 255];       // Pure black
                p[14] = [255, 255, 255, 255]; // Pure white
                p[15] = [192, 0, 0, 255];     // Red

                for idx in 16..256 {
                    p[idx] = p[idx % 16];
                }
            }
            ColorTheme::C64 => {
                // Strict Commodore 64 VIC-II hardware palette (Pepto / Colodore standard)
                p[0] = [64, 49, 141, 255];    // VIC-II #6 Blue background (#40318D)
                p[1] = [51, 51, 51, 255];     // VIC-II #11 Dark Grey (card dither base)
                p[2] = [112, 109, 235, 255];  // VIC-II #14 Light Blue (selection dither base)
                p[3] = [187, 187, 187, 255];  // VIC-II #15 Light Grey border
                p[4] = [119, 119, 119, 255];  // VIC-II #12 Medium Grey muted
                p[5] = [170, 255, 238, 255];  // VIC-II #3 Light Cyan body text
                p[6] = [255, 255, 255, 255];  // VIC-II #1 White header/accent
                p[7] = [238, 238, 119, 255];  // VIC-II #7 Yellow accent
                p[8] = [170, 255, 102, 255];  // VIC-II #13 Light Green
                p[9] = [221, 136, 85, 255];   // VIC-II #8 Orange
                p[10] = [255, 119, 119, 255]; // VIC-II #10 Light Red
                p[11] = [204, 68, 204, 255];  // VIC-II #4 Purple / Violet
                p[12] = [112, 109, 235, 255]; // VIC-II #14 Light Blue
                p[13] = [0, 0, 0, 255];       // VIC-II #0 Black
                p[14] = [255, 255, 255, 255]; // VIC-II #1 White
                p[15] = [102, 68, 0, 255];    // VIC-II #9 Brown

                for idx in 16..256 {
                    p[idx] = p[idx % 16];
                }
            }
            ColorTheme::Atari => {
                // Strict Atari 800 GTIA hardware color palette
                p[0] = [0, 0, 0, 255];        // GTIA Deep dark umber / black
                p[1] = [64, 40, 16, 255];     // GTIA Dark bronze (card dither base)
                p[2] = [144, 96, 24, 255];    // GTIA Gold selection (dither base)
                p[3] = [160, 96, 32, 255];    // GTIA Copper border
                p[4] = [192, 144, 64, 255];   // GTIA Gold-tan muted
                p[5] = [230, 200, 128, 255];  // GTIA Warm wheat body
                p[6] = [255, 240, 176, 255];  // GTIA Luminous gold text
                p[7] = [0, 208, 208, 255];    // GTIA Turquoise/teal accent
                p[8] = [96, 208, 64, 255];    // GTIA Lime green
                p[9] = [248, 208, 32, 255];   // GTIA Sunburst yellow
                p[10] = [224, 64, 48, 255];   // GTIA Burnt crimson
                p[11] = [192, 80, 208, 255];  // GTIA Orchid purple
                p[12] = [48, 224, 224, 255];  // GTIA Bright teal
                p[13] = [0, 0, 0, 255];       // GTIA Pure black
                p[14] = [255, 255, 255, 255]; // GTIA Pure white
                p[15] = [48, 24, 8, 255];     // GTIA Deep umber

                for idx in 16..256 {
                    let hue = (idx - 16) / 16;
                    let lum = ((idx - 16) % 16) as u8;
                    let lum_val = lum * 17;
                    let (hr, hg, hb) = match hue {
                        0 => (1.0, 1.0, 1.0),
                        1 => (1.0, 0.75, 0.25),
                        2 => (1.0, 0.5, 0.1),
                        3 => (1.0, 0.25, 0.15),
                        4 => (0.9, 0.2, 0.5),
                        5 => (0.7, 0.2, 0.8),
                        6 => (0.4, 0.25, 0.9),
                        7 => (0.2, 0.35, 1.0),
                        8 => (0.1, 0.65, 0.9),
                        9 => (0.0, 0.8, 0.8),
                        10 => (0.1, 0.8, 0.4),
                        11 => (0.35, 0.8, 0.2),
                        12 => (0.55, 0.75, 0.15),
                        13 => (0.85, 0.75, 0.1),
                        14 => (0.95, 0.6, 0.1),
                        _ => (0.6, 0.35, 0.15),
                    };
                    p[idx] = [
                        ((lum_val as f32) * hr).min(255.0) as u8,
                        ((lum_val as f32) * hg).min(255.0) as u8,
                        ((lum_val as f32) * hb).min(255.0) as u8,
                        255,
                    ];
                }
            }
            ColorTheme::Amber => {
                // Strictly monochromatic Amber CRT phosphor (P134/P20 - pure 588nm yellow-orange emission, blue = 0)
                p[0] = [0, 0, 0, 255];        // Tube off
                p[1] = [50, 32, 0, 255];      // Dim amber (card dither base)
                p[2] = [95, 61, 0, 255];      // Medium-low amber (selection dither base)
                p[3] = [115, 74, 0, 255];     // Border
                p[4] = [160, 103, 0, 255];    // Muted amber
                p[5] = [215, 139, 0, 255];    // Normal amber text
                p[6] = [255, 165, 0, 255];    // High intensity beam amber
                p[7] = [255, 185, 0, 255];    // Luminous amber glow
                p[8] = [225, 145, 0, 255];
                p[9] = [245, 158, 0, 255];
                p[10] = [170, 110, 0, 255];
                p[11] = [195, 126, 0, 255];
                p[12] = [235, 152, 0, 255];
                p[13] = [0, 0, 0, 255];
                p[14] = [255, 185, 0, 255];
                p[15] = [130, 84, 0, 255];

                for idx in 16..256 {
                    let v = (((idx - 16) * 255) / 239) as u8;
                    p[idx] = [v, ((v as u16 * 165) / 255) as u8, 0, 255];
                }
            }
            ColorTheme::GreenCrt => {
                // Strictly monochromatic P1 phosphor Green CRT (IBM 5151 / MDA - pure green emission, red = 0, blue = 0)
                p[0] = [0, 0, 0, 255];        // Tube off
                p[1] = [0, 45, 0, 255];       // Dim green (card dither base)
                p[2] = [0, 90, 0, 255];       // Medium-low green (selection dither base)
                p[3] = [0, 105, 0, 255];      // Border
                p[4] = [0, 150, 0, 255];      // Muted green phosphor
                p[5] = [0, 210, 0, 255];      // Normal P1 phosphor green body text
                p[6] = [0, 255, 0, 255];      // High intensity beam green
                p[7] = [0, 240, 0, 255];      // Accent green
                p[8] = [0, 220, 0, 255];
                p[9] = [0, 235, 0, 255];
                p[10] = [0, 160, 0, 255];
                p[11] = [0, 185, 0, 255];
                p[12] = [0, 225, 0, 255];
                p[13] = [0, 0, 0, 255];
                p[14] = [0, 255, 0, 255];
                p[15] = [0, 120, 0, 255];

                for idx in 16..256 {
                    let v = (((idx - 16) * 255) / 239) as u8;
                    p[idx] = [0, v, 0, 255];
                }
            }
        }

        p
    }

    pub fn set_palette_theme(&mut self, theme: ColorTheme) {
        self.palette = Self::palette_for_theme(theme);
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

        match (self.font_mode, color_index) {
            (PaletteMode::GreenCrt, 1) => {
                // 25% stipple dither for cards & tracks (pure monochrome green)
                for cy in y..y_end {
                    let row_start = (cy as usize) * (self.width as usize);
                    for cx in x..x_end {
                        let c = if (cx % 2 == 0) && (cy % 2 == 0) { 1 } else { 0 };
                        self.pixels[row_start + (cx as usize)] = c;
                    }
                }
            }
            (PaletteMode::GreenCrt, 2) => {
                // 50% checkerboard dither for selection
                for cy in y..y_end {
                    let row_start = (cy as usize) * (self.width as usize);
                    for cx in x..x_end {
                        let c = if (cx + cy) % 2 == 0 { 2 } else { 0 };
                        self.pixels[row_start + (cx as usize)] = c;
                    }
                }
            }
            (PaletteMode::Amber, 1) => {
                // 25% stipple dither for cards & tracks (pure monochrome amber)
                for cy in y..y_end {
                    let row_start = (cy as usize) * (self.width as usize);
                    for cx in x..x_end {
                        let c = if (cx % 2 == 0) && (cy % 2 == 0) { 1 } else { 0 };
                        self.pixels[row_start + (cx as usize)] = c;
                    }
                }
            }
            (PaletteMode::Amber, 2) => {
                // 50% checkerboard dither for selection
                for cy in y..y_end {
                    let row_start = (cy as usize) * (self.width as usize);
                    for cx in x..x_end {
                        let c = if (cx + cy) % 2 == 0 { 2 } else { 0 };
                        self.pixels[row_start + (cx as usize)] = c;
                    }
                }
            }
            (PaletteMode::C64, 1) => {
                // Authentic C64 GEOS-style 50% checkerboard dither: background blue (0) and dark grey (1)
                for cy in y..y_end {
                    let row_start = (cy as usize) * (self.width as usize);
                    for cx in x..x_end {
                        let c = if (cx + cy) % 2 == 0 { 1 } else { 0 };
                        self.pixels[row_start + (cx as usize)] = c;
                    }
                }
            }
            (PaletteMode::C64, 2) => {
                // 50% checkerboard dither: background blue (0) and light blue (2)
                for cy in y..y_end {
                    let row_start = (cy as usize) * (self.width as usize);
                    for cx in x..x_end {
                        let c = if (cx + cy) % 2 == 0 { 2 } else { 0 };
                        self.pixels[row_start + (cx as usize)] = c;
                    }
                }
            }
            (PaletteMode::Atari, 1) => {
                // Authentic Atari GEM 50% dither: dark umber (0) and bronze (1)
                for cy in y..y_end {
                    let row_start = (cy as usize) * (self.width as usize);
                    for cx in x..x_end {
                        let c = if (cx + cy) % 2 == 0 { 1 } else { 0 };
                        self.pixels[row_start + (cx as usize)] = c;
                    }
                }
            }
            (PaletteMode::Atari, 2) => {
                // Authentic Atari GEM 50% dither: dark umber (0) and gold (2)
                for cy in y..y_end {
                    let row_start = (cy as usize) * (self.width as usize);
                    for cx in x..x_end {
                        let c = if (cx + cy) % 2 == 0 { 2 } else { 0 };
                        self.pixels[row_start + (cx as usize)] = c;
                    }
                }
            }
            (PaletteMode::ZxSpectrum, 1) => {
                // ZX Spectrum 50% checkerboard dither: black (0) and blue (1)
                for cy in y..y_end {
                    let row_start = (cy as usize) * (self.width as usize);
                    for cx in x..x_end {
                        let c = if (cx + cy) % 2 == 0 { 1 } else { 0 };
                        self.pixels[row_start + (cx as usize)] = c;
                    }
                }
            }
            (PaletteMode::ZxSpectrum, 2) => {
                // ZX Spectrum 50% checkerboard dither: black (0) and bright blue (2)
                for cy in y..y_end {
                    let row_start = (cy as usize) * (self.width as usize);
                    for cx in x..x_end {
                        let c = if (cx + cy) % 2 == 0 { 2 } else { 0 };
                        self.pixels[row_start + (cx as usize)] = c;
                    }
                }
            }
            _ => {
                for cy in y..y_end {
                    let row_start = (cy as usize) * (self.width as usize);
                    for cx in x..x_end {
                        self.pixels[row_start + (cx as usize)] = color_index;
                    }
                }
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
        let fg = if is_table_border_char(ch) { 7 } else { fg };
        let code = unicode_to_cp437(ch);
        if let Some(font16) = font_8x16_for_mode(self.font_mode) {
            let glyph = &font16[code as usize];
            for row in 0..16 {
                let py = y + row;
                if py >= self.height { break; }
                let byte = glyph[row as usize];
                let row_start = (py as usize) * (self.width as usize);

                for col in 0..FONT_WIDTH {
                    let px = x + col;
                    if px >= self.width { break; }

                    let is_on = (byte & (0x80 >> col)) != 0;
                    let pixel_idx = row_start + (px as usize);

                    if is_on {
                        self.pixels[pixel_idx] = fg;
                    } else if let Some(bg_color) = bg {
                        self.pixels[pixel_idx] = bg_color;
                    }
                }
            }
        } else {
            let font = font_for_mode(self.font_mode);
            let glyph = font[code as usize];

            for row in 0..FONT_HEIGHT {
                let py = y + row;
                if py >= self.height { break; }
                let byte = glyph[row as usize];
                let row_start = (py as usize) * (self.width as usize);

                for col in 0..FONT_WIDTH {
                    let px = x + col;
                    if px >= self.width { break; }

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
        self.set_palette_theme(view.color_theme);
        self.font_mode = view.resolution.to_system_mode();
        self.clear(0); // clear to background

        let is_vga_font = font_8x16_for_mode(self.font_mode).is_some();
        let font_height = if is_vga_font { 16 } else { 8 };

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
                    if l.style.underline && l.y + font_height < self.height {
                        self.fill_rect(l.x, l.y + font_height, l.width, 1, l.style.fg.palette_index);
                    }
                }
            }
        }

        // Draw cursor if visible
        if let Some(cursor) = &view.cursor {
            if cursor.visible && cursor.x < self.width && cursor.y < self.height {
                let cur_h = if is_vga_font { 14 } else { 8 };
                self.fill_rect(cursor.x, cursor.y, 6, cur_h, 8); // Green block cursor
            }
        }

        // Draw system-specific retro mouse cursor if active
        if let Some((mx, my)) = view.mouse_pos {
            self.draw_mouse_cursor_for_view(view, mx, my);
        }
    }

    /// Finds the exact character cell bounds and underlying character glyph at `(mx, my)`.
    /// Strictly snaps both horizontally (`cx = (mx / char_w) * char_w`) and vertically (`cy = (my / char_h) * char_h`).
    pub fn find_char_cell_and_glyph(
        width: u16,
        height: u16,
        mx: u16,
        my: u16,
        view: Option<&View>,
    ) -> (u16, u16, Option<char>) {
        let char_w: u16 = 8;
        let char_h: u16 = if height <= 200 { 8 } else { 16 };

        // 1. Strict 2D character-grid snapping: cx and cy are ALWAYS exact multiples of char_w and char_h
        let cx = ((mx / char_w) * char_w).min(width.saturating_sub(char_w));
        let cy = ((my / char_h) * char_h).min(height.saturating_sub(char_h));

        // 2. Glyph lookup: check if any Text or Link element coincides with this (cx, cy) character cell
        let mut found_char = None;
        if let Some(v) = view {
            let cell_center_x = cx + char_w / 2;

            for el in &v.elements {
                match el {
                    Element::Text(t) => {
                        let t_center_y = t.y + char_h / 2;
                        if t_center_y >= cy && t_center_y < cy + char_h {
                            let char_count = t.text.chars().count() as u16;
                            let text_w = char_count * char_w;
                            if cell_center_x >= t.x && cell_center_x < t.x + text_w {
                                let col_idx = (cell_center_x - t.x) / char_w;
                                found_char = t.text.chars().nth(col_idx as usize);
                                break;
                            }
                        }
                    }
                    Element::Link(l) => {
                        let l_center_y = l.y + char_h / 2;
                        if l_center_y >= cy && l_center_y < cy + char_h {
                            let char_count = l.text.chars().count() as u16;
                            let text_w = char_count * char_w;
                            if cell_center_x >= l.x && cell_center_x < l.x + text_w {
                                let col_idx = (cell_center_x - l.x) / char_w;
                                found_char = l.text.chars().nth(col_idx as usize);
                                break;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        (cx, cy, found_char)
    }

    /// Draws the system-specific retro software mouse cursor at `(mx, my)` taking into account active view elements.
    pub fn draw_mouse_cursor_for_view(&mut self, view: &View, mx: u16, my: u16) {
        self.draw_mouse_cursor_internal(view.system_mode, mx, my, Some(view));
    }

    /// Draws the system-specific retro software mouse cursor at `(mx, my)`.
    pub fn draw_mouse_cursor(&mut self, mode: PaletteMode, mx: u16, my: u16) {
        self.draw_mouse_cursor_internal(mode, mx, my, None);
    }

    fn draw_mouse_cursor_internal(&mut self, mode: PaletteMode, mx: u16, my: u16, view: Option<&View>) {
        match mode {
            PaletteMode::Svga
            | PaletteMode::Sga
            | PaletteMode::Vga
            | PaletteMode::Ega
            | PaletteMode::Cga
            | PaletteMode::Amber
            | PaletteMode::GreenCrt => {
                // Classic DOS text mode full character cell block cursor.
                // Takes the whole single character (8px wide, 16px high or 8px high on CGA).
                // Adds cursor background (Bright Red on VGA/EGA/CGA, Amber beam on Amber, Green phosphor on Green)
                // and inverts the symbol glyph at the symbol place (to black / 0) for authentic DOS NC legibility.
                let cursor_bg = match mode {
                    PaletteMode::Amber => 6,   // High intensity Amber beam
                    PaletteMode::GreenCrt => 6, // High intensity Green phosphor beam
                    _ => 10,                   // Bright Red
                };

                let (cx, cy, maybe_ch) = Self::find_char_cell_and_glyph(self.width, self.height, mx, my, view);

                if let Some(ch) = maybe_ch {
                    let code = unicode_to_cp437(ch);
                    let glyph = &font::CP437_FONT_8X16[code as usize];
                    for r in 0..16 {
                        let py = cy + r;
                        if py >= self.height { break; }
                        let byte = glyph[r as usize];
                        let row_start = (py as usize) * (self.width as usize);
                        for c in 0..8 {
                            let px = cx + c;
                            if px >= self.width { break; }
                            let idx = row_start + (px as usize);
                            let is_glyph = (byte & (0x80 >> c)) != 0;
                            if is_glyph {
                                self.pixels[idx] = 0; // Inverted text glyph (black)
                            } else {
                                self.pixels[idx] = cursor_bg; // Added cursor background
                            }
                        }
                    }
                } else {
                    for r in 0..16 {
                        let py = cy + r;
                        if py >= self.height { break; }
                        let row_start = (py as usize) * (self.width as usize);
                        for c in 0..8 {
                            let px = cx + c;
                            if px >= self.width { break; }
                            let idx = row_start + (px as usize);
                            let val = self.pixels[idx];
                            if val <= 3 {
                                self.pixels[idx] = cursor_bg; // Added cursor background
                            } else {
                                self.pixels[idx] = 0; // Inverted text glyph (black)
                            }
                        }
                    }
                }
            }
            PaletteMode::C64 => {
                // Official Commodore 1351 / GEOS hardware mouse arrow pointer (11x14 sprite).
                const C64_GEOS_ARROW: [[u8; 11]; 14] = [
                    [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [1, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                    [1, 2, 2, 1, 0, 0, 0, 0, 0, 0, 0],
                    [1, 2, 2, 2, 1, 0, 0, 0, 0, 0, 0],
                    [1, 2, 2, 2, 2, 1, 0, 0, 0, 0, 0],
                    [1, 2, 2, 2, 2, 2, 1, 0, 0, 0, 0],
                    [1, 2, 2, 2, 2, 2, 2, 1, 0, 0, 0],
                    [1, 2, 2, 2, 2, 1, 1, 1, 1, 1, 0],
                    [1, 2, 2, 1, 2, 1, 0, 0, 0, 0, 0],
                    [1, 2, 1, 0, 1, 2, 1, 0, 0, 0, 0],
                    [1, 1, 0, 0, 0, 1, 2, 1, 0, 0, 0],
                    [1, 0, 0, 0, 0, 0, 1, 2, 1, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 1, 2, 1, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0],
                ];
                let mx = mx.min(self.width.saturating_sub(11));
                let my = my.min(self.height.saturating_sub(14));
                for r in 0..14 {
                    for c in 0..11 {
                        match C64_GEOS_ARROW[r][c] {
                            1 => self.set_pixel(mx + c as u16, my + r as u16, 13), // VIC-II Black outline
                            2 => self.set_pixel(mx + c as u16, my + r as u16, 14), // VIC-II White interior
                            _ => {}
                        }
                    }
                }
            }
            PaletteMode::ZxSpectrum => {
                // Official Sinclair ZX Spectrum AMX / Kempston mouse arrow pointer (8x12 pixels).
                const ZX_AMX_ARROW: [[u8; 8]; 12] = [
                    [1, 1, 0, 0, 0, 0, 0, 0],
                    [1, 2, 1, 0, 0, 0, 0, 0],
                    [1, 2, 2, 1, 0, 0, 0, 0],
                    [1, 2, 2, 2, 1, 0, 0, 0],
                    [1, 2, 2, 2, 2, 1, 0, 0],
                    [1, 2, 2, 2, 2, 2, 1, 0],
                    [1, 2, 2, 1, 1, 1, 1, 1],
                    [1, 2, 1, 2, 1, 0, 0, 0],
                    [1, 1, 0, 1, 2, 1, 0, 0],
                    [0, 0, 0, 0, 1, 2, 1, 0],
                    [0, 0, 0, 0, 1, 2, 1, 0],
                    [0, 0, 0, 0, 0, 1, 1, 0],
                ];
                let mx = mx.min(self.width.saturating_sub(8));
                let my = my.min(self.height.saturating_sub(12));
                for r in 0..12 {
                    for c in 0..8 {
                        match ZX_AMX_ARROW[r][c] {
                            1 => self.set_pixel(mx + c as u16, my + r as u16, 0), // Sinclair Black outline
                            2 => self.set_pixel(mx + c as u16, my + r as u16, 7), // Bright Cyan core
                            _ => {}
                        }
                    }
                }
            }
            PaletteMode::Atari => {
                // Official Atari GEM desktop arrow pointer (11x16 pixels).
                const ATARI_ARROW: [[u8; 11]; 16] = [
                    [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [1, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                    [1, 2, 2, 1, 0, 0, 0, 0, 0, 0, 0],
                    [1, 2, 2, 2, 1, 0, 0, 0, 0, 0, 0],
                    [1, 2, 2, 2, 2, 1, 0, 0, 0, 0, 0],
                    [1, 2, 2, 2, 2, 2, 1, 0, 0, 0, 0],
                    [1, 2, 2, 2, 2, 2, 2, 1, 0, 0, 0],
                    [1, 2, 2, 2, 2, 2, 2, 2, 1, 0, 0],
                    [1, 2, 2, 2, 2, 2, 2, 2, 2, 1, 0],
                    [1, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1],
                    [1, 2, 2, 1, 2, 2, 1, 0, 0, 0, 0],
                    [1, 2, 1, 0, 1, 2, 2, 1, 0, 0, 0],
                    [1, 1, 0, 0, 0, 1, 2, 2, 1, 0, 0],
                    [1, 0, 0, 0, 0, 0, 1, 2, 2, 1, 0],
                    [0, 0, 0, 0, 0, 0, 0, 1, 2, 2, 1],
                    [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0],
                ];
                let mx = mx.min(self.width.saturating_sub(11));
                let my = my.min(self.height.saturating_sub(16));
                for r in 0..16 {
                    for c in 0..11 {
                        match ATARI_ARROW[r][c] {
                            1 => self.set_pixel(mx + c as u16, my + r as u16, 13), // Pure black outline
                            2 => self.set_pixel(mx + c as u16, my + r as u16, 14), // Pure white interior
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::font::{font_for_mode, unicode_to_cp437};

    #[test]
    fn test_fonts_are_unique_and_authentic_per_system() {
        let vga = font_for_mode(PaletteMode::Vga);
        let zx = font_for_mode(PaletteMode::ZxSpectrum);
        let c64 = font_for_mode(PaletteMode::C64);
        let atari = font_for_mode(PaletteMode::Atari);

        let code_a = unicode_to_cp437('A') as usize;
        let code_0 = unicode_to_cp437('0') as usize;
        let code_s = unicode_to_cp437('S') as usize;

        // Each retro system has distinct glyph bitmaps for 'A'
        assert_ne!(vga[code_a], zx[code_a], "ZX 'A' should differ from VGA");
        assert_ne!(vga[code_a], c64[code_a], "C64 'A' should differ from VGA");
        assert_ne!(c64[code_a], zx[code_a], "C64 'A' should differ from ZX");

        // Each retro system has distinct glyph bitmaps for '0'
        assert_ne!(zx[code_0], c64[code_0]);
        assert_ne!(zx[code_0], vga[code_0]);

        // Each retro system has distinct glyph bitmaps for 'S'
        assert_ne!(zx[code_s], c64[code_s]);
        assert_ne!(c64[code_s], atari[code_s]);
    }

    #[test]
    fn test_framebuffer_draws_text_with_system_fonts() {
        for mode in PaletteMode::ALL {
            let (w, h) = mode.resolution();
            let mut fb = Framebuffer::new(w, h);
            fb.font_mode = mode;
            fb.draw_text(0, 0, "HELLO WORLD", 1, None);

            // Verify non-zero pixels were drawn
            let non_zero_count = fb.pixels.iter().filter(|&&p| p == 1).count();
            assert!(non_zero_count > 0, "Mode {:?} should render pixel data", mode);
        }
    }

    #[test]
    fn test_vga_8x16_box_drawing_continuous_vertical_lines() {
        let font16 = font_8x16_for_mode(PaletteMode::Vga).expect("VGA 8x16 font should exist");
        let vertical_code = unicode_to_cp437('│') as usize;
        let glyph = &font16[vertical_code];
        for row in 0..16 {
            assert_ne!(glyph[row], 0, "Vertical line character '│' must have non-zero bits on row {}", row);
        }
    }

    #[test]
    fn test_draw_mouse_cursor_across_all_system_modes() {
        for mode in PaletteMode::ALL {
            let (w, h) = mode.resolution();
            let mut fb = Framebuffer::new(w, h);
            fb.font_mode = mode;

            // Draw cursor in center and near boundaries
            fb.draw_mouse_cursor(mode, w / 2, h / 2);
            fb.draw_mouse_cursor(mode, 0, 0);
            fb.draw_mouse_cursor(mode, w - 1, h - 1);

            let non_zero = fb.pixels.iter().filter(|&&p| p != 0).count();
            assert!(non_zero > 0, "Mode {:?} should render cursor pixels", mode);
        }

        // Verify VGA text mode snaps to 8x16 character cell and draws full red character block (index 10, width 8)
        let mut fb_vga = Framebuffer::new(640, 400);
        fb_vga.draw_mouse_cursor(PaletteMode::Vga, 19, 35); // cell (col 2 = x 16..24, row 2 = y 32..48)
        // Red block should be drawn at x = 16..24, y = 32..48
        for row in 32..48 {
            for col in 16..24 {
                let idx = (row as usize) * 640 + (col as usize);
                assert_eq!(fb_vga.pixels[idx], 10, "VGA cursor must draw bright red (index 10) across full 8px cell at ({}, {})", col, row);
            }
        }

        // Verify glyph inversion when cursor is placed over character text
        let mut fb_text = Framebuffer::new(640, 400);
        fb_text.font_mode = PaletteMode::Vga;
        fb_text.draw_text(16, 32, "A", 14, None);
        let code_a = unicode_to_cp437('A');
        let glyph_a = font::CP437_FONT_8X16[code_a as usize];
        fb_text.draw_mouse_cursor(PaletteMode::Vga, 19, 35);
        for r in 0..16 {
            let byte = glyph_a[r];
            for c in 0..8 {
                let col = 16 + c as u16;
                let row = 32 + r as u16;
                let idx = (row as usize) * 640 + (col as usize);
                if (byte & (0x80 >> c)) != 0 {
                    assert_eq!(fb_text.pixels[idx], 0, "Glyph pixel of 'A' must invert to 0 at ({}, {})", col, row);
                } else {
                    assert_eq!(fb_text.pixels[idx], 10, "Background pixel of cell must be 10 at ({}, {})", col, row);
                }
            }
        }
    }

    #[test]
    fn test_char_grid_mouse_cursor_snapping() {
        use pixel_ssh_view::{Element, TextElement, TextStyle, Color};

        // Create a mock view with off-grid elements at y = 4 (top bar) and y = 54 (projects header)
        let mut view = View::new(640, 400);
        view.system_mode = pixel_ssh_view::SystemMode::Vga;
        view.add(Element::Text(TextElement {
            x: 12,
            y: 4,
            text: "DMYTRO YEMELIANOV".to_string(),
            style: TextStyle::new(Color::from_palette(7)),
        }));
        view.add(Element::Text(TextElement {
            x: 16,
            y: 54,
            text: "PROJECT // ARCHITECT".to_string(),
            style: TextStyle::new(Color::from_palette(12)),
        }));

        // Hover over top bar text at (19, 7) -> must snap to cx=16, cy=0 (Row 0: 0..16)
        let (cx1, cy1, ch1) = Framebuffer::find_char_cell_and_glyph(640, 400, 19, 7, Some(&view));
        assert_eq!(cx1, 16, "cx must snap to column 2 (16)");
        assert_eq!(cy1, 0, "cy must snap to row 0 (0), NOT jump to element y=4");
        assert_eq!(ch1, Some('M')); // "D" at 12..20, "M" at 20..28, center is 16+4=20 -> 'M'

        // Hover over project text at (25, 60) -> must snap to cx=24, cy=48 (Row 3: 48..64)
        let (cx2, cy2, ch2) = Framebuffer::find_char_cell_and_glyph(640, 400, 25, 60, Some(&view));
        assert_eq!(cx2, 24, "cx must snap to column 3 (24)");
        assert_eq!(cy2, 48, "cy must snap to row 3 (48), NOT jump to element y=54");
        assert_eq!(ch2, Some('R')); // "P" at 16..24, "R" at 24..32, center is 24+4=28 -> 'R'

        // Sweep mouse vertically across screen: every single cy must be an exact multiple of 16
        for my in 0..400 {
            let (cx, cy, _) = Framebuffer::find_char_cell_and_glyph(640, 400, 153, my, Some(&view));
            assert_eq!(cx % 8, 0, "cx must be multiple of 8 at my={}", my);
            assert_eq!(cy % 16, 0, "cy must be multiple of 16 at my={}", my);
        }
    }

    #[test]
    fn test_monochrome_palettes_have_strict_phosphor_purity() {
        let green_p = Framebuffer::palette_for_mode(PaletteMode::GreenCrt);
        for (i, entry) in green_p.iter().enumerate() {
            assert_eq!(entry[0], 0, "Green CRT index {} must have 0 red, got {}", i, entry[0]);
            assert_eq!(entry[2], 0, "Green CRT index {} must have 0 blue, got {}", i, entry[2]);
        }

        let amber_p = Framebuffer::palette_for_mode(PaletteMode::Amber);
        for (i, entry) in amber_p.iter().enumerate() {
            assert_eq!(entry[2], 0, "Amber CRT index {} must have 0 blue, got {}", i, entry[2]);
        }
    }

    #[test]
    fn test_dithered_fills_produce_alternating_stipple_and_checkerboard() {
        let mut fb_c64 = Framebuffer::new(320, 200);
        fb_c64.font_mode = PaletteMode::C64;
        fb_c64.fill_rect(0, 0, 4, 4, 1); // C64 50% checkerboard with dark grey (1) and background (0)
        assert_eq!(fb_c64.pixels[0], 1); // (0,0) -> 1
        assert_eq!(fb_c64.pixels[1], 0); // (1,0) -> 0
        assert_eq!(fb_c64.pixels[320], 0); // (0,1) -> 0
        assert_eq!(fb_c64.pixels[321], 1); // (1,1) -> 1

        let mut fb_green = Framebuffer::new(640, 400);
        fb_green.font_mode = PaletteMode::GreenCrt;
        fb_green.fill_rect(0, 0, 4, 4, 1); // Green 25% stipple
        assert_eq!(fb_green.pixels[0], 1); // (0,0) -> 1
        assert_eq!(fb_green.pixels[1], 0); // (1,0) -> 0
        assert_eq!(fb_green.pixels[640], 0); // (0,1) -> 0
        assert_eq!(fb_green.pixels[641], 0); // (1,1) -> 0
    }

    #[test]
    fn test_vga_8x16_box_drawing_horizontal_line_thickness() {
        let font16 = font_8x16_for_mode(PaletteMode::Vga).expect("VGA 8x16 font should exist");
        let horiz_code = unicode_to_cp437('─') as usize;
        let glyph = &font16[horiz_code];
        // Must be at least 2 pixels thick (rows 7 and 8)
        assert_ne!(glyph[7], 0, "Horizontal line '─' must have active pixels on row 7");
        assert_ne!(glyph[8], 0, "Horizontal line '─' must have active pixels on row 8");
    }

    #[test]
    fn test_box_drawing_chars_rendered_in_uniform_border_color() {
        let mut fb = Framebuffer::new(640, 400);
        fb.font_mode = PaletteMode::Vga;
        // Even if requested with fg = 5 (body gray), box border characters should be drawn in palette index 7
        fb.draw_text(0, 0, "│ Test │", 5, None);
        // Column 0 is '│', which is at x = 0..8. Row 0..16 should be drawn in color 7
        let p_border = fb.pixels[0 * 640 + 3];
        assert_eq!(p_border, 7, "Border '│' must be drawn in palette index 7");
    }
}
