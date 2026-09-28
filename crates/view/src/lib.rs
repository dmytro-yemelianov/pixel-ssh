#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SystemMode {
    #[default]
    Vga,
    Svga,
    Sga,
    Ega,
    Cga,
    ZxSpectrum,
    C64,
    Atari,
    Amber,
    GreenCrt,
}

impl SystemMode {
    pub const ALL: [SystemMode; 10] = [
        SystemMode::Svga,
        SystemMode::Sga,
        SystemMode::Vga,
        SystemMode::Ega,
        SystemMode::Cga,
        SystemMode::C64,
        SystemMode::Atari,
        SystemMode::ZxSpectrum,
        SystemMode::Amber,
        SystemMode::GreenCrt,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            SystemMode::Svga => "SVGA 800x600",
            SystemMode::Sga => "SGA 640x480",
            SystemMode::Vga => "VGA 640x400",
            SystemMode::Ega => "EGA 640x350",
            SystemMode::Cga => "CGA 320x200",
            SystemMode::ZxSpectrum => "ZX Spectrum",
            SystemMode::C64 => "Commodore 64",
            SystemMode::Atari => "Atari 800",
            SystemMode::Amber => "Amber CRT",
            SystemMode::GreenCrt => "Green CRT",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            SystemMode::Svga => "SVGA",
            SystemMode::Sga => "SGA",
            SystemMode::Vga => "VGA",
            SystemMode::Ega => "EGA",
            SystemMode::Cga => "CGA",
            SystemMode::ZxSpectrum => "ZX",
            SystemMode::C64 => "C64",
            SystemMode::Atari => "Atari",
            SystemMode::Amber => "Amber",
            SystemMode::GreenCrt => "Green",
        }
    }

    /// Native pixel resolution (width, height)
    pub fn resolution(&self) -> (u16, u16) {
        match self {
            SystemMode::Svga => (800, 600),
            SystemMode::Sga => (640, 480),
            SystemMode::Vga | SystemMode::Amber | SystemMode::GreenCrt => (640, 400),
            SystemMode::Ega => (640, 350),
            SystemMode::Cga | SystemMode::C64 | SystemMode::Atari => (320, 200),
            SystemMode::ZxSpectrum => (256, 192),
        }
    }

    /// Character grid dimensions (cols, rows)
    pub fn char_grid(&self) -> (u16, u16) {
        match self {
            SystemMode::Svga => (100, 37),
            SystemMode::Sga => (80, 30),
            SystemMode::Vga | SystemMode::Amber | SystemMode::GreenCrt => (80, 25),
            SystemMode::Ega => (80, 25),
            SystemMode::Cga | SystemMode::C64 => (40, 25),
            SystemMode::Atari => (40, 24),
            SystemMode::ZxSpectrum => (32, 24),
        }
    }

    /// Physical CRT / display aspect ratio (width_ratio, height_ratio)
    pub fn aspect_ratio(&self) -> (u32, u32) {
        (4, 3)
    }

    /// Pixel line height in the framebuffer (pixels per text row)
    pub fn line_height(&self) -> u16 {
        match self {
            SystemMode::Svga | SystemMode::Sga | SystemMode::Vga | SystemMode::Amber | SystemMode::GreenCrt => 16,
            SystemMode::Ega => 14,
            SystemMode::Cga | SystemMode::C64 | SystemMode::Atari | SystemMode::ZxSpectrum => 8,
        }
    }

    pub fn next(&self) -> Self {
        match self {
            SystemMode::Svga => SystemMode::Sga,
            SystemMode::Sga => SystemMode::Vga,
            SystemMode::Vga => SystemMode::Ega,
            SystemMode::Ega => SystemMode::Cga,
            SystemMode::Cga => SystemMode::C64,
            SystemMode::C64 => SystemMode::Atari,
            SystemMode::Atari => SystemMode::ZxSpectrum,
            SystemMode::ZxSpectrum => SystemMode::Amber,
            SystemMode::Amber => SystemMode::GreenCrt,
            SystemMode::GreenCrt => SystemMode::Svga,
        }
    }

    pub fn from_str_name(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "svga" | "supervga" => SystemMode::Svga,
            "sga" => SystemMode::Sga,
            "ega" => SystemMode::Ega,
            "cga" => SystemMode::Cga,
            "zx" | "zxspectrum" | "spectrum" => SystemMode::ZxSpectrum,
            "c64" | "commodore" | "commodore64" => SystemMode::C64,
            "atari" | "atari2600" | "atari800" => SystemMode::Atari,
            "amber" | "ambercrt" => SystemMode::Amber,
            "green" | "greencrt" | "crt" => SystemMode::GreenCrt,
            _ => SystemMode::Vga,
        }
    }
}

