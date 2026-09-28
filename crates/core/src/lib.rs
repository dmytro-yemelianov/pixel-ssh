use pixel_ssh_view::{
    horizontal_scroll, word_wrap, Color, Cursor, Element, InputEvent, Key, LinkElement,
    PaletteMode, RectElement, TextElement, TextStyle, View,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Projects,
    Resume,
    Contact,
    Help,
}

#[derive(Debug, Clone)]
pub struct Project {
    pub title: &'static str,
    pub slug: &'static str,
    pub tldr: &'static str,
    pub tags: &'static [&'static str],
    pub wip: bool,
}

pub static PROJECTS: &[Project] = &[
    Project {
        title: "RAPS",
        slug: "raps",
        tldr: "An open-core Rust CLI and MCP server that exposes 16 Autodesk Platform Services APIs to AI agents via typed contracts and 51 robust tools.",
        tags: &["rust", "mcp", "agents", "cad", "python"],
        wip: false,
    },
    Project {
        title: "Dry",
        slug: "dry",
        tldr: "A typed, units-aware multi-level IR and Rust engine for algorithmic machine toolpaths (FFF/CNC/laser/robot) — LLVM/MLIR for machine motion.",
        tags: &["rust", "compilers", "cnc", "3d-printing"],
        wip: false,
    },
    Project {
        title: "Glueball",
        slug: "glueball",
        tldr: "A CAD-agnostic routed-assembly solver for frames, weldments, tube & pipe, and wire harnesses that compiles 3D graphs into deterministic build plans without linking a geometry kernel.",
        tags: &["rust", "cad-engine", "solvers", "compilers"],
        wip: false,
    },
    Project {
        title: "Klen",
        slug: "klen",
        tldr: "A universal kernel for describing, executing, and formally verifying system safety contracts and state machine invariants in Lean 4.",
        tags: &["lean-4", "formal-methods", "verification", "rust"],
        wip: false,
    },
    Project {
        title: "FTTH CAD Lineage",
        slug: "ftth-cad-lineage",
        tldr: "Four generations of FTTH design tooling (FiberCAD -> Gluon -> Photon -> Muon). Gluon replaces an LLM-authors-the-drawing approach with a deterministic pipeline.",
        tags: &["python", "autocad", "agents", "llm"],
        wip: false,
    },
    Project {
        title: "VPA",
        slug: "vpa",
        tldr: "Verifiable Physical Agent: A domain-specific language, optimizing compiler, and formally verified VM for hard real-time physical automation.",
        tags: &["compilers", "formal-verification", "assembly"],
        wip: false,
    },
    Project {
        title: "Barvinca",
        slug: "barvinca",
        tldr: "A headless, high-throughput digital asset management and vector transformation engine deployed on Cloudflare Workers.",
        tags: &["typescript", "cloudflare-workers", "turborepo"],
        wip: false,
    },
    Project {
        title: "Upyr",
        slug: "upyr",
        tldr: "An offline-first, local privacy-preserving PII redaction and named-entity sanitization tool in pure Rust for LLM context ingestion pipelines.",
        tags: &["rust", "nlp", "privacy"],
        wip: false,
    },
    Project {
        title: "Dryer",
        slug: "dryer",
        tldr: "Embedded firmware and deterministic control loop for custom active filament drying chamber with closed-loop PID humidity regulation.",
        tags: &["rust", "embedded", "3d-printing"],
        wip: false,
    },
    Project {
        title: "Verbacorpus",
        slug: "verbacorpus",
        tldr: "A structured lexicon and etymological database API of Ukrainian language with linguistic graph search.",
        tags: &["typescript", "pwa", "api"],
        wip: false,
    },
    Project {
        title: "Ukrainian Pattern Archive",
        slug: "ukrainian-pattern-archive",
        tldr: "A structured, deduplicated corpus of Ukrainian vyshyvanka folk-embroidery charts powered by an OCR and computational geometry pipeline.",
        tags: &["python", "ocr", "computational-geometry"],
        wip: false,
    },
    Project {
        title: "Nasinnia",
        slug: "nasinnia",
        tldr: "A raster-first API and rendering engine for historical Ukrainian typography, pixel fonts, and bitmap glyph matrix generation.",
        tags: &["typescript", "typography", "canvas"],
        wip: false,
    },
    Project {
        title: "Criticality Lab",
        slug: "criticality-lab",
        tldr: "Interactive GPU-accelerated simulation of self-organized criticality, sandpile models, and percolation thresholds using WebGPU.",
        tags: &["rust", "wgpu", "simulation"],
        wip: false,
    },
    Project {
        title: "Tawara",
        slug: "tawara",
        tldr: "A WebAssembly runtime verification harness and memory assertion tool for sandboxed untrusted plugin evaluation.",
        tags: &["rust", "wasm", "verification"],
        wip: false,
    },
    Project {
        title: "Media Glitch",
        slug: "media-glitch",
        tldr: "Deterministic databending and bitstream corruptor implemented in WebAssembly for algorithmic visual glitch generation.",
        tags: &["rust", "wasm", "glitch-art"],
        wip: false,
    },
    Project {
        title: "Emoji Madness",
        slug: "emoji-madness",
        tldr: "Metal-accelerated cellular automata and particle system generating generative dynamic wallpaper simulations.",
        tags: &["swift", "metal", "generative-art"],
        wip: false,
    },
];

pub const RESUME_MAX_SCROLL: usize = 28;

