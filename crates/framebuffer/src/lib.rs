pub mod font;

use font::{
    font_8x16_for_mode, font_for_mode, unicode_to_cp437, EGA_FONT_8X14, FONT_HEIGHT, FONT_WIDTH,
};
use pixel_ssh_view::{is_table_border_char, ColorPalette, Element, PaletteMode, View};

pub const DEFAULT_WIDTH: u16 = 640;
pub const DEFAULT_HEIGHT: u16 = 400;

#[derive(Debug, Clone)]
pub struct Framebuffer {
    pub width: u16,
    pub height: u16,
    pub pixels: Vec<u8>,
    pub palette: [[u8; 4]; 256],
    pub font_mode: PaletteMode,
    /// Cell height comes from resolution; glyph family comes from `font_mode`.
    pub font_height: u16,
    pub color_palette: ColorPalette,
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
            font_height: 16,
            color_palette: ColorPalette::default(),
        }
    }

    pub fn default_palette() -> [[u8; 4]; 256] {
        Self::palette_for_palette(ColorPalette::default())
    }

    pub fn palette_for_mode(mode: PaletteMode) -> [[u8; 4]; 256] {
        Self::palette_for_palette(mode.to_theme())
    }

    pub fn palette_for_palette(theme: ColorPalette) -> [[u8; 4]; 256] {
        let mut p = [[0, 0, 0, 255]; 256];

        match theme {
            ColorPalette::Commander => {
                // Application palette inspired by DOS file managers. No source
                // palette or byte-for-byte match has been recorded.
                p[0] = [0, 0, 168, 255]; // 0: #0000a8 (Iconic Commander navy blue background)
                p[1] = [0, 168, 168, 255]; // 1: #00aaaa (Cyan header bar / accent)
                p[2] = [0, 168, 168, 255]; // 2: #00aaaa (Selected item cyan background bar)
                p[3] = [0, 168, 168, 255]; // 3: #00aaaa (Cyan table / panel border)
                p[4] = [85, 255, 255, 255]; // 4: #55ffff (Bright cyan muted/secondary)
                p[5] = [255, 255, 255, 255]; // 5: #ffffff (Bright white text body)
                p[6] = [255, 255, 85, 255]; // 6: #ffff55 (Bright yellow titles and headers)
                p[7] = [85, 255, 255, 255]; // 7: #55ffff (Bright cyan links/accents)
                p[8] = [85, 255, 85, 255]; // 8: #55ff55 (Bright green)
                p[9] = [255, 255, 85, 255]; // 9: #ffff55 (Bright yellow warning)
                p[10] = [255, 85, 85, 255]; // 10: #ff5555 (Bright red danger)
                p[11] = [255, 85, 255, 255]; // 11: #ff55ff (Bright magenta)
                p[12] = [0, 168, 168, 255]; // 12: #00aaaa (Dark cyan)
                p[13] = [0, 0, 0, 255]; // 13: #000000 (Pure black, shadows & clock background)
                p[14] = [255, 255, 255, 255]; // 14: #ffffff (Pure white)
                p[15] = [168, 168, 168, 255]; // 15: #aaaaaa (Dialog box light gray)

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
            ColorPalette::VgaModern => {
                p[0] = [13, 17, 23, 255]; // 0: #0d1117 (Dark background)
                p[1] = [22, 27, 34, 255]; // 1: #161b22 (Card header)
                p[2] = [33, 38, 45, 255]; // 2: #21262d (Selected item background)
                p[3] = [88, 96, 105, 255]; // 3: #586069 (Visible medium border gray)
                p[4] = [139, 148, 158, 255]; // 4: #8b949e (Muted gray)
                p[5] = [201, 209, 217, 255]; // 5: #c9d1d9 (Light text body)
                p[6] = [240, 246, 252, 255]; // 6: #f0f6fc (Bright white text)
                p[7] = [88, 166, 255, 255]; // 7: #58a6ff (Accent blue)
                p[8] = [63, 185, 80, 255]; // 8: #3fb950 (Terminal green)
                p[9] = [210, 153, 34, 255]; // 9: #d29922 (Warning yellow)
                p[10] = [248, 81, 73, 255]; // 10: #f85149 (Danger/WIP red)
                p[11] = [188, 140, 255, 255]; // 11: #bc8cff (Purple)
                p[12] = [57, 197, 187, 255]; // 12: #39c5bb (Cyan)
                p[13] = [0, 0, 0, 255]; // 13: #000000 (Pure black)
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
            ColorPalette::Ega => {
                // Application role palette using RGBI-like colors. Its role
                // indices and RGB values are not a verified EGA/CGA palette.
                p[0] = [0, 0, 0, 255]; // Black
                p[1] = [0, 0, 170, 255]; // Blue
                p[2] = [0, 170, 0, 255]; // Green
                p[3] = [0, 170, 170, 255]; // Cyan border
                p[4] = [170, 170, 170, 255]; // Light Gray / Muted
                p[5] = [255, 255, 255, 255]; // White body text
                p[6] = [255, 255, 255, 255]; // Bright white
                p[7] = [85, 255, 255, 255]; // Bright Cyan accent
                p[8] = [85, 255, 85, 255]; // Bright Green
                p[9] = [255, 255, 85, 255]; // Bright Yellow
                p[10] = [255, 85, 85, 255]; // Bright Red
                p[11] = [255, 85, 255, 255]; // Bright Magenta
                p[12] = [85, 255, 255, 255]; // Bright Cyan
                p[13] = [0, 0, 0, 255]; // Pure black
                p[14] = [255, 255, 255, 255]; // Pure white
                p[15] = [85, 85, 85, 255]; // Dark Gray

                for idx in 16..256 {
                    p[idx] = p[idx % 16];
                }
            }
            ColorPalette::ZxSpectrum => {
                // Application role palette inspired by the Spectrum's normal
                // and bright colors; values have not been verified to hardware.
                p[0] = [0, 0, 0, 255]; // Black bg
                p[1] = [0, 0, 192, 255]; // Blue header / dither base
                p[2] = [0, 0, 255, 255]; // Bright blue selection
                p[3] = [0, 192, 192, 255]; // Cyan border
                p[4] = [192, 192, 192, 255]; // White / muted
                p[5] = [255, 255, 0, 255]; // Bright yellow body text (iconic Spectrum look)
                p[6] = [255, 255, 255, 255]; // Bright white
                p[7] = [0, 255, 255, 255]; // Bright cyan accent
                p[8] = [0, 255, 0, 255]; // Bright green
                p[9] = [255, 255, 0, 255]; // Bright yellow
                p[10] = [255, 0, 0, 255]; // Bright red
                p[11] = [255, 0, 255, 255]; // Bright magenta
                p[12] = [0, 255, 255, 255]; // Bright cyan
                p[13] = [0, 0, 0, 255]; // Pure black
                p[14] = [255, 255, 255, 255]; // Pure white
                p[15] = [192, 0, 0, 255]; // Red

                for idx in 16..256 {
                    p[idx] = p[idx % 16];
                }
            }
            ColorPalette::C64 => {
                // Application role palette using C64-associated colors. No
                // Pepto or Colodore version/settings were recorded for this RGB set.
                p[0] = [64, 49, 141, 255]; // VIC-II #6 Blue background (#40318D)
                p[1] = [51, 51, 51, 255]; // VIC-II #11 Dark Grey (card dither base)
                p[2] = [112, 109, 235, 255]; // VIC-II #14 Light Blue (selection dither base)
                p[3] = [187, 187, 187, 255]; // VIC-II #15 Light Grey border
                p[4] = [119, 119, 119, 255]; // VIC-II #12 Medium Grey muted
                p[5] = [170, 255, 238, 255]; // VIC-II #3 Light Cyan body text
                p[6] = [255, 255, 255, 255]; // VIC-II #1 White header/accent
                p[7] = [238, 238, 119, 255]; // VIC-II #7 Yellow accent
                p[8] = [170, 255, 102, 255]; // VIC-II #13 Light Green
                p[9] = [221, 136, 85, 255]; // VIC-II #8 Orange
                p[10] = [255, 119, 119, 255]; // VIC-II #10 Light Red
                p[11] = [204, 68, 204, 255]; // VIC-II #4 Purple / Violet
                p[12] = [112, 109, 235, 255]; // VIC-II #14 Light Blue
                p[13] = [0, 0, 0, 255]; // VIC-II #0 Black
                p[14] = [255, 255, 255, 255]; // VIC-II #1 White
                p[15] = [102, 68, 0, 255]; // VIC-II #9 Brown

                for idx in 16..256 {
                    p[idx] = p[idx % 16];
                }
            }
            ColorPalette::Atari => {
                // Application RGB approximation inspired by Atari colors. GTIA
                // produces composite signals, so it has no single hardware RGB table.
                p[0] = [0, 0, 0, 255]; // GTIA Deep dark umber / black
                p[1] = [64, 40, 16, 255]; // GTIA Dark bronze (card dither base)
                p[2] = [144, 96, 24, 255]; // GTIA Gold selection (dither base)
                p[3] = [160, 96, 32, 255]; // GTIA Copper border
                p[4] = [192, 144, 64, 255]; // GTIA Gold-tan muted
                p[5] = [230, 200, 128, 255]; // GTIA Warm wheat body
                p[6] = [255, 240, 176, 255]; // GTIA Luminous gold text
                p[7] = [0, 208, 208, 255]; // GTIA Turquoise/teal accent
                p[8] = [96, 208, 64, 255]; // GTIA Lime green
                p[9] = [248, 208, 32, 255]; // GTIA Sunburst yellow
                p[10] = [224, 64, 48, 255]; // GTIA Burnt crimson
                p[11] = [192, 80, 208, 255]; // GTIA Orchid purple
                p[12] = [48, 224, 224, 255]; // GTIA Bright teal
                p[13] = [0, 0, 0, 255]; // GTIA Pure black
                p[14] = [255, 255, 255, 255]; // GTIA Pure white
                p[15] = [48, 24, 8, 255]; // GTIA Deep umber

                for (idx, color) in p.iter_mut().enumerate().skip(16) {
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
                    *color = [
                        ((lum_val as f32) * hr).min(255.0) as u8,
                        ((lum_val as f32) * hg).min(255.0) as u8,
                        ((lum_val as f32) * hb).min(255.0) as u8,
                        255,
                    ];
                }
            }
            ColorPalette::Amber => {
                // Single-tint amber display palette. The RGB values are a design
                // choice, not a measured P134/P20 phosphor representation.
                p[0] = [0, 0, 0, 255]; // Tube off
                p[1] = [50, 32, 0, 255]; // Dim amber (card dither base)
                p[2] = [95, 61, 0, 255]; // Medium-low amber (selection dither base)
                p[3] = [115, 74, 0, 255]; // Border
                p[4] = [160, 103, 0, 255]; // Muted amber
                p[5] = [215, 139, 0, 255]; // Normal amber text
                p[6] = [255, 165, 0, 255]; // High intensity beam amber
                p[7] = [255, 185, 0, 255]; // Luminous amber glow
                p[8] = [225, 145, 0, 255];
                p[9] = [245, 158, 0, 255];
                p[10] = [170, 110, 0, 255];
                p[11] = [195, 126, 0, 255];
                p[12] = [235, 152, 0, 255];
                p[13] = [0, 0, 0, 255];
                p[14] = [255, 185, 0, 255];
                p[15] = [130, 84, 0, 255];

                for (idx, color) in p.iter_mut().enumerate().skip(16) {
                    let v = (((idx - 16) * 255) / 239) as u8;
                    *color = [v, ((v as u16 * 165) / 255) as u8, 0, 255];
                }
            }
            ColorPalette::GreenCrt => {
                // Single-tint green display palette. The RGB values are a design
                // choice, not a measured IBM 5151 or P1 phosphor representation.
                p[0] = [0, 0, 0, 255]; // Tube off
                p[1] = [0, 45, 0, 255]; // Dim green (card dither base)
                p[2] = [0, 90, 0, 255]; // Medium-low green (selection dither base)
                p[3] = [0, 105, 0, 255]; // Border
                p[4] = [0, 150, 0, 255]; // Muted green phosphor
                p[5] = [0, 210, 0, 255]; // Normal P1 phosphor green body text
                p[6] = [0, 255, 0, 255]; // High intensity beam green
                p[7] = [0, 240, 0, 255]; // Accent green
                p[8] = [0, 220, 0, 255];
                p[9] = [0, 235, 0, 255];
                p[10] = [0, 160, 0, 255];
                p[11] = [0, 185, 0, 255];
                p[12] = [0, 225, 0, 255];
                p[13] = [0, 0, 0, 255];
                p[14] = [0, 255, 0, 255];
                p[15] = [0, 120, 0, 255];

                for (idx, color) in p.iter_mut().enumerate().skip(16) {
                    let v = (((idx - 16) * 255) / 239) as u8;
                    *color = [0, v, 0, 255];
                }
            }
        }

        p
    }

    pub fn set_color_palette(&mut self, palette: ColorPalette) {
        self.color_palette = palette;
        self.palette = Self::palette_for_palette(palette);
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

        match (self.color_palette, color_index) {
            (ColorPalette::GreenCrt, 1) => {
                // 25% stipple dither for cards & tracks (pure monochrome green)
                for cy in y..y_end {
                    let row_start = (cy as usize) * (self.width as usize);
                    for cx in x..x_end {
                        let c = if (cx % 2 == 0) && (cy % 2 == 0) { 1 } else { 0 };
                        self.pixels[row_start + (cx as usize)] = c;
                    }
                }
            }
            (ColorPalette::GreenCrt, 2) => {
                // 50% checkerboard dither for selection
                for cy in y..y_end {
                    let row_start = (cy as usize) * (self.width as usize);
                    for cx in x..x_end {
                        let c = if (cx + cy) % 2 == 0 { 2 } else { 0 };
                        self.pixels[row_start + (cx as usize)] = c;
                    }
                }
            }
            (ColorPalette::Amber, 1) => {
                // 25% stipple dither for cards & tracks (pure monochrome amber)
                for cy in y..y_end {
                    let row_start = (cy as usize) * (self.width as usize);
                    for cx in x..x_end {
                        let c = if (cx % 2 == 0) && (cy % 2 == 0) { 1 } else { 0 };
                        self.pixels[row_start + (cx as usize)] = c;
                    }
                }
            }
            (ColorPalette::Amber, 2) => {
                // 50% checkerboard dither for selection
                for cy in y..y_end {
                    let row_start = (cy as usize) * (self.width as usize);
                    for cx in x..x_end {
                        let c = if (cx + cy) % 2 == 0 { 2 } else { 0 };
                        self.pixels[row_start + (cx as usize)] = c;
                    }
                }
            }
            (ColorPalette::C64, 1) => {
                // Application 50% checkerboard: background blue (0) and dark grey (1).
                for cy in y..y_end {
                    let row_start = (cy as usize) * (self.width as usize);
                    for cx in x..x_end {
                        let c = if (cx + cy) % 2 == 0 { 1 } else { 0 };
                        self.pixels[row_start + (cx as usize)] = c;
                    }
                }
            }
            (ColorPalette::C64, 2) => {
                // 50% checkerboard dither: background blue (0) and light blue (2)
                for cy in y..y_end {
                    let row_start = (cy as usize) * (self.width as usize);
                    for cx in x..x_end {
                        let c = if (cx + cy) % 2 == 0 { 2 } else { 0 };
                        self.pixels[row_start + (cx as usize)] = c;
                    }
                }
            }
            (ColorPalette::Atari, 1) => {
                // Application 50% checkerboard: dark umber (0) and bronze (1).
                for cy in y..y_end {
                    let row_start = (cy as usize) * (self.width as usize);
                    for cx in x..x_end {
                        let c = if (cx + cy) % 2 == 0 { 1 } else { 0 };
                        self.pixels[row_start + (cx as usize)] = c;
                    }
                }
            }
            (ColorPalette::Atari, 2) => {
                // Application 50% checkerboard: dark umber (0) and gold (2).
                for cy in y..y_end {
                    let row_start = (cy as usize) * (self.width as usize);
                    for cx in x..x_end {
                        let c = if (cx + cy) % 2 == 0 { 2 } else { 0 };
                        self.pixels[row_start + (cx as usize)] = c;
                    }
                }
            }
            (ColorPalette::ZxSpectrum, 1) => {
                // ZX Spectrum 50% checkerboard dither: black (0) and blue (1)
                for cy in y..y_end {
                    let row_start = (cy as usize) * (self.width as usize);
                    for cx in x..x_end {
                        let c = if (cx + cy) % 2 == 0 { 1 } else { 0 };
                        self.pixels[row_start + (cx as usize)] = c;
                    }
                }
            }
            (ColorPalette::ZxSpectrum, 2) => {
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
        if w == 0 || h == 0 {
            return;
        }
        self.fill_rect(x, y, w, 1, color_index);
        self.fill_rect(x, y + h - 1, w, 1, color_index);
        self.fill_rect(x, y, 1, h, color_index);
        self.fill_rect(x + w - 1, y, 1, h, color_index);
    }

    pub fn draw_char(&mut self, x: u16, y: u16, ch: char, fg: u8, bg: Option<u8>) {
        let fg = if is_table_border_char(ch) { 7 } else { fg };
        let code = unicode_to_cp437(ch);
        let tall_glyph: Option<(&[u8], u16)> = if self.font_height == 16
            && !matches!(
                self.font_mode,
                PaletteMode::C64 | PaletteMode::Atari | PaletteMode::ZxSpectrum
            ) {
            font_8x16_for_mode(self.font_mode).map(|font16| (font16[code as usize].as_slice(), 16))
        } else if self.font_height == 14
            && matches!(self.font_mode, PaletteMode::Ega | PaletteMode::Vga)
        {
            let start = (code as usize) * 14;
            Some((&EGA_FONT_8X14[start..start + 14], 14))
        } else {
            None
        };
        if let Some((glyph, glyph_height)) = tall_glyph {
            for row in 0..glyph_height {
                let py = y + row;
                if py >= self.height {
                    break;
                }
                let byte = glyph[row as usize];
                let row_start = (py as usize) * (self.width as usize);

                for col in 0..FONT_WIDTH {
                    let px = x + col;
                    if px >= self.width {
                        break;
                    }

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

            for row in 0..self.font_height {
                let py = y + row;
                if py >= self.height {
                    break;
                }
                let source_row = (row as usize * FONT_HEIGHT as usize) / self.font_height as usize;
                let byte = glyph[source_row];
                let row_start = (py as usize) * (self.width as usize);

                for col in 0..FONT_WIDTH {
                    let px = x + col;
                    if px >= self.width {
                        break;
                    }

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
        self.set_color_palette(view.color_palette);
        self.font_mode = view.font_mode;
        self.font_height = view.resolution.line_height();
        self.clear(0); // clear to background

        let font_height = self.font_height;

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
                        if py >= self.height {
                            break;
                        }
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
                    if l.style.underline && l.y + font_height - 1 < self.height {
                        self.fill_rect(
                            l.x,
                            l.y + font_height - 1,
                            l.width,
                            1,
                            l.style.fg.palette_index,
                        );
                    }
                }
            }
        }

        // Draw cursor if visible
        if let Some(cursor) = &view.cursor {
            if cursor.visible && cursor.x < self.width && cursor.y < self.height {
                let cur_h = font_height.saturating_sub(2);
                self.fill_rect(cursor.x, cursor.y, 6, cur_h, 8); // Green block cursor
            }
        }

        // Draw system-specific retro mouse cursor if active
        if let Some((mx, my)) = view.mouse_pos {
            self.draw_mouse_cursor_for_view(view, mx, my);
        }
    }

    /// Finds the visible character under the pointer. Text can start between
    /// grid lines, so a glyph uses its rendered origin; empty space uses the
    /// resolution's character grid.
    pub fn find_char_cell_and_glyph(
        width: u16,
        height: u16,
        mx: u16,
        my: u16,
        view: Option<&View>,
    ) -> (u16, u16, Option<char>) {
        let char_w: u16 = 8;
        let char_h = view.map(|v| v.resolution.line_height()).unwrap_or_else(|| {
            if height == 350 {
                14
            } else if height <= 200 {
                8
            } else {
                16
            }
        });

        // Use the grid only as a fallback for empty cells.
        let cx = ((mx / char_w) * char_w).min(width.saturating_sub(char_w));
        let cy = ((my / char_h) * char_h).min(height.saturating_sub(char_h));

        // 2. Glyph lookup follows paint order. Modal elements are appended after
        // the screen beneath them, so inspect from the topmost element down.
        // An opaque modal background intentionally hides any lower glyph.
        if let Some(v) = view {
            for el in v.elements.iter().rev() {
                match el {
                    Element::Text(t) => {
                        let text_w = (t.text.chars().count() as u16).saturating_mul(char_w);
                        if mx >= t.x
                            && mx < t.x.saturating_add(text_w)
                            && my >= t.y
                            && my < t.y.saturating_add(char_h)
                        {
                            let col_idx = (mx - t.x) / char_w;
                            return (
                                t.x + col_idx * char_w,
                                t.y,
                                t.text.chars().nth(col_idx as usize),
                            );
                        }
                    }
                    Element::Link(l) => {
                        let text_w = (l.text.chars().count() as u16).saturating_mul(char_w);
                        if mx >= l.x
                            && mx < l.x.saturating_add(text_w)
                            && my >= l.y
                            && my < l.y.saturating_add(char_h)
                        {
                            let col_idx = (mx - l.x) / char_w;
                            return (
                                l.x + col_idx * char_w,
                                l.y,
                                l.text.chars().nth(col_idx as usize),
                            );
                        }
                    }
                    Element::Rect(r)
                        if r.filled
                            && mx >= r.x
                            && mx < r.x.saturating_add(r.width)
                            && my >= r.y
                            && my < r.y.saturating_add(r.height) =>
                    {
                        break;
                    }
                    Element::Sprite(s)
                        if mx >= s.x
                            && mx < s.x.saturating_add(s.width)
                            && my >= s.y
                            && my < s.y.saturating_add(s.height) =>
                    {
                        break;
                    }
                    _ => {}
                }
            }
        }

        (cx, cy, None)
    }

    /// Draws the system-specific retro software mouse cursor at `(mx, my)` taking into account active view elements.
    pub fn draw_mouse_cursor_for_view(&mut self, view: &View, mx: u16, my: u16) {
        let (cx, cy, _) =
            Self::find_char_cell_and_glyph(self.width, self.height, mx, my, Some(view));
        let char_h = view.resolution.line_height();
        let cursor_bg = match view.color_palette {
            pixel_ssh_view::ColorPalette::Amber | pixel_ssh_view::ColorPalette::GreenCrt => 6,
            _ => 10,
        };
        let mut counts = [0u16; 256];
        for row in cy..cy.saturating_add(char_h).min(self.height) {
            for col in cx..cx.saturating_add(8).min(self.width) {
                counts[self.pixels[row as usize * self.width as usize + col as usize] as usize] +=
                    1;
            }
        }
        let background = counts
            .iter()
            .enumerate()
            .max_by_key(|(_, count)| *count)
            .map(|(index, _)| index as u8)
            .unwrap_or(0);
        for row in cy..cy.saturating_add(char_h).min(self.height) {
            for col in cx..cx.saturating_add(8).min(self.width) {
                let index = row as usize * self.width as usize + col as usize;
                self.pixels[index] = if self.pixels[index] == background {
                    cursor_bg
                } else {
                    0
                };
            }
        }
    }

    /// Draws the system-specific retro software mouse cursor at `(mx, my)`.
    pub fn draw_mouse_cursor(&mut self, mode: PaletteMode, mx: u16, my: u16) {
        self.draw_mouse_cursor_internal(mode, mx, my);
    }

    fn draw_mouse_cursor_internal(&mut self, mode: PaletteMode, mx: u16, my: u16) {
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
                // and inverts the symbol glyph at the symbol place (to black / 0)
                // for a high-contrast DOS-style cursor.
                let cursor_bg = match mode {
                    PaletteMode::Amber => 6,    // High intensity Amber beam
                    PaletteMode::GreenCrt => 6, // High intensity Green phosphor beam
                    _ => 10,                    // Bright Red
                };

                let (cx, cy, _) =
                    Self::find_char_cell_and_glyph(self.width, self.height, mx, my, None);
                for r in 0..mode.line_height() {
                    let py = cy + r;
                    if py >= self.height {
                        break;
                    }
                    let row_start = (py as usize) * (self.width as usize);
                    for c in 0..8 {
                        let px = cx + c;
                        if px >= self.width {
                            break;
                        }
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
            PaletteMode::C64 => {
                // C64-style 11x14 mouse pointer.
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
                for (r, row) in C64_GEOS_ARROW.iter().enumerate() {
                    for (c, pixel) in row.iter().enumerate() {
                        match pixel {
                            1 => self.set_pixel(mx + c as u16, my + r as u16, 13), // VIC-II Black outline
                            2 => self.set_pixel(mx + c as u16, my + r as u16, 14), // VIC-II White interior
                            _ => {}
                        }
                    }
                }
            }
            PaletteMode::ZxSpectrum => {
                // ZX-style 8x12 mouse pointer.
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
                for (r, row) in ZX_AMX_ARROW.iter().enumerate() {
                    for (c, pixel) in row.iter().enumerate() {
                        match pixel {
                            1 => self.set_pixel(mx + c as u16, my + r as u16, 0), // Sinclair Black outline
                            2 => self.set_pixel(mx + c as u16, my + r as u16, 7), // Bright Cyan core
                            _ => {}
                        }
                    }
                }
            }
            PaletteMode::Atari => {
                // Atari-style 11x16 mouse pointer.
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
                for (r, row) in ATARI_ARROW.iter().enumerate() {
                    for (c, pixel) in row.iter().enumerate() {
                        match pixel {
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
    use pixel_ssh_view::{Color, RectElement, TextElement, TextStyle};

    #[test]
    fn ega_glyphs_stay_within_fourteen_pixel_rows() {
        let mut fb = Framebuffer::new(8, 28);
        fb.font_mode = PaletteMode::Ega;
        fb.font_height = 14;
        fb.draw_char(0, 0, '█', 5, None);
        assert!(fb.pixels[..14 * 8].iter().any(|pixel| *pixel != 0));
        assert!(fb.pixels[14 * 8..].iter().all(|pixel| *pixel == 0));
        let (_, row, _) = Framebuffer::find_char_cell_and_glyph(640, 350, 3, 15, None);
        assert_eq!(row, 14);
    }

    #[test]
    fn ega_uses_its_own_fourteen_scanline_glyph() {
        let mut ega = Framebuffer::new(8, 16);
        ega.font_mode = PaletteMode::Ega;
        ega.font_height = 14;
        ega.draw_char(0, 0, 'A', 5, None);
        let mut vga = Framebuffer::new(8, 16);
        vga.font_mode = PaletteMode::Vga;
        vga.draw_char(0, 0, 'A', 5, None);

        assert!(ega.pixels[11 * 8..12 * 8].iter().all(|pixel| *pixel == 0));
        assert!(vga.pixels[11 * 8..12 * 8].contains(&5));
    }

    #[test]
    fn cga_mouse_cursor_stays_within_eight_pixel_row() {
        let mut fb = Framebuffer::new(8, 16);
        fb.draw_mouse_cursor(PaletteMode::Cga, 0, 0);
        assert!(fb.pixels[..8 * 8].iter().any(|pixel| *pixel != 0));
        assert!(fb.pixels[8 * 8..].iter().all(|pixel| *pixel == 0));
    }

    #[test]
    fn fonts_are_distinct_per_system() {
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
            assert!(non_zero_count > 0, "Mode {mode:?} should render pixel data");
        }
    }

    #[test]
    fn test_vga_8x16_box_drawing_continuous_vertical_lines() {
        let font16 = font_8x16_for_mode(PaletteMode::Vga).expect("VGA 8x16 font should exist");
        let vertical_code = unicode_to_cp437('│') as usize;
        let glyph = &font16[vertical_code];
        for (row, byte) in glyph.iter().enumerate() {
            assert_ne!(
                *byte, 0,
                "Vertical line character '│' must have non-zero bits on row {row}"
            );
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
            assert!(non_zero > 0, "Mode {mode:?} should render cursor pixels");
        }

        // Verify VGA text mode snaps to 8x16 character cell and draws full red character block (index 10, width 8)
        let mut fb_vga = Framebuffer::new(640, 400);
        fb_vga.draw_mouse_cursor(PaletteMode::Vga, 19, 35); // cell (col 2 = x 16..24, row 2 = y 32..48)
                                                            // Red block should be drawn at x = 16..24, y = 32..48
        for row in 32..48 {
            for col in 16..24 {
                let idx = (row as usize) * 640 + (col as usize);
                assert_eq!(
                    fb_vga.pixels[idx], 10,
                    "VGA cursor must draw bright red (index 10) across full 8px cell at ({col}, {row})"
                );
            }
        }

        // Verify glyph inversion when cursor is placed over character text
        let mut fb_text = Framebuffer::new(640, 400);
        fb_text.font_mode = PaletteMode::Vga;
        fb_text.draw_text(16, 32, "A", 14, None);
        let code_a = unicode_to_cp437('A');
        let glyph_a = font::CP437_FONT_8X16[code_a as usize];
        fb_text.draw_mouse_cursor(PaletteMode::Vga, 19, 35);
        for (r, byte) in glyph_a.iter().enumerate() {
            for c in 0..8 {
                let col = 16 + c as u16;
                let row = 32 + r as u16;
                let idx = (row as usize) * 640 + (col as usize);
                if (byte & (0x80 >> c)) != 0 {
                    assert_eq!(
                        fb_text.pixels[idx], 0,
                        "Glyph pixel of 'A' must invert to 0 at ({col}, {row})"
                    );
                } else {
                    assert_eq!(
                        fb_text.pixels[idx], 10,
                        "Background pixel of cell must be 10 at ({col}, {row})"
                    );
                }
            }
        }
    }

    #[test]
    fn theme_glyphs_always_fit_the_resolution_cell_height() {
        use pixel_ssh_view::{InterfaceTheme, ResolutionMode};

        for resolution in ResolutionMode::ALL {
            let (width, height) = resolution.resolution();
            for theme in InterfaceTheme::ALL {
                let mut view = View::new(width, height);
                view.resolution = resolution;
                view.interface_theme = theme;
                view.font_mode = theme.font_mode();
                view.add(Element::Text(TextElement {
                    x: 0,
                    y: 0,
                    text: "A".to_string(),
                    style: TextStyle::new(Color::from_palette(6)),
                }));

                let mut framebuffer = Framebuffer::new(width, height);
                framebuffer.draw_view(&view);
                assert_eq!(framebuffer.font_height, resolution.line_height());
            }
        }
    }

    #[test]
    fn hit_test_uses_the_topmost_modal_glyph() {
        let mut view = View::new(80, 32);
        view.resolution = pixel_ssh_view::ResolutionMode::Cga;
        view.add(Element::Text(TextElement {
            x: 8,
            y: 8,
            text: "P".to_string(),
            style: TextStyle::new(Color::from_palette(5)),
        }));
        view.add(Element::Rect(RectElement {
            x: 0,
            y: 0,
            width: 80,
            height: 32,
            color: Color::from_palette(0),
            filled: true,
        }));
        view.add(Element::Text(TextElement {
            x: 8,
            y: 8,
            text: "S".to_string(),
            style: TextStyle::new(Color::from_palette(6)),
        }));

        let (_, _, glyph) = Framebuffer::find_char_cell_and_glyph(80, 32, 10, 10, Some(&view));
        assert_eq!(glyph, Some('S'));

        let (_, _, blank) = Framebuffer::find_char_cell_and_glyph(80, 32, 42, 10, Some(&view));
        assert_eq!(blank, None, "opaque modal background hides underlying UI");
    }

    #[test]
    fn zx_cursor_inverts_only_the_visible_modal_character_cell() {
        use pixel_ssh_view::ResolutionMode;

        let mut view = View::new(256, 192);
        view.resolution = ResolutionMode::ZxSpectrum;
        // The modal's off-grid text covers the underlying screen text.
        view.add(Element::Text(TextElement {
            x: 8,
            y: 9,
            text: "A".to_string(),
            style: TextStyle::new(Color::from_palette(5)),
        }));
        view.add(Element::Rect(RectElement {
            x: 8,
            y: 8,
            width: 24,
            height: 16,
            color: Color::from_palette(0),
            filled: true,
        }));
        view.add(Element::Text(TextElement {
            x: 8,
            y: 9,
            text: "S".to_string(),
            style: TextStyle::new(Color::from_palette(6)),
        }));

        let mut without_cursor = Framebuffer::new(256, 192);
        without_cursor.draw_view(&view);
        view.mouse_pos = Some((10, 10));
        let mut with_cursor = Framebuffer::new(256, 192);
        with_cursor.draw_view(&view);

        let (cx, cy, glyph) = Framebuffer::find_char_cell_and_glyph(256, 192, 10, 10, Some(&view));
        assert_eq!((cx, cy, glyph), (8, 9, Some('S')));
        for y in 0..192 {
            for x in 0..256 {
                let index = y * 256 + x;
                if (8..16).contains(&x) && (9..17).contains(&y) {
                    let expected = if without_cursor.pixels[index] == 0 {
                        10
                    } else {
                        0
                    };
                    assert_eq!(with_cursor.pixels[index], expected, "pixel ({x}, {y})");
                } else {
                    assert_eq!(
                        with_cursor.pixels[index], without_cursor.pixels[index],
                        "outside cursor ({x}, {y})"
                    );
                }
            }
        }
    }

    #[test]
    fn cursor_uses_rendered_text_origin_and_grid_for_empty_cells() {
        use pixel_ssh_view::{Color, Element, TextElement, TextStyle};

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

        // The top bar begins four pixels below the grid row.
        let (cx1, cy1, ch1) = Framebuffer::find_char_cell_and_glyph(640, 400, 19, 7, Some(&view));
        assert_eq!(cx1, 12);
        assert_eq!(cy1, 4);
        assert_eq!(ch1, Some('D'));

        // The project heading begins six pixels below its grid row.
        let (cx2, cy2, ch2) = Framebuffer::find_char_cell_and_glyph(640, 400, 25, 60, Some(&view));
        assert_eq!(cx2, 24, "cx must snap to column 3 (24)");
        assert_eq!(cy2, 54);
        assert_eq!(ch2, Some('R'));

        let (cx3, cy3, ch3) = Framebuffer::find_char_cell_and_glyph(640, 400, 153, 95, Some(&view));
        assert_eq!((cx3, cy3, ch3), (152, 80, None));
    }

    #[test]
    fn monochrome_palettes_keep_unwanted_rgb_channels_zero() {
        let green_p = Framebuffer::palette_for_mode(PaletteMode::GreenCrt);
        for (i, entry) in green_p.iter().enumerate() {
            assert_eq!(
                entry[0], 0,
                "Green CRT index {} must have 0 red, got {}",
                i, entry[0]
            );
            assert_eq!(
                entry[2], 0,
                "Green CRT index {} must have 0 blue, got {}",
                i, entry[2]
            );
        }

        let amber_p = Framebuffer::palette_for_mode(PaletteMode::Amber);
        for (i, entry) in amber_p.iter().enumerate() {
            assert_eq!(
                entry[2], 0,
                "Amber CRT index {} must have 0 blue, got {}",
                i, entry[2]
            );
        }
    }

    #[test]
    fn test_dithered_fills_produce_alternating_stipple_and_checkerboard() {
        let mut fb_c64 = Framebuffer::new(320, 200);
        fb_c64.font_mode = PaletteMode::C64;
        fb_c64.set_color_palette(ColorPalette::C64);
        fb_c64.fill_rect(0, 0, 4, 4, 1); // C64 50% checkerboard with dark grey (1) and background (0)
        assert_eq!(fb_c64.pixels[0], 1); // (0,0) -> 1
        assert_eq!(fb_c64.pixels[1], 0); // (1,0) -> 0
        assert_eq!(fb_c64.pixels[320], 0); // (0,1) -> 0
        assert_eq!(fb_c64.pixels[321], 1); // (1,1) -> 1

        let mut fb_green = Framebuffer::new(640, 400);
        fb_green.font_mode = PaletteMode::GreenCrt;
        fb_green.set_color_palette(ColorPalette::GreenCrt);
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
        assert_ne!(
            glyph[7], 0,
            "Horizontal line '─' must have active pixels on row 7"
        );
        assert_ne!(
            glyph[8], 0,
            "Horizontal line '─' must have active pixels on row 8"
        );
    }

    #[test]
    fn test_box_drawing_chars_rendered_in_uniform_border_color() {
        let mut fb = Framebuffer::new(640, 400);
        fb.font_mode = PaletteMode::Vga;
        // Even if requested with fg = 5 (body gray), box border characters should be drawn in palette index 7
        fb.draw_text(0, 0, "│ Test │", 5, None);
        // Column 0 is '│', which is at x = 0..8. Row 0..16 should be drawn in color 7
        let p_border = fb.pixels[3];
        assert_eq!(p_border, 7, "Border '│' must be drawn in palette index 7");
    }
}