pub type PaletteMode = SystemMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ActiveModal {
    #[default]
    None,
    Visuals,
    Help,
    System,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VisualEffects {
    pub scanlines: f32,
    pub pixel_grid: f32,
    pub chromatic: f32,
    pub afterglow: f32,
    pub curvature: f32,
    pub jitter: f32,
    pub magnet: f32,
    pub noise: f32,
    pub antenna_hum: f32,
}

impl Default for VisualEffects {
    fn default() -> Self {
        Self {
            scanlines: 0.25,
            pixel_grid: 0.16,
            chromatic: 0.0,
            afterglow: 0.15,
            curvature: 0.0,
            jitter: 0.0,
            magnet: 0.0,
            noise: 0.0,
            antenna_hum: 0.0,
        }
    }
}

impl VisualEffects {
    pub fn clean() -> Self {
        Self {
            scanlines: 0.0,
            pixel_grid: 0.0,
            chromatic: 0.0,
            afterglow: 0.0,
            curvature: 0.0,
            jitter: 0.0,
            magnet: 0.0,
            noise: 0.0,
            antenna_hum: 0.0,
        }
    }

    pub fn crt_trinitron() -> Self {
        Self {
            scanlines: 0.35,
            pixel_grid: 0.20,
            chromatic: 0.15,
            afterglow: 0.25,
            curvature: 0.20,
            jitter: 0.0,
            magnet: 0.0,
            noise: 0.0,
            antenna_hum: 0.0,
        }
    }

    pub fn crt_arcade() -> Self {
        Self {
            scanlines: 0.55,
            pixel_grid: 0.35,
            chromatic: 0.45,
            afterglow: 0.30,
            curvature: 0.35,
            jitter: 0.15,
            magnet: 0.0,
            noise: 0.0,
            antenna_hum: 0.0,
        }
    }

    pub fn phosphor_bloom() -> Self {
        Self {
            scanlines: 0.65,
            pixel_grid: 0.40,
            chromatic: 0.0,
            afterglow: 0.75,
            curvature: 0.20,
            jitter: 0.05,
            magnet: 0.0,
            noise: 0.0,
            antenna_hum: 0.0,
        }
    }

    pub fn retro_glitch() -> Self {
        Self {
            scanlines: 0.75,
            pixel_grid: 0.50,
            chromatic: 0.90,
            afterglow: 0.40,
            curvature: 0.40,
            jitter: 0.50,
            magnet: 0.0,
            noise: 0.0,
            antenna_hum: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Platform {
    #[default]
    Web,
    Terminal,
}

#[derive(Debug, Clone, PartialEq)]
pub struct View {
    pub width: u16,
    pub height: u16,
    pub elements: Vec<Element>,
    pub cursor: Option<Cursor>,
    pub mouse_pos: Option<(u16, u16)>,
    pub palette_mode: SystemMode,
    pub system_mode: SystemMode,
    pub visual_effects: VisualEffects,
    pub platform: Platform,
}

impl View {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            elements: Vec::new(),
            cursor: None,
            mouse_pos: None,
            palette_mode: SystemMode::default(),
            system_mode: SystemMode::default(),
            visual_effects: VisualEffects::default(),
            platform: Platform::default(),
        }
    }

    pub fn add(&mut self, element: Element) {
        self.elements.push(element);
    }

    /// Finds any clickable link at coordinate (x, y)
    pub fn link_at(&self, x: u16, y: u16) -> Option<&LinkElement> {
        for element in &self.elements {
            if let Element::Link(link) = element {
                if x >= link.x && x < link.x + link.width && y >= link.y && y <= link.y + link.height + 2 {
                    return Some(link);
                }
            }
        }
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Element {
    Text(TextElement),
    Rect(RectElement),
    Sprite(SpriteElement),
    Link(LinkElement),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub palette_index: u8,
}

impl Color {
    pub const fn from_palette(idx: u8) -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            palette_index: idx,
        }
    }

    pub const fn rgb(r: u8, g: u8, b: u8, palette_index: u8) -> Self {
        Self {
            r,
            g,
            b,
            palette_index,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextStyle {
    pub fg: Color,
    pub bg: Option<Color>,
    pub bold: bool,
    pub underline: bool,
}

impl TextStyle {
    pub const fn new(fg: Color) -> Self {
        Self {
            fg,
            bg: None,
            bold: false,
            underline: false,
        }
    }

    pub const fn with_bg(mut self, bg: Color) -> Self {
        self.bg = Some(bg);
        self
    }

    pub const fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    pub const fn underline(mut self) -> Self {
        self.underline = true;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextElement {
    pub x: u16,
    pub y: u16,
    pub text: String,
    pub style: TextStyle,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkElement {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub text: String,
    pub url: String,
    pub style: TextStyle,
}

impl LinkElement {
    pub fn new(x: u16, y: u16, text: impl Into<String>, url: impl Into<String>, style: TextStyle) -> Self {
        let text = text.into();
        let width = (text.len() as u16) * 8;
        let height = 8;
        Self {
            x,
            y,
            width,
            height,
            text,
            url: url.into(),
            style: style.underline(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RectElement {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub color: Color,
    pub filled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpriteElement {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub data: Vec<u8>, // palette indices
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cursor {
    pub x: u16,
    pub y: u16,
    pub visible: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Key {
    Char(char),
    Enter,
    Backspace,
    Tab,
    Escape,
    Up,
    Down,
    Left,
    Right,
    PageUp,
    PageDown,
    Home,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    Left,
    Right,
    Middle,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputEvent {
    KeyDown(Key),
    KeyUp(Key),
    PointerMove { x: u16, y: u16 },
    PointerDown { x: u16, y: u16, button: Button },
    PointerUp { x: u16, y: u16, button: Button },
    PointerLeave,
    Wheel { dx: i16, dy: i16 },
    Resize { width: u16, height: u16 },
}

/// Word wrap utility that wraps text to fit within `max_chars` per line.
pub fn word_wrap(text: &str, max_chars: usize) -> Vec<String> {
    let mut lines = Vec::new();
    for paragraph in text.split('\n') {
        let trimmed = paragraph.trim();
        if trimmed.is_empty() {
            lines.push(String::new());
            continue;
        }
        let words: Vec<&str> = trimmed.split_whitespace().collect();
        let mut cur_line = String::new();
        for word in words {
            if cur_line.is_empty() {
                cur_line.push_str(word);
            } else if cur_line.len() + 1 + word.len() <= max_chars {
                cur_line.push(' ');
                cur_line.push_str(word);
            } else {
                lines.push(cur_line);
                cur_line = word.to_string();
            }
        }
        if !cur_line.is_empty() {
            lines.push(cur_line);
        }
    }
    lines
}

/// Marquee / horizontal ticker utility for text that exceeds `max_chars`.
/// If text fits, returns it as-is. If longer, smoothly slides across `max_chars`.
pub fn horizontal_scroll(text: &str, max_chars: usize, tick: usize) -> String {
    let char_count = text.chars().count();
    if char_count <= max_chars {
        return text.to_string();
    }
    let ticker = format!("{}   •   ", text);
    let ticker_chars: Vec<char> = ticker.chars().collect();
    let cycle = ticker_chars.len();
    let offset = (tick / 4) % cycle;
    let mut out = String::with_capacity(max_chars);
    for i in 0..max_chars {
        out.push(ticker_chars[(offset + i) % cycle]);
    }
    out
}

/// Returns true if the character is a Unicode box-drawing or table-border character.
pub fn is_table_border_char(ch: char) -> bool {
    matches!(
        ch,
        '─' | '│' | '┌' | '┐' | '└' | '┘' | '├' | '┤' | '┬' | '┴' | '┼'
            | '━' | '┃' | '┏' | '┓' | '┗' | '┛' | '┣' | '┫' | '┳' | '┻' | '╋'
            | '═' | '║' | '╒' | '╓' | '╔' | '╕' | '╖' | '╗' | '╘' | '╙' | '╚'
            | '╛' | '╜' | '╝' | '╞' | '╟' | '╠' | '╡' | '╢' | '╣' | '╤' | '╥'
            | '╦' | '╧' | '╨' | '╩' | '╪' | '╫' | '╬' | '╭' | '╮' | '╯' | '╰'
    )
}
