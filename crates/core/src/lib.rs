use pixel_ssh_view::{
    word_wrap, Color, Cursor, Element, InputEvent, Key, LinkElement, RectElement, TextElement,
    TextStyle, View,
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
            status: String::from("Ready. [1-4] Tabs | [Click/Select] | [Enter] Detail | [Esc] Back"),
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
                            if self.selected_project >= self.scroll_offset + 12 {
                                self.scroll_offset = self.selected_project - 11;
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
            InputEvent::PointerDown { x, y, .. } => {
                // 1. Tab clicks (y: 24..48)
                if y >= 24 && y <= 48 {
                    if x >= 8 && x < 135 {
                        self.current_tab = Tab::Projects;
                        self.show_detail = false;
                        return true;
                    } else if x >= 135 && x < 255 {
                        self.current_tab = Tab::Resume;
                        self.show_detail = false;
                        return true;
                    } else if x >= 255 && x < 375 {
                        self.current_tab = Tab::Contact;
                        self.show_detail = false;
                        return true;
                    } else if x >= 375 && x < 495 {
                        self.current_tab = Tab::Help;
                        self.show_detail = false;
                        return true;
                    }
                }
                // 2. Project list interaction
                if self.current_tab == Tab::Projects {
                    if self.show_detail {
                        // Click on back link / button (y: 330..365)
                        if y >= 330 && y <= 365 && x < 320 {
                            self.show_detail = false;
                            return true;
                        }
                    } else if y >= 56 && y < 350 {
                        let row_idx = ((y - 56) / 24) as usize;
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
                false
            }
            InputEvent::Resize { .. } => true,
            _ => false,
        }
    }

    /// Renders the platform-independent `View` model at 640x400.
    pub fn render(&self) -> View {
        let mut view = View::new(640, 400);

        // Header background (palette index 1 = dark card)
        view.add(Element::Rect(RectElement {
            x: 0,
            y: 0,
            width: 640,
            height: 24,
            color: Color::from_palette(1),
            filled: true,
        }));

        // Title text
        view.add(Element::Text(TextElement {
            x: 12,
            y: 4,
            text: "DMYTRO YEMELIANOV - SYSTEMS & AI ARCHITECT".to_string(),
            style: TextStyle::new(Color::from_palette(7)).bold(), // Accent blue
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
            width: 640,
            height: 23,
            color: Color::from_palette(0),
            filled: true,
        }));

        // Tab bar buttons
        let tabs = [
            (Tab::Projects, "[1] Projects", 12),
            (Tab::Resume, "[2] Resume", 140),
            (Tab::Contact, "[3] Contact", 260),
            (Tab::Help, "[4] Help", 380),
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

        // Horizontal line separator
        view.add(Element::Rect(RectElement {
            x: 0,
            y: 49,
            width: 640,
            height: 1,
            color: Color::from_palette(3), // Border gray
            filled: true,
        }));

        // Content Area (y: 54 to 370)
        match self.current_tab {
            Tab::Projects => self.render_projects(&mut view),
            Tab::Resume => self.render_resume(&mut view),
            Tab::Contact => self.render_contact(&mut view),
            Tab::Help => self.render_help(&mut view),
        }

        // Footer status bar background
        view.add(Element::Rect(RectElement {
            x: 0,
            y: 376,
            width: 640,
            height: 24,
            color: Color::from_palette(1),
            filled: true,
        }));

        view.add(Element::Text(TextElement {
            x: 12,
            y: 382,
            text: self.status.clone(),
            style: TextStyle::new(Color::from_palette(8)), // Green prompt
        }));

        view.cursor = Some(Cursor {
            x: 620,
            y: 382,
            visible: true,
        });

        view
    }

    fn render_projects(&self, view: &mut View) {
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
            let mut y = 170;
            for line in wrapped_lines {
                view.add(Element::Text(TextElement {
                    x: 16,
                    y,
                    text: line,
                    style: TextStyle::new(Color::from_palette(5)),
                }));
                y += 18;
            }

            // Clickable Return button
            view.add(Element::Link(LinkElement::new(
                16,
                340,
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

            // Tags formatted with full width
            let tag_str = p.tags.iter().map(|t| format!("<{}>", t)).collect::<Vec<_>>().join(" ");
            let tag_display = if tag_str.len() > 34 {
                &tag_str[..34]
            } else {
                &tag_str
            };

            view.add(Element::Text(TextElement {
                x: 260,
                y: y + 2,
                text: tag_display.to_string(),
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

    fn render_resume(&self, view: &mut View) {
        view.add(Element::Text(TextElement {
            x: 16,
            y: 58,
            text: "DMYTRO YEMELIANOV - SENIOR SYSTEMS & AI INFRASTRUCTURE ENGINEER".to_string(),
            style: TextStyle::new(Color::from_palette(6)).bold(),
        }));

        // Clickable header links
        view.add(Element::Link(LinkElement::new(
            16,
            78,
            "[ Web: yemelianov.dev ]",
            "https://yemelianov.dev",
            TextStyle::new(Color::from_palette(12)),
        )));
        view.add(Element::Link(LinkElement::new(
            232,
            78,
            "[ GitHub: dmytro-yemelianov ]",
            "https://github.com/dmytro-yemelianov",
            TextStyle::new(Color::from_palette(7)),
        )));
        view.add(Element::Link(LinkElement::new(
            504,
            78,
            "[ LinkedIn ]",
            "https://linkedin.com/in/dmytro-yemelianov",
            TextStyle::new(Color::from_palette(7)),
        )));

        // Divider
        view.add(Element::Rect(RectElement {
            x: 16,
            y: 96,
            width: 608,
            height: 1,
            color: Color::from_palette(3),
            filled: true,
        }));

        // Section 1: Track Record
        view.add(Element::Text(TextElement {
            x: 16,
            y: 106,
            text: "PRODUCTION AI & RAG TRACK RECORD (2023 - 2026):".to_string(),
            style: TextStyle::new(Color::from_palette(8)).bold(),
        }));

        let r1 = "Built and operated an Azure OpenAI (GPT) RAG expert system over internal documentation and CI/CD build logs at Sitecore, used daily by 5 engineering teams to triage build failures and query compliance documentation. HIPAA-compliant, zero-data-leakage architecture with deterministic verification checks.";
        let mut y = 126;
        for line in word_wrap(r1, 74) {
            view.add(Element::Text(TextElement {
                x: 16,
                y,
                text: line,
                style: TextStyle::new(Color::from_palette(5)),
            }));
            y += 16;
        }

        // Section 2: Agentic Systems & Infrastructure
        y += 8;
        view.add(Element::Text(TextElement {
            x: 16,
            y,
            text: "AGENTIC SYSTEMS, PROTOCOLS & COMPILERS (2026):".to_string(),
            style: TextStyle::new(Color::from_palette(11)).bold(),
        }));
        y += 18;

        let r2 = "Author of RAPS: an open-core Rust platform with 51 Model Context Protocol tools exposing 16 Autodesk APIs. Developer of Dry (typed motion compiler IR for 3D/CNC) and Glueball (CAD routed-assembly solver). Primary environment: Claude Code with bounded tool budgets, deterministic test suites, and typed schema contracts.";
        for line in word_wrap(r2, 74) {
            view.add(Element::Text(TextElement {
                x: 16,
                y,
                text: line,
                style: TextStyle::new(Color::from_palette(5)),
            }));
            y += 16;
        }

        // Section 3: Core Competencies
        y += 8;
        view.add(Element::Text(TextElement {
            x: 16,
            y,
            text: "CORE COMPETENCIES & TECH STACK:".to_string(),
            style: TextStyle::new(Color::from_palette(7)).bold(),
        }));
        y += 18;

        let r3 = "Languages: Rust, Python, TypeScript, Lean 4, C# | Platforms: Cloudflare Workers, PostgreSQL, Azure, WebGL, WebGPU | Focus: Compilers, MCP Tools, Formal Verification, Real-Time Distributed Systems.";
        for line in word_wrap(r3, 74) {
            view.add(Element::Text(TextElement {
                x: 16,
                y,
                text: line,
                style: TextStyle::new(Color::from_palette(4)),
            }));
            y += 16;
        }
    }

    fn render_contact(&self, view: &mut View) {
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

    fn render_help(&self, view: &mut View) {
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
                "[1, 2, 3, 4]       Directly switch between Projects, Resume, Contact, and Help",
                "[Tab]              Cycle forward to the next tab",
                "[Up/Down] or [k/j] Navigate and select project in the list view",
                "[Enter]            Open detailed project architecture view",
                "[Esc] or [q]       Close project detail and return to list",
            ]),
            ("MOUSE & TOUCH INTERACTION:", vec![
                "Tabs & Buttons:    Click any tab header or button to activate it immediately",
                "Project Rows:      Click any project row to select it; click again to view details",
                "Hyperlinks:        Click any underlined link to open repository, email, or profile",
            ]),
            ("TERMINAL SSH ACCESS:", vec![
                "Connect command:   ssh -p 2222 guest@yemelianov.dev (or localhost:2222)",
                "Full ANSI 256:     Supports terminal box-drawing, true color palettes, and OSC 8 links",
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
