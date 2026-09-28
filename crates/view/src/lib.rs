#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PaletteMode {
    #[default]
    Vga,
    ZxSpectrum,
    C64,
    Atari,
    Amber,
    GreenCrt,
}

impl PaletteMode {
    pub const ALL: [PaletteMode; 6] = [
        PaletteMode::Vga,
        PaletteMode::ZxSpectrum,
        PaletteMode::C64,
        PaletteMode::Atari,
        PaletteMode::Amber,
        PaletteMode::GreenCrt,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            PaletteMode::Vga => "VGA Modern",
            PaletteMode::ZxSpectrum => "ZX Spectrum",
            PaletteMode::C64 => "Commodore 64",
            PaletteMode::Atari => "Atari 2600",
            PaletteMode::Amber => "Amber CRT",
            PaletteMode::GreenCrt => "Green CRT",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            PaletteMode::Vga => "VGA",
            PaletteMode::ZxSpectrum => "ZX",
            PaletteMode::C64 => "C64",
            PaletteMode::Atari => "Atari",
            PaletteMode::Amber => "Amber",
            PaletteMode::GreenCrt => "Green",
        }
    }

    /// Native pixel resolution (width, height)
    pub fn resolution(&self) -> (u16, u16) {
        match self {
            PaletteMode::Vga | PaletteMode::Amber | PaletteMode::GreenCrt => (640, 400),
            PaletteMode::C64 | PaletteMode::Atari => (320, 200),
            PaletteMode::ZxSpectrum => (256, 192),
        }
    }

    /// Character grid dimensions (cols, rows)
    pub fn char_grid(&self) -> (u16, u16) {
        match self {
            PaletteMode::Vga | PaletteMode::Amber | PaletteMode::GreenCrt => (80, 25),
            PaletteMode::C64 => (40, 25),
            PaletteMode::Atari => (40, 25),
            PaletteMode::ZxSpectrum => (32, 24),
        }
    }

    /// Physical CRT / display aspect ratio (width_ratio, height_ratio)
    pub fn aspect_ratio(&self) -> (u32, u32) {
        (4, 3)
    }

    /// Pixel line height in the framebuffer (pixels per text row)
    pub fn line_height(&self) -> u16 {
        match self {
            PaletteMode::Vga | PaletteMode::Amber | PaletteMode::GreenCrt => 16,
            PaletteMode::C64 | PaletteMode::Atari | PaletteMode::ZxSpectrum => 8,
        }
    }

    pub fn next(&self) -> Self {
        match self {
            PaletteMode::Vga => PaletteMode::ZxSpectrum,
            PaletteMode::ZxSpectrum => PaletteMode::C64,
            PaletteMode::C64 => PaletteMode::Atari,
            PaletteMode::Atari => PaletteMode::Amber,
            PaletteMode::Amber => PaletteMode::GreenCrt,
            PaletteMode::GreenCrt => PaletteMode::Vga,
        }
    }

    pub fn from_str_name(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "zx" | "zxspectrum" | "spectrum" => PaletteMode::ZxSpectrum,
            "c64" | "commodore" | "commodore64" => PaletteMode::C64,
            "atari" | "atari2600" | "atari800" => PaletteMode::Atari,
            "amber" | "ambercrt" => PaletteMode::Amber,
            "green" | "greencrt" | "crt" => PaletteMode::GreenCrt,
            _ => PaletteMode::Vga,
        }
    }
}

pub type SystemMode = PaletteMode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct View {
    pub width: u16,
    pub height: u16,
    pub elements: Vec<Element>,
    pub cursor: Option<Cursor>,
    pub palette_mode: PaletteMode,
}

impl View {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            elements: Vec::new(),
            cursor: None,
            palette_mode: PaletteMode::default(),
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
