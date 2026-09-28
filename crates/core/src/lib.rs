use pixel_ssh_view::{Color, Cursor, Element, InputEvent, Key, RectElement, TextElement, TextStyle, View};

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
        wip: true,
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

pub struct App {
    pub current_tab: Tab,
    pub selected_project: usize,
    pub show_detail: bool,
    pub status: String,
    pub scroll_offset: usize,
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
            status: String::from("Ready. [1-4] Tabs | [Up/Down] Select | [Enter] Detail | [Esc] Back"),
            scroll_offset: 0,
        }
    }

    /// Updates the application state with a normalized input event.
    /// Returns `true` if the state changed and a re-render is required (`dirty`).
    pub fn update(&mut self, event: InputEvent) -> bool {
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
                    if self.current_tab == Tab::Projects && !self.show_detail {
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
                    if self.current_tab == Tab::Projects && !self.show_detail {
                        if self.selected_project + 1 < PROJECTS.len() {
                            self.selected_project += 1;
                            if self.selected_project >= self.scroll_offset + 8 {
                                self.scroll_offset = self.selected_project - 7;
                            }
                            return true;
                        }
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
            InputEvent::PointerDown(_) => {
                // Future interactive pointer support
                false
            }
            InputEvent::Resize { .. } => true,
            _ => false,
        }
    }

    /// Renders the platform-independent `View` model.
    pub fn render(&self) -> View {
        // Standard logical resolution: 40 cols x 12 rows (in 8x16 font = 320x192 / 320x200)
        let mut view = View::new(320, 200);

        // Header background (palette index 1 = dark card)
        view.add(Element::Rect(RectElement {
            x: 0,
            y: 0,
            width: 320,
            height: 18,
            color: Color::from_palette(1),
            filled: true,
        }));

        // Title text
        view.add(Element::Text(TextElement {
            x: 4,
            y: 1,
            text: "DMYTRO YEMELIANOV - SYSTEMS & AI".to_string(),
            style: TextStyle::new(Color::from_palette(7)).bold(), // Accent blue
        }));

        // Tab bar
        let tabs = [
            (Tab::Projects, "[1] Projects"),
            (Tab::Resume, "[2] Resume"),
            (Tab::Contact, "[3] Contact"),
            (Tab::Help, "[4] Help"),
        ];

        let mut tab_x = 4;
        for (tab, label) in tabs {
            let active = self.current_tab == tab;
            let style = if active {
                TextStyle::new(Color::from_palette(6))
                    .with_bg(Color::from_palette(7))
                    .bold()
            } else {
                TextStyle::new(Color::from_palette(4))
            };
            view.add(Element::Text(TextElement {
                x: tab_x,
                y: 18,
                text: label.to_string(),
                style,
            }));
            tab_x += (label.len() as u16 + 2) * 8;
        }

        // Horizontal line separator
        view.add(Element::Rect(RectElement {
            x: 0,
            y: 35,
            width: 320,
            height: 1,
            color: Color::from_palette(3), // Border gray
            filled: true,
        }));

        // Content Area (y: 38 to 180)
        match self.current_tab {
            Tab::Projects => self.render_projects(&mut view),
            Tab::Resume => self.render_resume(&mut view),
            Tab::Contact => self.render_contact(&mut view),
            Tab::Help => self.render_help(&mut view),
        }

        // Footer status bar
        view.add(Element::Rect(RectElement {
            x: 0,
            y: 185,
            width: 320,
            height: 15,
            color: Color::from_palette(1),
            filled: true,
        }));

        view.add(Element::Text(TextElement {
            x: 4,
            y: 186,
            text: self.status.clone(),
            style: TextStyle::new(Color::from_palette(8)), // Green prompt
        }));

        view.cursor = Some(Cursor {
            x: 310,
            y: 186,
            visible: true,
        });

        view
    }

    fn render_projects(&self, view: &mut View) {
        if self.show_detail {
            let p = &PROJECTS[self.selected_project];
            view.add(Element::Text(TextElement {
                x: 8,
                y: 40,
                text: format!("PROJECT: {}", p.title),
                style: TextStyle::new(Color::from_palette(6)).bold(),
            }));
            view.add(Element::Text(TextElement {
                x: 8,
                y: 56,
                text: format!("Tags: {}", p.tags.join(", ")),
                style: TextStyle::new(Color::from_palette(7)),
            }));
            view.add(Element::Rect(RectElement {
                x: 8,
                y: 72,
                width: 304,
                height: 1,
                color: Color::from_palette(3),
                filled: true,
            }));

            // Word wrap TL;DR description into 36-char lines
            let words: Vec<&str> = p.tldr.split_whitespace().collect();
            let mut line = String::new();
            let mut y = 78;
            for word in words {
                if line.len() + word.len() + 1 > 36 {
                    view.add(Element::Text(TextElement {
                        x: 8,
                        y,
                        text: line.clone(),
                        style: TextStyle::new(Color::from_palette(5)),
                    }));
                    line.clear();
                    y += 14;
                }
                if !line.is_empty() {
                    line.push(' ');
                }
                line.push_str(word);
            }
            if !line.is_empty() {
                view.add(Element::Text(TextElement {
                    x: 8,
                    y,
                    text: line,
                    style: TextStyle::new(Color::from_palette(5)),
                }));
            }

            view.add(Element::Text(TextElement {
                x: 8,
                y: 165,
                text: "<Press [ESC] to return to list>".to_string(),
                style: TextStyle::new(Color::from_palette(9)),
            }));
            return;
        }

        let max_visible = 8;
        let start = self.scroll_offset;
        let end = (start + max_visible).min(PROJECTS.len());

        for (i, p) in PROJECTS[start..end].iter().enumerate() {
            let idx = start + i;
            let is_sel = idx == self.selected_project;
            let y = 40 + (i as u16) * 17;

            if is_sel {
                view.add(Element::Rect(RectElement {
                    x: 4,
                    y: y - 1,
                    width: 312,
                    height: 16,
                    color: Color::from_palette(2),
                    filled: true,
                }));
            }

            let marker = if is_sel { ">" } else { " " };
            let fg = if is_sel {
                Color::from_palette(6) // White
            } else if p.wip {
                Color::from_palette(10) // Red
            } else {
                Color::from_palette(5) // Light gray
            };

            let num = format!("{:02}", idx + 1);
            let title = if p.title.len() > 18 {
                &p.title[..18]
            } else {
                p.title
            };

            view.add(Element::Text(TextElement {
                x: 8,
                y,
                text: format!("{} [{}] {:<18}", marker, num, title),
                style: TextStyle::new(fg).bold(),
            }));

            // Tags
            let tag_str = p.tags.iter().take(2).map(|t| format!("<{}>", t)).collect::<Vec<_>>().join(" ");
            view.add(Element::Text(TextElement {
                x: 215,
                y,
                text: tag_str,
                style: TextStyle::new(Color::from_palette(4)),
            }));
        }
    }

    fn render_resume(&self, view: &mut View) {
        view.add(Element::Text(TextElement {
            x: 8,
            y: 40,
            text: "PROFILE: Dmytro Yemelianov".to_string(),
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));
        view.add(Element::Text(TextElement {
            x: 8,
            y: 56,
            text: "Role: Systems & Agentic Workflows Engineer".to_string(),
            style: TextStyle::new(Color::from_palette(7)),
        }));
        view.add(Element::Text(TextElement {
            x: 8,
            y: 72,
            text: "Exp: 3+ years Azure OpenAI RAG in production".to_string(),
            style: TextStyle::new(Color::from_palette(5)),
        }));
        view.add(Element::Text(TextElement {
            x: 8,
            y: 88,
            text: "Stack: Rust, Python, TypeScript, Lean 4, C#".to_string(),
            style: TextStyle::new(Color::from_palette(5)),
        }));
        view.add(Element::Text(TextElement {
            x: 8,
            y: 104,
            text: "Core: Compilers, Toolpaths, CAD/PLM, MCP".to_string(),
            style: TextStyle::new(Color::from_palette(5)),
        }));
        view.add(Element::Text(TextElement {
            x: 8,
            y: 124,
            text: "\"I bridge deterministic engineering with".to_string(),
            style: TextStyle::new(Color::from_palette(9)),
        }));
        view.add(Element::Text(TextElement {
            x: 8,
            y: 140,
            text: "generative AI via typed schema contracts\"".to_string(),
            style: TextStyle::new(Color::from_palette(9)),
        }));
    }

    fn render_contact(&self, view: &mut View) {
        view.add(Element::Text(TextElement {
            x: 8,
            y: 40,
            text: "CONTACT & LINKS:".to_string(),
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));
        view.add(Element::Text(TextElement {
            x: 8,
            y: 60,
            text: "Email:    dmytro@yemelianov.dev".to_string(),
            style: TextStyle::new(Color::from_palette(7)),
        }));
        view.add(Element::Text(TextElement {
            x: 8,
            y: 80,
            text: "GitHub:   github.com/dmytro-yemelianov".to_string(),
            style: TextStyle::new(Color::from_palette(7)),
        }));
        view.add(Element::Text(TextElement {
            x: 8,
            y: 100,
            text: "LinkedIn: in/dmytro-yemelianov".to_string(),
            style: TextStyle::new(Color::from_palette(7)),
        }));
        view.add(Element::Text(TextElement {
            x: 8,
            y: 120,
            text: "Web:      https://yemelianov.dev".to_string(),
            style: TextStyle::new(Color::from_palette(7)),
        }));
        view.add(Element::Text(TextElement {
            x: 8,
            y: 140,
            text: "SSH:      ssh guest@yemelianov.dev".to_string(),
            style: TextStyle::new(Color::from_palette(8)).bold(),
        }));
    }

    fn render_help(&self, view: &mut View) {
        view.add(Element::Text(TextElement {
            x: 8,
            y: 40,
            text: "NAVIGATION & SHORTCUTS:".to_string(),
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));
        view.add(Element::Text(TextElement {
            x: 8,
            y: 60,
            text: "[1, 2, 3, 4]  Switch tabs directly".to_string(),
            style: TextStyle::new(Color::from_palette(5)),
        }));
        view.add(Element::Text(TextElement {
            x: 8,
            y: 78,
            text: "[Tab]         Cycle next tab".to_string(),
            style: TextStyle::new(Color::from_palette(5)),
        }));
        view.add(Element::Text(TextElement {
            x: 8,
            y: 96,
            text: "[Up / k]      Previous project".to_string(),
            style: TextStyle::new(Color::from_palette(5)),
        }));
        view.add(Element::Text(TextElement {
            x: 8,
            y: 114,
            text: "[Down / j]    Next project".to_string(),
            style: TextStyle::new(Color::from_palette(5)),
        }));
        view.add(Element::Text(TextElement {
            x: 8,
            y: 132,
            text: "[Enter]       Open/close project detail".to_string(),
            style: TextStyle::new(Color::from_palette(5)),
        }));
        view.add(Element::Text(TextElement {
            x: 8,
            y: 150,
            text: "[Esc / q]     Close detail".to_string(),
            style: TextStyle::new(Color::from_palette(5)),
        }));
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
