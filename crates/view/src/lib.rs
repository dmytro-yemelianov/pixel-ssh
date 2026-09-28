#[derive(Debug, Clone, PartialEq, Eq)]
pub struct View {
    pub width: u16,
    pub height: u16,
    pub elements: Vec<Element>,
    pub cursor: Option<Cursor>,
}

impl View {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            elements: Vec::new(),
            cursor: None,
        }
    }

    pub fn add(&mut self, element: Element) {
        self.elements.push(element);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Element {
    Text(TextElement),
    Rect(RectElement),
    Sprite(SpriteElement),
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextElement {
    pub x: u16,
    pub y: u16,
    pub text: String,
    pub style: TextStyle,
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
    PointerDown(Button),
    PointerUp(Button),
    Wheel { dx: i16, dy: i16 },
    Resize { width: u16, height: u16 },
}