pub struct App {
    pub current_tab: Tab,
    pub selected_project: usize,
    pub show_detail: bool,
    pub status: String,
    pub scroll_offset: usize,
    pub resume_scroll: usize,
    pub palette_mode: PaletteMode,
    pub tick: usize,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            current_tab: Tab::Projects,
            selected_project: 0,
            show_detail: false,
            status: String::from("Ready. [1-4] Tabs | [P] Palette | [Enter] Detail | [j/k/Wheel] Scroll"),
            scroll_offset: 0,
            resume_scroll: 0,
            palette_mode: PaletteMode::default(),
            tick: 0,
        }
    }

    /// Updates the application state with a normalized input event.
    /// Returns `true` if the state changed and a re-render is required (`dirty`).
    pub fn update(&mut self, event: InputEvent) -> bool {
        self.tick = self.tick.wrapping_add(1);
        match event {
            InputEvent::KeyDown(key) => match key {
                Key::Char('1') => {
                    self.current_tab = Tab::Projects;
                    self.show_detail = false;
                    true
                }
                Key::Char('2') => {
                    self.current_tab = Tab::Resume;
                    self.show_detail = false;
                    true
                }
                Key::Char('3') => {
                    self.current_tab = Tab::Contact;
                    self.show_detail = false;
                    true
                }
                Key::Char('4') => {
                    self.current_tab = Tab::Help;
                    self.show_detail = false;
                    true
                }
                Key::Char('p') | Key::Char('P') => {
                    self.palette_mode = self.palette_mode.next();
                    let (w, h) = self.palette_mode.resolution();
                    let (c, r) = self.palette_mode.char_grid();
                    self.status = format!("{}: {}x{} ({}x{})", self.palette_mode.name(), w, h, c, r);
                    true
                }
                Key::Tab => {
                    self.current_tab = match self.current_tab {
                        Tab::Projects => Tab::Resume,
                        Tab::Resume => Tab::Contact,
                        Tab::Contact => Tab::Help,
                        Tab::Help => Tab::Projects,
                    };
                    self.show_detail = false;
                    true
                }
                Key::Up | Key::Char('k') => {
                    if self.current_tab == Tab::Resume {
                        if self.resume_scroll > 0 {
                            self.resume_scroll -= 1;
                            return true;
                        }
                    } else if self.current_tab == Tab::Projects && !self.show_detail {
                        if self.selected_project > 0 {
                            self.selected_project -= 1;
                            if self.selected_project < self.scroll_offset {
                                self.scroll_offset = self.selected_project;
                            }
                            return true;
                        }
                    }
                    false
                }
                Key::Down | Key::Char('j') => {
                    if self.current_tab == Tab::Resume {
                        if self.resume_scroll < RESUME_MAX_SCROLL {
                            self.resume_scroll += 1;
                            return true;
                        }
                    } else if self.current_tab == Tab::Projects && !self.show_detail {
                        if self.selected_project + 1 < PROJECTS.len() {
                            self.selected_project += 1;
                            if self.selected_project >= self.scroll_offset + 12 {
                                self.scroll_offset = self.selected_project - 11;
                            }
                            return true;
                        }
                    }
                    false
                }
                Key::PageUp => {
                    if self.current_tab == Tab::Resume {
                        self.resume_scroll = self.resume_scroll.saturating_sub(6);
                        return true;
                    }
                    false
                }
                Key::PageDown => {
                    if self.current_tab == Tab::Resume {
                        self.resume_scroll = (self.resume_scroll + 6).min(RESUME_MAX_SCROLL);
                        return true;
                    }
                    false
                }
                Key::Enter => {
                    if self.current_tab == Tab::Projects {
                        self.show_detail = !self.show_detail;
                        true
                    } else {
                        false
                    }
                }
                Key::Escape | Key::Char('q') => {
                    if self.show_detail {
                        self.show_detail = false;
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            },
            InputEvent::Wheel { dy, .. } => {
                if self.current_tab == Tab::Resume {
                    let delta = (dy.abs() as usize).max(1);
                    if dy > 0 {
                        self.resume_scroll = (self.resume_scroll + delta).min(RESUME_MAX_SCROLL);
                    } else if dy < 0 {
                        self.resume_scroll = self.resume_scroll.saturating_sub(delta);
                    }
                    return true;
                } else if self.current_tab == Tab::Projects && !self.show_detail {
                    if dy > 0 && self.selected_project + 1 < PROJECTS.len() {
                        self.selected_project += 1;
                        if self.selected_project >= self.scroll_offset + 12 {
                            self.scroll_offset = self.selected_project - 11;
                        }
                        return true;
                    } else if dy < 0 && self.selected_project > 0 {
                        self.selected_project -= 1;
                        if self.selected_project < self.scroll_offset {
                            self.scroll_offset = self.selected_project;
                        }
                        return true;
                    }
                }
                false
            }
            InputEvent::PointerDown { x, y, .. } => {
                let (cols, _rows) = self.palette_mode.char_grid();
                let (width, height) = self.palette_mode.resolution();

                // 1. Tab bar clicks & Mocked System switcher button
                let tab_y_range = if cols == 80 { 24..=48 } else { 10..=22 };
                if tab_y_range.contains(&y) {
                    if cols == 80 {
                        if x >= 8 && x < 125 {
                            self.current_tab = Tab::Projects;
                            self.show_detail = false;
                            return true;
                        } else if x >= 125 && x < 240 {
                            self.current_tab = Tab::Resume;
                            self.show_detail = false;
                            return true;
                        } else if x >= 240 && x < 355 {
                            self.current_tab = Tab::Contact;
                            self.show_detail = false;
                            return true;
                        } else if x >= 355 && x < 460 {
                            self.current_tab = Tab::Help;
                            self.show_detail = false;
                            return true;
                        } else if x >= 460 && x <= width {
                            self.palette_mode = self.palette_mode.next();
                            let (w, h) = self.palette_mode.resolution();
                            let (c, r) = self.palette_mode.char_grid();
                            self.status = format!("{}: {}x{} ({}x{})", self.palette_mode.name(), w, h, c, r);
                            return true;
                        }
                    } else if cols == 40 {
                        if x < 56 {
                            self.current_tab = Tab::Projects;
                            self.show_detail = false;
                            return true;
                        } else if x < 112 {
                            self.current_tab = Tab::Resume;
                            self.show_detail = false;
                            return true;
                        } else if x < 168 {
                            self.current_tab = Tab::Contact;
                            self.show_detail = false;
                            return true;
                        } else if x < 216 {
                            self.current_tab = Tab::Help;
                            self.show_detail = false;
                            return true;
                        } else {
                            self.palette_mode = self.palette_mode.next();
                            let (w, h) = self.palette_mode.resolution();
                            let (c, r) = self.palette_mode.char_grid();
                            self.status = format!("{}: {}x{} ({}x{})", self.palette_mode.name(), w, h, c, r);
                            return true;
                        }
                    } else {
                        // 32 cols (ZX Spectrum)
                        if x < 48 {
                            self.current_tab = Tab::Projects;
                            self.show_detail = false;
                            return true;
                        } else if x < 98 {
                            self.current_tab = Tab::Resume;
                            self.show_detail = false;
                            return true;
                        } else if x < 148 {
                            self.current_tab = Tab::Contact;
                            self.show_detail = false;
                            return true;
                        } else if x < 188 {
                            self.current_tab = Tab::Help;
                            self.show_detail = false;
                            return true;
                        } else {
                            self.palette_mode = self.palette_mode.next();
                            let (w, h) = self.palette_mode.resolution();
                            let (c, r) = self.palette_mode.char_grid();
                            self.status = format!("{}: {}x{} ({}x{})", self.palette_mode.name(), w, h, c, r);
                            return true;
                        }
                    }
                }

                // 2. Resume scrollbar interaction
                if self.current_tab == Tab::Resume {
                    let scrollbar_x_start = width.saturating_sub(18);
                    if x >= scrollbar_x_start {
                        let top_arrow_y = if cols == 80 { 96..=116 } else { 40..=54 };
                        let bot_arrow_y = if cols == 80 { 334..=354 } else { (height.saturating_sub(35))..=(height.saturating_sub(18)) };
                        if top_arrow_y.contains(&y) {
                            self.resume_scroll = self.resume_scroll.saturating_sub(2);
                            return true;
                        } else if bot_arrow_y.contains(&y) {
                            self.resume_scroll = (self.resume_scroll + 2).min(RESUME_MAX_SCROLL);
                            return true;
                        } else {
                            let track_top = if cols == 80 { 116.0 } else { 54.0 };
                            let track_bot = if cols == 80 { 334.0 } else { (height.saturating_sub(35)) as f32 };
                            if (y as f32) > track_top && (y as f32) < track_bot {
                                let ratio = (y as f32 - track_top) / (track_bot - track_top);
                                self.resume_scroll = ((ratio * RESUME_MAX_SCROLL as f32).round() as usize).min(RESUME_MAX_SCROLL);
                                return true;
                            }
                        }
                    }
                }

                // 3. Project list interaction
                if self.current_tab == Tab::Projects {
                    if self.show_detail {
                        let back_y = if cols == 80 { 330..=365 } else { (height.saturating_sub(30))..=(height.saturating_sub(8)) };
                        if back_y.contains(&y) {
                            self.show_detail = false;
                            return true;
                        }
                    } else {
                        let list_top = if cols == 80 { 56 } else { 24 };
                        let row_h = if cols == 80 { 24 } else if cols == 40 { 12 } else { 11 };
                        if y >= list_top && y < height.saturating_sub(16) {
                            let row_idx = ((y - list_top) / row_h) as usize;
                            let project_idx = self.scroll_offset + row_idx;
                            if project_idx < PROJECTS.len() {
                                if self.selected_project == project_idx {
                                    self.show_detail = true;
                                } else {
                                    self.selected_project = project_idx;
                                }
                                return true;
                            }
                        }
                    }
                }
                false
            }
            InputEvent::Resize { .. } => true,
            _ => false,
        }
    }

    /// Renders the platform-independent `View` model matching the active mocked system.
    pub fn render(&self) -> View {
        let (width, height) = self.palette_mode.resolution();
        let (cols, _rows) = self.palette_mode.char_grid();
        let mut view = View::new(width, height);
        view.palette_mode = self.palette_mode;

        if cols == 80 {
            // Header background (palette index 1 = dark card)
            view.add(Element::Rect(RectElement {
                x: 0,
                y: 0,
                width,
                height: 24,
                color: Color::from_palette(1),
                filled: true,
            }));

            // Title text
            view.add(Element::Text(TextElement {
                x: 12,
                y: 4,
                text: "DMYTRO YEMELIANOV - SYSTEMS & AI ARCHITECT".to_string(),
                style: TextStyle::new(Color::from_palette(7)).bold(),
            }));

            // Header link to personal website
            view.add(Element::Link(LinkElement::new(
                470,
                4,
                "[ yemelianov.dev ]",
                "https://yemelianov.dev",
                TextStyle::new(Color::from_palette(12)),
            )));

            // Tab bar background
            view.add(Element::Rect(RectElement {
                x: 0,
                y: 25,
                width,
                height: 23,
                color: Color::from_palette(0),
                filled: true,
            }));

            let tabs = [
                (Tab::Projects, "[1] Projects", 12),
                (Tab::Resume, "[2] Resume", 128),
                (Tab::Contact, "[3] Contact", 238),
                (Tab::Help, "[4] Help", 348),
            ];

            for (tab, label, tx) in tabs {
                let active = self.current_tab == tab;
                let style = if active {
                    TextStyle::new(Color::from_palette(6))
                        .with_bg(Color::from_palette(7))
                        .bold()
                } else {
                    TextStyle::new(Color::from_palette(4))
                };
                view.add(Element::Text(TextElement {
                    x: tx,
                    y: 28,
                    text: label.to_string(),
                    style,
                }));
            }

            let pal_text = format!("[P] Sys: {}", self.palette_mode.name());
            view.add(Element::Link(LinkElement::new(
                464,
                28,
                pal_text,
                "#palette",
                TextStyle::new(Color::from_palette(9)).bold(),
            )));

            view.add(Element::Rect(RectElement {
                x: 0,
                y: 49,
                width,
                height: 1,
                color: Color::from_palette(3),
                filled: true,
            }));
        } else if cols == 40 {
            // 40 columns (Commodore 64 / Atari 800)
            view.add(Element::Rect(RectElement {
                x: 0,
                y: 0,
                width,
                height: 10,
                color: Color::from_palette(1),
                filled: true,
            }));

            view.add(Element::Text(TextElement {
                x: 4,
                y: 1,
                text: "D.YEMELIANOV - AI ARCHITECT".to_string(),
                style: TextStyle::new(Color::from_palette(7)).bold(),
            }));

            view.add(Element::Link(LinkElement::new(
                248,
                1,
                "[dev]",
                "https://yemelianov.dev",
                TextStyle::new(Color::from_palette(12)),
            )));

            view.add(Element::Rect(RectElement {
                x: 0,
                y: 10,
                width,
                height: 10,
                color: Color::from_palette(0),
                filled: true,
            }));

            let tabs = [
                (Tab::Projects, "[1]Proj", 4),
                (Tab::Resume, "[2]Res", 60),
                (Tab::Contact, "[3]Cont", 116),
                (Tab::Help, "[4]?", 172),
            ];

            for (tab, label, tx) in tabs {
                let active = self.current_tab == tab;
                let style = if active {
                    TextStyle::new(Color::from_palette(6))
                        .with_bg(Color::from_palette(7))
                        .bold()
                } else {
                    TextStyle::new(Color::from_palette(4))
                };
                view.add(Element::Text(TextElement {
                    x: tx,
                    y: 11,
                    text: label.to_string(),
                    style,
                }));
            }

            let pal_text = format!("[P]{}", self.palette_mode.short_name());
            view.add(Element::Link(LinkElement::new(
                224,
                11,
                pal_text,
                "#palette",
                TextStyle::new(Color::from_palette(9)).bold(),
            )));

            view.add(Element::Rect(RectElement {
                x: 0,
                y: 20,
                width,
                height: 1,
                color: Color::from_palette(3),
                filled: true,
            }));
        } else {
            // 32 columns (ZX Spectrum)
            view.add(Element::Rect(RectElement {
                x: 0,
                y: 0,
                width,
                height: 10,
                color: Color::from_palette(1),
                filled: true,
            }));

            view.add(Element::Text(TextElement {
                x: 2,
                y: 1,
                text: "D.YEMELIANOV - ARCHITECT".to_string(),
                style: TextStyle::new(Color::from_palette(7)).bold(),
            }));

            view.add(Element::Rect(RectElement {
                x: 0,
                y: 10,
                width,
                height: 10,
                color: Color::from_palette(0),
                filled: true,
            }));

            let tabs = [
                (Tab::Projects, "[1]PRJ", 2),
                (Tab::Resume, "[2]RES", 52),
                (Tab::Contact, "[3]CNT", 102),
                (Tab::Help, "[4]?", 152),
            ];

            for (tab, label, tx) in tabs {
                let active = self.current_tab == tab;
                let style = if active {
                    TextStyle::new(Color::from_palette(6))
                        .with_bg(Color::from_palette(7))
                        .bold()
                } else {
                    TextStyle::new(Color::from_palette(4))
                };
                view.add(Element::Text(TextElement {
                    x: tx,
                    y: 11,
                    text: label.to_string(),
                    style,
                }));
            }

            let pal_text = format!("[P]{}", self.palette_mode.short_name());
            view.add(Element::Link(LinkElement::new(
                192,
                11,
                pal_text,
                "#palette",
                TextStyle::new(Color::from_palette(9)).bold(),
            )));

            view.add(Element::Rect(RectElement {
                x: 0,
                y: 20,
                width,
                height: 1,
                color: Color::from_palette(3),
                filled: true,
            }));
        }

        // Content Area
        match self.current_tab {
            Tab::Projects => self.render_projects(&mut view),
            Tab::Resume => self.render_resume(&mut view),
            Tab::Contact => self.render_contact(&mut view),
            Tab::Help => self.render_help(&mut view),
        }

        // Footer status bar
        if cols == 80 {
            view.add(Element::Rect(RectElement {
                x: 0,
                y: 376,
                width,
                height: 24,
                color: Color::from_palette(1),
                filled: true,
            }));

            view.add(Element::Text(TextElement {
                x: 12,
                y: 382,
                text: self.status.clone(),
                style: TextStyle::new(Color::from_palette(8)),
            }));

            view.cursor = Some(Cursor {
                x: 620,
                y: 382,
                visible: true,
            });
        } else if cols == 40 {
            view.add(Element::Rect(RectElement {
                x: 0,
                y: 190,
                width,
                height: 10,
                color: Color::from_palette(1),
                filled: true,
            }));

            view.add(Element::Text(TextElement {
                x: 4,
                y: 191,
                text: "[1-4]Tabs [P]Sys [j/k]Scroll [Enter]".to_string(),
                style: TextStyle::new(Color::from_palette(8)),
            }));

            view.cursor = Some(Cursor {
                x: 308,
                y: 191,
                visible: true,
            });
        } else {
            // 32 cols (ZX Spectrum)
            view.add(Element::Rect(RectElement {
                x: 0,
                y: 182,
                width,
                height: 10,
                color: Color::from_palette(1),
                filled: true,
            }));

            view.add(Element::Text(TextElement {
                x: 2,
                y: 183,
                text: "1-4:Tab P:Sys k/j:Nav ?:Help".to_string(),
                style: TextStyle::new(Color::from_palette(8)),
            }));

            view.cursor = Some(Cursor {
                x: 246,
                y: 183,
                visible: true,
            });
        }

        view
    }

    fn render_projects(&self, view: &mut View) {
        let (cols, _) = self.palette_mode.char_grid();
        match cols {
            80 => self.render_projects_80(view),
            40 => self.render_projects_40(view),
            _ => self.render_projects_32(view),
        }
    }

    fn render_projects_80(&self, view: &mut View) {
        if self.show_detail {
            let p = &PROJECTS[self.selected_project];

            view.add(Element::Text(TextElement {
                x: 16,
                y: 58,
                text: format!("PROJECT // {}", p.title),
                style: TextStyle::new(Color::from_palette(12)).bold(),
            }));

            let status_badge = if p.wip {
                "STATUS: In Active Development"
            } else {
                "STATUS: Production-Ready / Stable Open-Source"
            };
            view.add(Element::Text(TextElement {
                x: 16,
                y: 76,
                text: status_badge.to_string(),
                style: TextStyle::new(if p.wip { Color::from_palette(10) } else { Color::from_palette(8) }),
            }));

            view.add(Element::Text(TextElement {
                x: 16,
                y: 94,
                text: format!("TAGS: {}", p.tags.join(", ")),
                style: TextStyle::new(Color::from_palette(11)),
            }));

            // Clickable repository link
            let gh_url = format!("https://github.com/dmytro-yemelianov/{}", p.slug);
            view.add(Element::Link(LinkElement::new(
                16,
                116,
                format!("[ Open Repository: {} ]", gh_url),
                gh_url,
                TextStyle::new(Color::from_palette(7)),
            )));

            // Divider line
            view.add(Element::Rect(RectElement {
                x: 16,
                y: 136,
                width: 608,
                height: 1,
                color: Color::from_palette(3),
                filled: true,
            }));

            view.add(Element::Text(TextElement {
                x: 16,
                y: 148,
                text: "ARCHITECTURAL OVERVIEW & SPECIFICATION:".to_string(),
                style: TextStyle::new(Color::from_palette(6)).bold(),
            }));

            // Word wrap TL;DR description across 74 columns
            let wrapped_lines = word_wrap(p.tldr, 74);
            let mut y = 168;
            for line in wrapped_lines {
                view.add(Element::Text(TextElement {
                    x: 16,
                    y,
                    text: line,
                    style: TextStyle::new(Color::from_palette(5)),
                }));
                y += 16;
            }

            // Architecture / structural ASCII diagrams
            y += 4;
            let diagram: &[&str] = match p.slug {
                "raps" => &[
                    "┌─────────────────┐       ┌──────────────────────┐       ┌─────────────────┐",
                    "│ AI Agent / LLM  │ ───►  │ RAPS MCP Tool Router │ ───►  │ Autodesk APIs   │",
                    "│ (Claude Code)   │ ◄───  │ 51 Typed Contracts   │ ◄───  │ Data Mgmt, OSS, │",
                    "│                 │ JSON  │ Auth & Secret Mgmt   │ HTTPS │ Model Deriv, ACC│",
                    "└─────────────────┘       └──────────────────────┘       └─────────────────┘",
                ],
                "dry" => &[
                    "┌────────────────┐      ┌─────────────────────────┐      ┌─────────────────┐",
                    "│ Parametric AST │ ──►  │ Dry Motion Compiler IR  │ ──►  │ Hardware Output │",
                    "│ Curve Geometry │      │ • Units-aware validation│      │ • G-Code (CNC)  │",
                    "│ Tool Spec      │      │ • Spatial acceleration  │      │ • 3D Print FFF  │",
                    "└────────────────┘      └─────────────────────────┘      └─────────────────┘",
                ],
                "glueball" => &[
                    "┌────────────────┐      ┌─────────────────────────┐      ┌─────────────────┐",
                    "│ 3D Frame Graph │ ──►  │ CAD-Agnostic Solver     │ ──►  │ Cut & Weld      │",
                    "│ Routed Ports   │      │ • Kinematic constraints │      │ Deterministic   │",
                    "│ Structural Rays│      │ • Zero-kernel dependency│      │ Assembly Plan   │",
                    "└────────────────┘      └─────────────────────────┘      └─────────────────┘",
                ],
                "klen" => &[
                    "┌────────────────┐      ┌─────────────────────────┐      ┌─────────────────┐",
                    "│ Safety Contract│ ──►  │ Lean 4 Verified Kernel  │ ──►  │ Executable Rust │",
                    "│ Invariants     │      │ • Inductive proofs      │      │ Safe Determinism│",
                    "│ State Machine  │      │ • Machine-checked specs │      │ Zero Violations │",
                    "└────────────────┘      └─────────────────────────┘      └─────────────────┘",
                ],
                _ => &[
                    "┌───────────────────────────┬──────────────────────────────────────────────┐",
                    "│ Component / Attribute     │ Implementation Specification                 │",
                    "├───────────────────────────┼──────────────────────────────────────────────┤",
                    "│ Core Runtime & Engine     │ Rust (Safe, zero-overhead memory runtime)    │",
                    "│ Target Distribution       │ WebAssembly / Native Container CLI           │",
                    "│ Verification & Test Suite │ Deterministic regression & contract harness  │",
                    "└───────────────────────────┴──────────────────────────────────────────────┘",
                ],
            };

            for line in diagram {
                if y + 14 < 340 {
                    view.add(Element::Text(TextElement {
                        x: 16,
                        y,
                        text: line.to_string(),
                        style: TextStyle::new(Color::from_palette(7)),
                    }));
                    y += 14;
                }
            }

            // Clickable Return button
            view.add(Element::Link(LinkElement::new(
                16,
                344,
                "< [ESC] Return to Project List >",
                "#back",
                TextStyle::new(Color::from_palette(9)).bold(),
            )));
            return;
        }

        // Project List (Up to 12 visible at 640x400)
        let max_visible = 12;
        let start = self.scroll_offset;
        let end = (start + max_visible).min(PROJECTS.len());

        for (i, p) in PROJECTS[start..end].iter().enumerate() {
            let idx = start + i;
            let is_sel = idx == self.selected_project;
            let y = 56 + (i as u16) * 24;

            if is_sel {
                view.add(Element::Rect(RectElement {
                    x: 8,
                    y: y - 2,
                    width: 624,
                    height: 22,
                    color: Color::from_palette(2),
                    filled: true,
                }));
            }

            let marker = if is_sel { ">" } else { " " };
            let fg = if is_sel {
                Color::from_palette(6)
            } else if p.wip {
                Color::from_palette(10)
            } else {
                Color::from_palette(5)
            };

            let num = format!("{:02}", idx + 1);
            let title = if p.title.len() > 22 {
                &p.title[..22]
            } else {
                p.title
            };

            view.add(Element::Text(TextElement {
                x: 16,
                y: y + 2,
                text: format!("{} [{}] {:<22}", marker, num, title),
                style: TextStyle::new(fg).bold(),
            }));

            // Tags formatted with horizontal auto-scroll when selected or long
            let tag_str = p.tags.iter().map(|t| format!("<{}>", t)).collect::<Vec<_>>().join(" ");
            let tag_display = if is_sel {
                horizontal_scroll(&tag_str, 34, self.tick)
            } else if tag_str.len() > 34 {
                format!("{}...", &tag_str[..31])
            } else {
                tag_str
            };

            view.add(Element::Text(TextElement {
                x: 260,
                y: y + 2,
                text: tag_display,
                style: TextStyle::new(Color::from_palette(4)),
            }));

            // Detail arrow
            view.add(Element::Text(TextElement {
                x: 550,
                y: y + 2,
                text: if is_sel { "[Enter] ->" } else { "  Details " }.to_string(),
                style: TextStyle::new(if is_sel { Color::from_palette(7) } else { Color::from_palette(3) }),
            }));
        }

        // Scroll guidance note if more items exist
        if PROJECTS.len() > max_visible {
            view.add(Element::Text(TextElement {
                x: 16,
                y: 352,
                text: format!("Showing {}-{} of {} projects. Use [Up/Down] or [k/j] to scroll.", start + 1, end, PROJECTS.len()),
                style: TextStyle::new(Color::from_palette(4)),
            }));
        }
    }

    fn render_projects_40(&self, view: &mut View) {
        if self.show_detail {
            let p = &PROJECTS[self.selected_project];

            view.add(Element::Text(TextElement {
                x: 4,
                y: 24,
                text: format!("PRJ // {}", p.title),
                style: TextStyle::new(Color::from_palette(12)).bold(),
            }));

            let status_badge = if p.wip { "STATUS: WIP / Dev" } else { "STATUS: Stable / Open" };
            view.add(Element::Text(TextElement {
                x: 4,
                y: 34,
                text: status_badge.to_string(),
                style: TextStyle::new(if p.wip { Color::from_palette(10) } else { Color::from_palette(8) }),
            }));

            let tag_str = format!("TAGS: {}", p.tags.join(", "));
            let tag_display = if tag_str.len() > 38 { format!("{}...", &tag_str[..35]) } else { tag_str };
            view.add(Element::Text(TextElement {
                x: 4,
                y: 44,
                text: tag_display,
                style: TextStyle::new(Color::from_palette(11)),
            }));

            let gh_url = format!("https://github.com/dmytro-yemelianov/{}", p.slug);
            view.add(Element::Link(LinkElement::new(
                4,
                54,
                format!("[ Repo: github/.../{} ]", p.slug),
                gh_url,
                TextStyle::new(Color::from_palette(7)),
            )));

            view.add(Element::Rect(RectElement {
                x: 4,
                y: 64,
                width: 312,
                height: 1,
                color: Color::from_palette(3),
                filled: true,
            }));

            view.add(Element::Text(TextElement {
                x: 4,
                y: 68,
                text: "ARCHITECTURAL OVERVIEW:".to_string(),
                style: TextStyle::new(Color::from_palette(6)).bold(),
            }));

            let wrapped_lines = word_wrap(p.tldr, 38);
            let mut y = 78;
            for line in wrapped_lines.into_iter().take(3) {
                view.add(Element::Text(TextElement {
                    x: 4,
                    y,
                    text: line,
                    style: TextStyle::new(Color::from_palette(5)),
                }));
                y += 9;
            }

            let diagram: &[&str] = match p.slug {
                "raps" => &[
                    "┌──────────┐      ┌─────────────┐",
                    "│ AI Agent │ ──►  │ RAPS MCP Hub│",
                    "│ (Claude) │ ◄──  │ 51 APIs/JSON│",
                    "└──────────┘      └──────┬──────┘",
                    "   Autodesk APIs ◄───────┘",
                ],
                "dry" => &[
                    "┌──────────┐      ┌─────────────┐",
                    "│Param AST │ ──►  │ Dry Engine  │",
                    "│Kinematics│      │ Units/Motion│",
                    "└──────────┘      └──────┬──────┘",
                    "   CNC / 3D G-Code ◄─────┘",
                ],
                "glueball" => &[
                    "┌──────────┐      ┌─────────────┐",
                    "│Frame AST │ ──►  │Solvers Core │",
                    "│3D Rays   │      │Zero-Kernel  │",
                    "└──────────┘      └──────┬──────┘",
                    "   Assembly Plan ◄───────┘",
                ],
                "klen" => &[
                    "┌──────────┐      ┌─────────────┐",
                    "│Safety In │ ──►  │Lean 4 Kernel│",
                    "│Invariant │      │Formal Proof │",
                    "└──────────┘      └──────┬──────┘",
                    "   Safe Rust Exec ◄──────┘",
                ],
                _ => &[
                    "┌────────────────────────────────────┐",
                    "│ Core Engine : Safe Rust Runtime    │",
                    "│ Target      : WASM / Native CLI    │",
                    "│ Verification: Machine Invariants   │",
                    "└────────────────────────────────────┘",
                ],
            };

            y += 2;
            for line in diagram {
                if y + 8 < 174 {
                    view.add(Element::Text(TextElement {
                        x: 4,
                        y,
                        text: line.to_string(),
                        style: TextStyle::new(Color::from_palette(7)),
                    }));
                    y += 9;
                }
            }

            view.add(Element::Link(LinkElement::new(
                4,
                176,
                "< [ESC] Back to List >",
                "#back",
                TextStyle::new(Color::from_palette(9)).bold(),
            )));
            return;
        }

        let max_visible = 12;
        let start = self.scroll_offset;
        let end = (start + max_visible).min(PROJECTS.len());

        for (i, p) in PROJECTS[start..end].iter().enumerate() {
            let idx = start + i;
            let is_sel = idx == self.selected_project;
            let y = 24 + (i as u16) * 12;

            if is_sel {
                view.add(Element::Rect(RectElement {
                    x: 2,
                    y: y - 1,
                    width: 316,
                    height: 11,
                    color: Color::from_palette(2),
                    filled: true,
                }));
            }

            let marker = if is_sel { ">" } else { " " };
            let fg = if is_sel {
                Color::from_palette(6)
            } else if p.wip {
                Color::from_palette(10)
            } else {
                Color::from_palette(5)
            };

            let num = format!("{:02}", idx + 1);
            let title = if p.title.len() > 14 {
                &p.title[..14]
            } else {
                p.title
            };

            let tag_str = p.tags.iter().map(|t| format!("<{}>", t)).collect::<Vec<_>>().join(" ");
            let tag_display = if is_sel {
                horizontal_scroll(&tag_str, 17, self.tick)
            } else if tag_str.len() > 17 {
                format!("{}...", &tag_str[..14])
            } else {
                tag_str
            };

            view.add(Element::Text(TextElement {
                x: 4,
                y,
                text: format!("{} [{}] {:<14} {}", marker, num, title, tag_display),
                style: TextStyle::new(fg).bold(),
            }));
        }

        if PROJECTS.len() > max_visible {
            view.add(Element::Text(TextElement {
                x: 4,
                y: 176,
                text: format!("{}-{} of {}. [j/k]Nav [Enter]Info", start + 1, end, PROJECTS.len()),
                style: TextStyle::new(Color::from_palette(4)),
            }));
        }
    }

    fn render_projects_32(&self, view: &mut View) {
        if self.show_detail {
            let p = &PROJECTS[self.selected_project];

            view.add(Element::Text(TextElement {
                x: 2,
                y: 24,
                text: format!("PRJ // {}", p.title),
                style: TextStyle::new(Color::from_palette(12)).bold(),
            }));

            let status_badge = if p.wip { "STATUS: WIP / Dev" } else { "STATUS: Stable / Open" };
            view.add(Element::Text(TextElement {
                x: 2,
                y: 33,
                text: status_badge.to_string(),
                style: TextStyle::new(if p.wip { Color::from_palette(10) } else { Color::from_palette(8) }),
            }));

            let tag_str = format!("TAGS: {}", p.tags.join(", "));
            let tag_display = if tag_str.len() > 30 { format!("{}...", &tag_str[..27]) } else { tag_str };
            view.add(Element::Text(TextElement {
                x: 2,
                y: 42,
                text: tag_display,
                style: TextStyle::new(Color::from_palette(11)),
            }));

            let gh_url = format!("https://github.com/dmytro-yemelianov/{}", p.slug);
            view.add(Element::Link(LinkElement::new(
                2,
                51,
                format!("[ Repo: .../{} ]", p.slug),
                gh_url,
                TextStyle::new(Color::from_palette(7)),
            )));

            view.add(Element::Rect(RectElement {
                x: 2,
                y: 60,
                width: 252,
                height: 1,
                color: Color::from_palette(3),
                filled: true,
            }));

            view.add(Element::Text(TextElement {
                x: 2,
                y: 64,
                text: "OVERVIEW:".to_string(),
                style: TextStyle::new(Color::from_palette(6)).bold(),
            }));

            let wrapped_lines = word_wrap(p.tldr, 30);
            let mut y = 73;
            for line in wrapped_lines.into_iter().take(3) {
                view.add(Element::Text(TextElement {
                    x: 2,
                    y,
                    text: line,
                    style: TextStyle::new(Color::from_palette(5)),
                }));
                y += 9;
            }

            y += 2;
            let diagram: &[&str] = match p.slug {
                "raps" => &[
                    "┌──────────┐  ┌──────────────┐",
                    "│ AI Agent │─►│ RAPS MCP Hub │",
                    "│ (Claude) │◄─│ 51 APIs/JSON │",
                    "└──────────┘  └──────┬───────┘",
                    "  Autodesk APIs ◄────┘",
                ],
                "dry" => &[
                    "┌──────────┐  ┌──────────────┐",
                    "│Param AST │─►│ Dry Engine   │",
                    "│Kinematics│  │ Units/Motion │",
                    "└──────────┘  └──────┬───────┘",
                    "  CNC / 3D G-Code ◄──┘",
                ],
                "glueball" => &[
                    "┌──────────┐  ┌──────────────┐",
                    "│Frame AST │─►│ Solvers Core │",
                    "│3D Rays   │  │ Zero-Kernel  │",
                    "└──────────┘  └──────┬───────┘",
                    "  Assembly Plan ◄────┘",
                ],
                "klen" => &[
                    "┌──────────┐  ┌──────────────┐",
                    "│Safety In │─►│ Lean 4 Kernel│",
                    "│Invariant │  │ Formal Proof │",
                    "└──────────┘  └──────┬───────┘",
                    "  Safe Rust Exec ◄───┘",
                ],
                _ => &[
                    "┌────────────────────────────┐",
                    "│ Engine : Safe Rust Runtime │",
                    "│ Target : WASM / Native CLI │",
                    "│ Verif  : Machine Invariants│",
                    "└────────────────────────────┘",
                ],
            };

            for line in diagram {
                if y + 8 < 166 {
                    view.add(Element::Text(TextElement {
                        x: 2,
                        y,
                        text: line.to_string(),
                        style: TextStyle::new(Color::from_palette(7)),
                    }));
                    y += 9;
                }
            }

            view.add(Element::Link(LinkElement::new(
                2,
                168,
                "< [ESC] Back to List >",
                "#back",
                TextStyle::new(Color::from_palette(9)).bold(),
            )));
            return;
        }

        let max_visible = 12;
        let start = self.scroll_offset;
        let end = (start + max_visible).min(PROJECTS.len());

        for (i, p) in PROJECTS[start..end].iter().enumerate() {
            let idx = start + i;
            let is_sel = idx == self.selected_project;
            let y = 24 + (i as u16) * 11;

            if is_sel {
                view.add(Element::Rect(RectElement {
                    x: 1,
                    y: y - 1,
                    width: 254,
                    height: 11,
                    color: Color::from_palette(2),
                    filled: true,
                }));
            }

            let marker = if is_sel { ">" } else { " " };
            let fg = if is_sel {
                Color::from_palette(6)
            } else if p.wip {
                Color::from_palette(10)
            } else {
                Color::from_palette(5)
            };

            let num = format!("{:02}", idx + 1);
            let title = if p.title.len() > 12 {
                &p.title[..12]
            } else {
                p.title
            };

            let tag_str = p.tags.iter().map(|t| format!("<{}>", t)).collect::<Vec<_>>().join(" ");
            let tag_display = if is_sel {
                horizontal_scroll(&tag_str, 12, self.tick)
            } else if tag_str.len() > 12 {
                format!("{}...", &tag_str[..9])
            } else {
                tag_str
            };

            view.add(Element::Text(TextElement {
                x: 2,
                y,
                text: format!("{}{:<2} {:<12} {}", marker, num, title, tag_display),
                style: TextStyle::new(fg).bold(),
            }));
        }

        if PROJECTS.len() > max_visible {
            view.add(Element::Text(TextElement {
                x: 2,
                y: 168,
                text: format!("{}-{} of {}. [j/k]Nav [Enter]", start + 1, end, PROJECTS.len()),
                style: TextStyle::new(Color::from_palette(4)),
            }));
        }
    }

    fn render_resume(&self, view: &mut View) {
        let (cols, _) = self.palette_mode.char_grid();
        match cols {
            80 => self.render_resume_80(view),
            40 => self.render_resume_40(view),
            _ => self.render_resume_32(view),
        }
    }

    fn render_resume_80(&self, view: &mut View) {
        view.add(Element::Text(TextElement {
            x: 16,
            y: 56,
            text: "DMYTRO YEMELIANOV - SENIOR SYSTEMS & AI INFRASTRUCTURE ENGINEER".to_string(),
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));

        // Clickable header links
        view.add(Element::Link(LinkElement::new(
            16,
            74,
            "[ Web: yemelianov.dev ]",
            "https://yemelianov.dev",
            TextStyle::new(Color::from_palette(12)),
        )));
        view.add(Element::Link(LinkElement::new(
            220,
            74,
            "[ GitHub: dmytro-yemelianov ]",
            "https://github.com/dmytro-yemelianov",
            TextStyle::new(Color::from_palette(7)),
        )));
        view.add(Element::Link(LinkElement::new(
            484,
            74,
            "[ LinkedIn ]",
            "https://linkedin.com/in/dmytro-yemelianov",
            TextStyle::new(Color::from_palette(7)),
        )));

        // Top divider
        view.add(Element::Rect(RectElement {
            x: 16,
            y: 90,
            width: 600,
            height: 1,
            color: Color::from_palette(3),
            filled: true,
        }));

        let resume_lines: &[(&str, u8, bool)] = &[
            ("CAREER EXPERIENCE & TECHNICAL IMPACT SUMMARY:", 6, true),
            ("┌────────────┬───────────────────────┬──────────┬────────────────────────┐", 7, false),
            ("│ Period     │ Role & Organization   │ Domain   │ Deliverables & Impact  │", 6, true),
            ("├────────────┼───────────────────────┼──────────┼────────────────────────┤", 7, false),
            ("│ 2023──2026 │ Lead AI Systems Eng.  │ Sitecore │ Azure OpenAI RAG, 5 tms│", 5, false),
            ("│            │ AI Infrastructure     │ GenAI/ML │ CI/CD automated triage │", 4, false),
            ("├────────────┼───────────────────────┼──────────┼────────────────────────┤", 7, false),
            ("│ 2021──2023 │ Principal Architect   │ CAD/BIM  │ WebGL 3D model engine, │", 5, false),
            ("│            │ Computational Geometry│ Rust/C++ │ Multi-tenant workers   │", 4, false),
            ("├────────────┼───────────────────────┼──────────┼────────────────────────┤", 7, false),
            ("│ 2018──2021 │ Senior Systems Eng.   │ Robotics │ Hard real-time loops,  │", 5, false),
            ("│            │ Motion Compilers      │ Embedded │ Deterministic states   │", 4, false),
            ("└────────────┴───────────────────────┴──────────┴────────────────────────┘", 7, false),
            ("", 5, false),
            ("PRODUCTION AI & RAG TRACK RECORD (Sitecore, 2023 - 2026):", 8, true),
            ("• Built & operated Azure OpenAI (GPT-4) RAG expert system over logs & docs.", 5, false),
            ("• Used daily across 5 global teams, cutting triage latency by 60%.", 5, false),
            ("• HIPAA-compliant, zero-data-leakage with automated verification checks.", 5, false),
            ("• Fine-tuned specialized models on domain corpus with strict regression.", 4, false),
            ("", 5, false),
            ("AGENTIC PLATFORMS & OPEN-SOURCE COMPILERS (2025 - 2026):", 11, true),
            ("• Author of RAPS: Rust platform with 51 Model Context Protocol tools.", 5, false),
            ("• Developer of Dry: Typed, units-aware motion compiler IR (CNC / 3D).", 5, false),
            ("• Developer of Glueball: CAD-agnostic routed-assembly solver for frames.", 5, false),
            ("• Developer of Klen: Universal safety contract & invariant kernel (Lean 4).", 5, false),
            ("", 5, false),
            ("CORE COMPETENCIES & SPECIALIZATIONS:", 9, true),
            ("┌───────────────────────────────────┬────────────────────────────────────┐", 3, false),
            ("│ SYSTEMS, COMPILERS & GRAPHICS     │ AI, PROTOCOLS & DISTRIBUTED DATA   │", 6, true),
            ("├───────────────────────────────────┼────────────────────────────────────┤", 3, false),
            ("│ ■ Languages: Rust, C++, C#, Lean 4│ ■ AI: Model Context Protocol (MCP) │", 5, false),
            ("│ ■ Graphics: WebGL2, WebGPU, Metal │ ■ LLM: Azure OpenAI, Prompt Evals  │", 5, false),
            ("│ ■ Compilers: LLVM/MLIR, Domain IRs│ ■ Cloud: Cloudflare Workers, Azure │", 5, false),
            ("│ ■ Kinematics: 3D/CNC Slicing Core │ ■ Data: PostgreSQL, Vector DBs     │", 5, false),
            ("└───────────────────────────────────┴────────────────────────────────────┘", 3, false),
            ("", 5, false),
            ("EDUCATION, FORMAL VERIFICATION & RESEARCH:", 12, true),
            ("• Master of Science in Computer Science & Applied Mathematics (Honors).", 5, false),
            ("• Formal Methods: Lean 4 Interactive Theorem Prover, machine-checked.", 5, false),
            ("• Continuous R&D: Zero-allocation structures, deterministic simulation.", 4, false),
            ("", 5, false),
            ("AVAILABILITY & LOCATION:", 6, true),
            ("• Target Roles: Senior / Staff / Principal Systems, AI Infra & Rust.", 5, false),
            ("• Location: Vinnytsia, Ukraine / Remote worldwide / Open to relocation.", 5, false),
            ("• Direct contact: dyemelianov@icloud.com | GitHub: @dmytro-yemelianov", 7, true),
        ];

        // Vertical scroll window from y = 98 to y = 352
        for (i, &(line_text, pal_idx, bold)) in resume_lines.iter().enumerate() {
            let line_y = 98 + ((i as i32) - (self.resume_scroll as i32)) * 16;
            if line_y >= 98 && line_y <= 348 {
                let mut style = TextStyle::new(Color::from_palette(pal_idx));
                if bold {
                    style = style.bold();
                }
                view.add(Element::Text(TextElement {
                    x: 16,
                    y: line_y as u16,
                    text: line_text.to_string(),
                    style,
                }));
            }
        }

        // Retro scrollbar on the right edge (x: 622..634)
        view.add(Element::Text(TextElement {
            x: 624,
            y: 98,
            text: "▲".to_string(),
            style: TextStyle::new(Color::from_palette(7)).bold(),
        }));

        // Track bar
        view.add(Element::Rect(RectElement {
            x: 627,
            y: 114,
            width: 2,
            height: 220,
            color: Color::from_palette(3),
            filled: true,
        }));

        // Scroll thumb
        let track_h = 220.0 - 18.0;
        let thumb_ratio = self.resume_scroll as f32 / RESUME_MAX_SCROLL as f32;
        let thumb_y = 114 + (thumb_ratio * track_h) as u16;
        view.add(Element::Rect(RectElement {
            x: 624,
            y: thumb_y,
            width: 8,
            height: 18,
            color: Color::from_palette(6),
            filled: true,
        }));

        view.add(Element::Text(TextElement {
            x: 624,
            y: 338,
            text: "▼".to_string(),
            style: TextStyle::new(Color::from_palette(7)).bold(),
        }));

        // Bottom scroll status
        view.add(Element::Text(TextElement {
            x: 16,
            y: 356,
            text: format!(
                "Lines {}-{} of {} | Use [Up/Down], [k/j], [Wheel], or click track to scroll",
                self.resume_scroll + 1,
                (self.resume_scroll + 16).min(resume_lines.len()),
                resume_lines.len()
            ),
            style: TextStyle::new(Color::from_palette(4)),
        }));
    }

    fn render_resume_40(&self, view: &mut View) {
        view.add(Element::Text(TextElement {
            x: 4,
            y: 22,
            text: "DMYTRO YEMELIANOV - AI ARCHITECT".to_string(),
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));

        view.add(Element::Link(LinkElement::new(
            4,
            31,
            "[yemelianov.dev]",
            "https://yemelianov.dev",
            TextStyle::new(Color::from_palette(12)),
        )));
        view.add(Element::Link(LinkElement::new(
            138,
            31,
            "[github]",
            "https://github.com/dmytro-yemelianov",
            TextStyle::new(Color::from_palette(7)),
        )));
        view.add(Element::Link(LinkElement::new(
            212,
            31,
            "[linkedin]",
            "https://linkedin.com/in/dmytro-yemelianov",
            TextStyle::new(Color::from_palette(7)),
        )));

        view.add(Element::Rect(RectElement {
            x: 4,
            y: 40,
            width: 302,
            height: 1,
            color: Color::from_palette(3),
            filled: true,
        }));

        let resume_lines: &[(&str, u8, bool)] = &[
            ("CAREER & IMPACT SUMMARY:", 6, true),
            ("┌─────────────────────────────────────┐", 7, false),
            ("│ 2023─2026 | Sitecore (Lead AI Eng)  │", 6, true),
            ("│ Azure OpenAI RAG across 5 teams.    │", 5, false),
            ("├─────────────────────────────────────┤", 7, false),
            ("│ 2021─2023 | CAD/BIM (Principal Arch)│", 6, true),
            ("│ WebGL 3D model engine & Rust/C++.   │", 5, false),
            ("├─────────────────────────────────────┤", 7, false),
            ("│ 2018─2021 | Robotics (Sr Systems)   │", 6, true),
            ("│ Real-time motion compilers & loops. │", 5, false),
            ("└─────────────────────────────────────┘", 7, false),
            ("", 5, false),
            ("PRODUCTION AI & RAG (Sitecore):", 8, true),
            ("• Built Azure OpenAI RAG expert system.", 5, false),
            ("• Used by 5 teams, -60% triage latency.", 5, false),
            ("• HIPAA-compliant, zero data leakage.", 5, false),
            ("• Fine-tuned specialized domain models.", 4, false),
            ("", 5, false),
            ("AGENTIC PLATFORMS & COMPILERS:", 11, true),
            ("• Author of RAPS: 51 MCP tools in Rust.", 5, false),
            ("• Developer of Dry: CNC/3D motion IR.", 5, false),
            ("• Developer of Glueball: frame solver.", 5, false),
            ("• Developer of Klen: Lean 4 kernel.", 5, false),
            ("", 5, false),
            ("CORE COMPETENCIES:", 9, true),
            ("• Systems: Rust, C++, C#, Lean 4", 5, false),
            ("• Graphics: WebGL2, WebGPU, Metal", 5, false),
            ("• AI/LLM: MCP Protocols, Azure OpenAI", 5, false),
            ("• Cloud: Cloudflare Workers, Azure", 5, false),
            ("• Kinematics: 3D/CNC Slicing & Motion", 5, false),
            ("", 5, false),
            ("EDUCATION & RESEARCH:", 12, true),
            ("• M.Sc. CompSci & Applied Math (Honors)", 5, false),
            ("• Formal verification in Lean 4", 5, false),
            ("", 5, false),
            ("AVAILABILITY & LOCATION:", 6, true),
            ("• Vinnytsia, Ukraine / Remote / Relo", 5, false),
            ("• Contact: dyemelianov@icloud.com", 7, true),
        ];

        for (i, &(line_text, pal_idx, bold)) in resume_lines.iter().enumerate() {
            let line_y = 43 + ((i as i32) - (self.resume_scroll as i32)) * 9;
            if line_y >= 43 && line_y <= 170 {
                let mut style = TextStyle::new(Color::from_palette(pal_idx));
                if bold {
                    style = style.bold();
                }
                view.add(Element::Text(TextElement {
                    x: 4,
                    y: line_y as u16,
                    text: line_text.to_string(),
                    style,
                }));
            }
        }

        // Scrollbar at x = 310
        view.add(Element::Text(TextElement {
            x: 310,
            y: 43,
            text: "▲".to_string(),
            style: TextStyle::new(Color::from_palette(7)).bold(),
        }));

        view.add(Element::Rect(RectElement {
            x: 313,
            y: 53,
            width: 2,
            height: 110,
            color: Color::from_palette(3),
            filled: true,
        }));

        let track_h = 110.0 - 12.0;
        let thumb_ratio = self.resume_scroll as f32 / RESUME_MAX_SCROLL as f32;
        let thumb_y = 53 + (thumb_ratio * track_h) as u16;
        view.add(Element::Rect(RectElement {
            x: 310,
            y: thumb_y,
            width: 8,
            height: 12,
            color: Color::from_palette(6),
            filled: true,
        }));

        view.add(Element::Text(TextElement {
            x: 310,
            y: 165,
            text: "▼".to_string(),
            style: TextStyle::new(Color::from_palette(7)).bold(),
        }));

        view.add(Element::Text(TextElement {
            x: 4,
            y: 177,
            text: format!("Lines {}-{} of {} [j/k/Wheel]", self.resume_scroll + 1, (self.resume_scroll + 14).min(resume_lines.len()), resume_lines.len()),
            style: TextStyle::new(Color::from_palette(4)),
        }));
    }

    fn render_resume_32(&self, view: &mut View) {
        view.add(Element::Text(TextElement {
            x: 2,
            y: 22,
            text: "D.YEMELIANOV - AI ARCHITECT".to_string(),
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));

        view.add(Element::Link(LinkElement::new(
            2,
            31,
            "[yemelianov.dev]",
            "https://yemelianov.dev",
            TextStyle::new(Color::from_palette(12)),
        )));
        view.add(Element::Link(LinkElement::new(
            134,
            31,
            "[github]",
            "https://github.com/dmytro-yemelianov",
            TextStyle::new(Color::from_palette(7)),
        )));

        view.add(Element::Rect(RectElement {
            x: 2,
            y: 40,
            width: 240,
            height: 1,
            color: Color::from_palette(3),
            filled: true,
        }));

        let resume_lines: &[(&str, u8, bool)] = &[
            ("CAREER & IMPACT SUMMARY:", 6, true),
            ("┌───────────────────────────┐", 7, false),
            ("│ 2023─2026 | Sitecore Lead │", 6, true),
            ("│ Azure OpenAI RAG, 5 teams │", 5, false),
            ("├───────────────────────────┤", 7, false),
            ("│ 2021─2023 | CAD/BIM Arch  │", 6, true),
            ("│ WebGL 3D engine, Rust/C++ │", 5, false),
            ("├───────────────────────────┤", 7, false),
            ("│ 2018─2021 | Robotics Sys  │", 6, true),
            ("│ Real-time motion compiler │", 5, false),
            ("└───────────────────────────┘", 7, false),
            ("", 5, false),
            ("PRODUCTION AI (Sitecore):", 8, true),
            ("• Azure OpenAI RAG expert.", 5, false),
            ("• 5 teams, -60% triage time.", 5, false),
            ("• HIPAA-compliant & secure.", 5, false),
            ("", 5, false),
            ("AGENTIC PLATFORMS (2025):", 11, true),
            ("• RAPS: 51 MCP tools in Rust", 5, false),
            ("• Dry: CNC/3D motion IR.", 5, false),
            ("• Glueball: frame solver.", 5, false),
            ("• Klen: Lean 4 verification.", 5, false),
            ("", 5, false),
            ("CORE COMPETENCIES:", 9, true),
            ("• Rust, C++, C#, Lean 4", 5, false),
            ("• WebGL2, WebGPU, Metal", 5, false),
            ("• MCP, Azure OpenAI, RAG", 5, false),
            ("• Cloudflare, Azure, SQL", 5, false),
            ("", 5, false),
            ("EDUCATION & RESEARCH:", 12, true),
            ("• M.Sc. Applied Math (Honor)", 5, false),
            ("• Lean 4 Formal Verification", 5, false),
            ("", 5, false),
            ("CONTACT & AVAILABILITY:", 6, true),
            ("• dyemelianov@icloud.com", 7, true),
            ("• Remote worldwide / Relo", 5, false),
        ];

        for (i, &(line_text, pal_idx, bold)) in resume_lines.iter().enumerate() {
            let line_y = 43 + ((i as i32) - (self.resume_scroll as i32)) * 9;
            if line_y >= 43 && line_y <= 164 {
                let mut style = TextStyle::new(Color::from_palette(pal_idx));
                if bold {
                    style = style.bold();
                }
                view.add(Element::Text(TextElement {
                    x: 2,
                    y: line_y as u16,
                    text: line_text.to_string(),
                    style,
                }));
            }
        }

        // Scrollbar at x = 246
        view.add(Element::Text(TextElement {
            x: 246,
            y: 43,
            text: "▲".to_string(),
            style: TextStyle::new(Color::from_palette(7)).bold(),
        }));

        view.add(Element::Rect(RectElement {
            x: 249,
            y: 53,
            width: 2,
            height: 100,
            color: Color::from_palette(3),
            filled: true,
        }));

        let track_h = 100.0 - 12.0;
        let thumb_ratio = self.resume_scroll as f32 / RESUME_MAX_SCROLL as f32;
        let thumb_y = 53 + (thumb_ratio * track_h) as u16;
        view.add(Element::Rect(RectElement {
            x: 246,
            y: thumb_y,
            width: 8,
            height: 12,
            color: Color::from_palette(6),
            filled: true,
        }));

        view.add(Element::Text(TextElement {
            x: 246,
            y: 155,
            text: "▼".to_string(),
            style: TextStyle::new(Color::from_palette(7)).bold(),
        }));

        view.add(Element::Text(TextElement {
            x: 2,
            y: 170,
            text: format!("Lines {}-{} of {} [k/j]", self.resume_scroll + 1, (self.resume_scroll + 13).min(resume_lines.len()), resume_lines.len()),
            style: TextStyle::new(Color::from_palette(4)),
        }));
    }

    fn render_contact(&self, view: &mut View) {
        let (cols, _) = self.palette_mode.char_grid();
        match cols {
            80 => self.render_contact_80(view),
            40 => self.render_contact_40(view),
            _ => self.render_contact_32(view),
        }
    }

    fn render_contact_80(&self, view: &mut View) {
        view.add(Element::Text(TextElement {
            x: 16,
            y: 58,
            text: "CONNECT & DIRECT CONTACT:".to_string(),
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));

        view.add(Element::Text(TextElement {
            x: 16,
            y: 78,
            text: "Click any underlined link below to open directly in a new tab:".to_string(),
            style: TextStyle::new(Color::from_palette(4)),
        }));

        // Divider
        view.add(Element::Rect(RectElement {
            x: 16,
            y: 96,
            width: 608,
            height: 1,
            color: Color::from_palette(3),
            filled: true,
        }));

        // Clickable contact cards
        let links = [
            ("Email:    dyemelianov@icloud.com", "mailto:dyemelianov@icloud.com", 112),
            ("GitHub:   https://github.com/dmytro-yemelianov", "https://github.com/dmytro-yemelianov", 144),
            ("LinkedIn: https://linkedin.com/in/dmytro-yemelianov", "https://linkedin.com/in/dmytro-yemelianov", 176),
            ("Website:  https://yemelianov.dev", "https://yemelianov.dev", 208),
            ("SSH:      ssh guest@yemelianov.dev (Port 2222)", "ssh://guest@yemelianov.dev:2222", 240),
        ];

        for (label, url, ly) in links {
            view.add(Element::Rect(RectElement {
                x: 16,
                y: ly - 4,
                width: 608,
                height: 24,
                color: Color::from_palette(1),
                filled: true,
            }));
            view.add(Element::Link(LinkElement::new(
                24,
                ly,
                label,
                url,
                TextStyle::new(Color::from_palette(12)).bold(),
            )));
        }

        // Additional information wrapped
        let mut y = 280;
        let info = "Location: Vinnytsia, Ukraine / Remote worldwide\nAvailability: Full-time Senior / Lead / Architect roles in Systems, AI Infrastructure, and Agentic Engineering.";
        for line in word_wrap(info, 74) {
            view.add(Element::Text(TextElement {
                x: 16,
                y,
                text: line,
                style: TextStyle::new(Color::from_palette(5)),
            }));
            y += 18;
        }
    }

    fn render_contact_40(&self, view: &mut View) {
        view.add(Element::Text(TextElement {
            x: 4,
            y: 24,
            text: "CONNECT & DIRECT CONTACT:".to_string(),
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));

        view.add(Element::Text(TextElement {
            x: 4,
            y: 34,
            text: "Click link to open in new tab:".to_string(),
            style: TextStyle::new(Color::from_palette(4)),
        }));

        view.add(Element::Rect(RectElement {
            x: 4,
            y: 43,
            width: 312,
            height: 1,
            color: Color::from_palette(3),
            filled: true,
        }));

        let links = [
            ("Email:   dyemelianov@icloud.com", "mailto:dyemelianov@icloud.com", 47),
            ("GitHub:  github.com/dmytro-yemelianov", "https://github.com/dmytro-yemelianov", 72),
            ("LinkedIn: linkedin.com/in/dmytro-yemelianov", "https://linkedin.com/in/dmytro-yemelianov", 97),
            ("Website: https://yemelianov.dev", "https://yemelianov.dev", 122),
            ("SSH:     ssh -p 2222 guest@dev", "ssh://guest@yemelianov.dev:2222", 147),
        ];

        for (label, url, ly) in links {
            view.add(Element::Rect(RectElement {
                x: 4,
                y: ly,
                width: 312,
                height: 18,
                color: Color::from_palette(1),
                filled: true,
            }));
            view.add(Element::Link(LinkElement::new(
                8,
                ly + 4,
                label,
                url,
                TextStyle::new(Color::from_palette(12)).bold(),
            )));
        }

        view.add(Element::Text(TextElement {
            x: 4,
            y: 173,
            text: "Vinnytsia, UA / Remote worldwide".to_string(),
            style: TextStyle::new(Color::from_palette(5)),
        }));
    }

    fn render_contact_32(&self, view: &mut View) {
        view.add(Element::Text(TextElement {
            x: 2,
            y: 24,
            text: "CONNECT & CONTACT:".to_string(),
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));

        view.add(Element::Text(TextElement {
            x: 2,
            y: 33,
            text: "Click link to open:".to_string(),
            style: TextStyle::new(Color::from_palette(4)),
        }));

        view.add(Element::Rect(RectElement {
            x: 2,
            y: 42,
            width: 252,
            height: 1,
            color: Color::from_palette(3),
            filled: true,
        }));

        let links = [
            ("Email: dyemelianov@icloud.com", "mailto:dyemelianov@icloud.com", 46),
            ("GH:    github/.../yemelianov", "https://github.com/dmytro-yemelianov", 70),
            ("In:    linkedin/.../yemelianov", "https://linkedin.com/in/dmytro-yemelianov", 94),
            ("Web:   yemelianov.dev", "https://yemelianov.dev", 118),
            ("SSH:   ssh -p 2222 guest@dev", "ssh://guest@yemelianov.dev:2222", 142),
        ];

        for (label, url, ly) in links {
            view.add(Element::Rect(RectElement {
                x: 2,
                y: ly,
                width: 252,
                height: 18,
                color: Color::from_palette(1),
                filled: true,
            }));
            view.add(Element::Link(LinkElement::new(
                6,
                ly + 4,
                label,
                url,
                TextStyle::new(Color::from_palette(12)).bold(),
            )));
        }

        view.add(Element::Text(TextElement {
            x: 2,
            y: 168,
            text: "Vinnytsia, UA / Remote / Relo".to_string(),
            style: TextStyle::new(Color::from_palette(5)),
        }));
    }

    fn render_help(&self, view: &mut View) {
        let (cols, _) = self.palette_mode.char_grid();
        match cols {
            80 => self.render_help_80(view),
            40 => self.render_help_40(view),
            _ => self.render_help_32(view),
        }
    }

    fn render_help_80(&self, view: &mut View) {
        view.add(Element::Text(TextElement {
            x: 16,
            y: 58,
            text: "INTERACTION GUIDE & PLATFORM ARCHITECTURE:".to_string(),
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));

        view.add(Element::Rect(RectElement {
            x: 16,
            y: 78,
            width: 608,
            height: 1,
            color: Color::from_palette(3),
            filled: true,
        }));

        let mut y = 92;

        let sections = [
            ("KEYBOARD SHORTCUTS:", vec![
                "[1, 2, 3, 4]       Directly switch between Projects, Resume, Contact, Help",
                "[Tab]              Cycle forward to the next tab",
                "[P]                Cycle Retro Systems (VGA, ZX, C64, Atari, Amber, Green)",
                "[Up/Down] or [k/j] Navigate project list OR vertically scroll Resume tab",
                "[PgUp / PgDn]      Fast scroll Resume content up / down",
                "[Enter]            Open detailed project architecture view",
                "[Esc] or [q]       Close project detail and return to list",
            ]),
            ("MOUSE & TOUCH INTERACTION:", vec![
                "Tabs & Palette:    Click tab headers or [P] button to switch views/palettes",
                "Resume Scrollbar:  Click ▲ / ▼ or drag/click vertical scroll track to scroll",
                "Wheel:             Scroll up and down through Resume lines and projects",
                "Project Rows:      Click project row to select; click again for details",
                "Hyperlinks:        Click underlined link to open repo, email, or profile",
            ]),
            ("TERMINAL SSH ACCESS:", vec![
                "Connect command:   ssh -p 2222 guest@yemelianov.dev (or localhost:2222)",
                "Full ANSI 256:     Box-drawing, retro palettes, and OSC 8 links",
            ]),
        ];

        for (header, lines) in sections {
            view.add(Element::Text(TextElement {
                x: 16,
                y,
                text: header.to_string(),
                style: TextStyle::new(Color::from_palette(7)).bold(),
            }));
            y += 16;
            for line in lines {
                view.add(Element::Text(TextElement {
                    x: 24,
                    y,
                    text: line.to_string(),
                    style: TextStyle::new(Color::from_palette(5)),
                }));
                y += 14;
            }
            y += 8;
        }
    }

    fn render_help_40(&self, view: &mut View) {
        view.add(Element::Text(TextElement {
            x: 4,
            y: 24,
            text: "GUIDE & CONTROLS:".to_string(),
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));

        view.add(Element::Rect(RectElement {
            x: 4,
            y: 33,
            width: 312,
            height: 1,
            color: Color::from_palette(3),
            filled: true,
        }));

        let lines = [
            ("KEYBOARD SHORTCUTS:", 7, true),
            ("[1-4] Tabs  [P] Sys Mode", 5, false),
            ("[k/j] Scroll [Enter] Detail", 5, false),
            ("[Esc/q] Return to list", 5, false),
            ("", 5, false),
            ("MOUSE & TOUCH:", 7, true),
            ("Click tabs, [P]Sys, or rows", 5, false),
            ("Click scrollbar or use wheel", 5, false),
            ("Click links to open directly", 5, false),
            ("", 5, false),
            ("TERMINAL SSH ACCESS:", 7, true),
            ("ssh -p 2222 guest@yemelianov.dev", 6, false),
            ("Direct ANSI terminal stream", 4, false),
        ];

        let mut y = 38;
        for (line_text, pal_idx, bold) in lines {
            if !line_text.is_empty() {
                let mut style = TextStyle::new(Color::from_palette(pal_idx));
                if bold {
                    style = style.bold();
                }
                view.add(Element::Text(TextElement {
                    x: 4,
                    y,
                    text: line_text.to_string(),
                    style,
                }));
            }
            y += 10;
        }
    }

    fn render_help_32(&self, view: &mut View) {
        view.add(Element::Text(TextElement {
            x: 2,
            y: 24,
            text: "CONTROLS & HELP:".to_string(),
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));

        view.add(Element::Rect(RectElement {
            x: 2,
            y: 33,
            width: 252,
            height: 1,
            color: Color::from_palette(3),
            filled: true,
        }));

        let lines = [
            ("KEYBOARD CONTROLS:", 7, true),
            ("1-4: Tabs   P: Sys Mode", 5, false),
            ("k/j: Scroll Enter: Detail", 5, false),
            ("Esc: Return to list", 5, false),
            ("", 5, false),
            ("MOUSE & TOUCH:", 7, true),
            ("Click tabs, P:Sys, or rows", 5, false),
            ("Click scrollbar or use wheel", 5, false),
            ("Click links to open directly", 5, false),
            ("", 5, false),
            ("TERMINAL SSH ACCESS:", 7, true),
            ("ssh -p 2222", 6, false),
            ("guest@yemelianov.dev", 6, false),
        ];

        let mut y = 38;
        for (line_text, pal_idx, bold) in lines {
            if !line_text.is_empty() {
                let mut style = TextStyle::new(Color::from_palette(pal_idx));
                if bold {
                    style = style.bold();
                }
                view.add(Element::Text(TextElement {
                    x: 2,
                    y,
                    text: line_text.to_string(),
                    style,
                }));
            }
            y += 10;
        }
    }
}

pub struct Session {
    pub id: String,
    pub app: App,
}

impl Session {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            app: App::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_systems_and_tabs_render_within_bounds() {
        for mode in PaletteMode::ALL {
            let (w, h) = mode.resolution();
            let (cols, rows) = mode.char_grid();

            assert!(w > 0 && h > 0);
            assert!(cols > 0 && rows > 0);

            let mut app = App::new();
            app.palette_mode = mode;

            for tab in [Tab::Projects, Tab::Resume, Tab::Contact, Tab::Help] {
                app.current_tab = tab;
                app.show_detail = false;

                let view = app.render();
                assert_eq!(view.width, w);
                assert_eq!(view.height, h);
                assert_eq!(view.palette_mode, mode);

                for elem in &view.elements {
                    match elem {
                        Element::Text(t) => {
                            assert!(t.x < w, "Text '{:?}' x={} out of bounds w={}", t.text, t.x, w);
                            assert!(t.y < h, "Text '{:?}' y={} out of bounds h={}", t.text, t.y, h);
                            let text_len = t.text.chars().count() as u16;
                            assert!(
                                t.x + text_len * 8 <= w + 8, // slight margin for right-aligned items
                                "Text '{:?}' (len {}) at x={} overflows w={} for mode {:?}",
                                t.text, text_len, t.x, w, mode
                            );
                        }
                        Element::Rect(r) => {
                            assert!(r.x <= w, "Rect x={} out of bounds w={}", r.x, w);
                            assert!(r.y <= h, "Rect y={} out of bounds h={}", r.y, h);
                        }
                        Element::Link(l) => {
                            assert!(l.x < w, "Link '{:?}' x={} out of bounds w={}", l.text, l.x, w);
                            assert!(l.y < h, "Link '{:?}' y={} out of bounds h={}", l.text, l.y, h);
                        }
                        Element::Sprite(s) => {
                            assert!(s.x < w && s.y < h);
                        }
                    }
                }

                // Also test project detail view
                if tab == Tab::Projects {
                    app.show_detail = true;
                    for p_idx in 0..PROJECTS.len().min(4) {
                        app.selected_project = p_idx;
                        let detail_view = app.render();
                        for elem in &detail_view.elements {
                            if let Element::Text(t) = elem {
                                assert!(t.x < w && t.y < h);
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_system_switching_key_and_click() {
        let mut app = App::new();
        assert_eq!(app.palette_mode, PaletteMode::Vga);
        assert_eq!(app.palette_mode.char_grid(), (80, 25));
        assert_eq!(app.palette_mode.resolution(), (640, 400));

        // Press 'p'
        assert!(app.update(InputEvent::KeyDown(Key::Char('p'))));
        assert_eq!(app.palette_mode, PaletteMode::ZxSpectrum);
        assert_eq!(app.palette_mode.char_grid(), (32, 24));
        assert_eq!(app.palette_mode.resolution(), (256, 192));

        // Press 'p'
        assert!(app.update(InputEvent::KeyDown(Key::Char('p'))));
        assert_eq!(app.palette_mode, PaletteMode::C64);
        assert_eq!(app.palette_mode.char_grid(), (40, 25));
        assert_eq!(app.palette_mode.resolution(), (320, 200));
    }
}
