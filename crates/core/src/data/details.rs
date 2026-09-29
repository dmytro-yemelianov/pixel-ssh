//! In-depth architectural overviews, subsystem breakdowns, technical specifications,
//! and bespoke ASCII dataflow diagrams for all 16 portfolio projects.

use crate::data::Project;
use pixel_ssh_view::{horizontal_scroll, word_wrap};

pub struct ProjectDetail {
    pub overview: &'static str,
    pub subsystems: &'static [&'static str],
    pub specs: &'static [(&'static str, &'static str)],
    pub diagram_80: &'static [&'static str],
    pub diagram_40: &'static [&'static str],
    pub diagram_32: &'static [&'static str],
    pub highlights: &'static [&'static str],
}

pub fn get_project_detail(slug: &str) -> ProjectDetail {
    match slug {
        "raps" => ProjectDetail {
            overview: "RAPS combines a Rust CLI, a small HTTP kernel, and an MCP server around Autodesk Platform Services. Its typed tool contracts let agents run CAD cloud operations through the same commands used by people. The local credential vault and repeatable CLI flow address the manual provisioning work that motivated the project.",
            subsystems: &[
                "vinca-kernel: Microkernel HTTP client, rate limiter, credential vault, retry engine",
                "raps-cli: 230+ clap-derived subcommands with structured table and JSON outputs",
                "raps-mcp: 51 typed MCP tools for Claude Code, Codex, and Cursor Agent",
                "saddle-aps-client: Strongly typed 2-legged & 3-legged OAuth2 authentication harness",
                "barvinca-edge: Remote Cloudflare Workers integration gateway and billing proxy",
            ],
                        specs: &[
                ("Language & Core", "Rust 1.88+ (Zero-alloc streaming JSON, safe)"),
                ("Integrated APIs", "16 APS APIs (Data Mgmt, Derivative, ACC, AEC)"),
                ("MCP Tool Catalog", "51 registered tools with strict JSON Schemas"),
                ("Security & Vault", "AES-GCM encrypted token vault, zero disk leaks"),
                ("Throughput Rate", "500 ACC projects built in ~4m 12s (~300x speed)"),
            ],
            diagram_80: &[
                "AI coding agents",
                "      │ MCP calls",
                "      ▼",
                "    RAPS tool router",
                "      ├─► Data Management / OSS",
                "      ├─► Model Derivative",
                "      └─► ACC and other APS APIs",
            ],
            diagram_40: &[
                "Agents ─► RAPS MCP",
                "          ├─► Data Mgmt",
                "          ├─► Derivative",
                "          └─► ACC / APS",
            ],
            diagram_32: &[
                "Agents -> RAPS MCP",
                "          ├─ Data Mgmt",
                "          ├─ Derivative",
                "          └─ ACC / APS",
            ],
            highlights: &[
                "51 typed Model Context Protocol tools covering 16 Autodesk APIs",
                "AES-GCM hardware-accelerated local secret vault with zero disk leaks",
                "Automated 2-legged and 3-legged OAuth2 token refresh and scope auditing",
                "Official recognition by Autodesk Developer Advocacy team (invited demo 2026)",
            ],
        },
        "dry" => ProjectDetail {
            overview: "Dry treats machine motion as a compiler problem. A units-aware IR retains parametric curves while the planner lowers them into jerk-bounded paths for printers, CNC machines, lasers, and robots. Backends emit each machine dialect from the same motion model.",
            subsystems: &[
                "dry-ir: Multi-level typed IR preserving parametric curves & feature semantics",
                "dry-kinematics: Jerk-bounded S-curve acceleration & spatial lookahead planner",
                "dry-optimizer: Continuous G2/C2 arc and clothoid transition synthesizer",
                "dry-emitters: Dialects for Marlin, Klipper, RS-274 G-Code, KUKA KRL, ABB RAPID",
                "dry-py & dry-wasm: PyO3 native bindings and zero-allocation WebAssembly engine",
            ],
                        specs: &[
                ("Language & Core", "Safe Rust core (zero heap alloc in lookahead)"),
                ("Kinematics Model", "6-axis spatial trajectory solver, jerk bounds"),
                ("Verification", "Machine-checked dimensional unit safety (uom)"),
                ("Continuity Proof", "Guaranteed C2 curvature continuity on joins"),
                ("Target Dialects", "3D (Klipper/Marlin), CNC (RS-274), KUKA (KRL)"),
            ],
            diagram_80: &[
                "Parametric geometry + machine intent",
                "                 ▼",
                "          typed, units-aware IR",
                "                 ▼",
                "       jerk-bounded trajectory",
                "          ├─► FFF / CNC / laser",
                "          └─► robot dialects",
            ],
            diagram_40: &[
                "Parametric curves",
                "       ▼",
                "Units-aware IR -> trajectory",
                "       ├─ FFF / CNC",
                "       └─ robot dialects",
            ],
            diagram_32: &[
                "Curves + machine intent",
                "        ▼",
                "Typed IR / unit checks",
                "        ▼",
                "FFF | CNC | robot",
            ],
            highlights: &[
                "Zero-overhead dimensional unit safety enforced at compile time",
                "Continuous C2 clothoid trajectory planning prevents machine vibration",
                "Unified multi-level IR lowers seamlessly to both 3D printers and CNCs",
                "Deterministic execution pipeline guarantees byte-identical G-Code emission",
            ],
        },
        "glueball" => ProjectDetail {
            overview: "Glueball plans routed assemblies from an attributed graph of frames, joints, pipes, and wires. The solver produces build plans and manufacturing reports without loading a proprietary CAD geometry kernel.",
            subsystems: &[
                "glueball-core: Graph model, section catalogs, topological rules, BLAKE3 hasher",
                "glueball-cli: Headless deterministic solver and report verification binary",
                "glueball-py: PyO3 abi3 extension for Autodesk Fusion and FreeCAD host scripts",
                "glueball-wasm: In-browser solver binding with hand-rolled zero-copy C ABI",
                "glueball-mcp: Stdio MCP server exposing structural assembly operations to LLMs",
            ],
                        specs: &[
                ("Geometry Kernel", "0 KB (Pure topological solver; zero B-Rep CAD)"),
                ("Reproducibility", "Canonical BLAKE3 content-hashed plan & report"),
                ("Contract Design", "JSON Schema-first specification precedes solver"),
                ("Host Adapters", "Autodesk Fusion script, FreeCAD OpenCASCADE"),
                ("Verification", "Property-based topological graph invariants"),
            ],
            diagram_80: &[
                "Frames ─────┐",
                "Pipes ──────┼─► attributed assembly graph",
                "Wires ──────┘              │",
                "                           ▼",
                "                    Glueball solver",
                "                     ├─► build plan",
                "                     └─► manufacturing report",
            ],
            diagram_40: &[
                "Frames ─┐",
                "Pipes ──┼─► graph solver",
                "Wires ──┘       ├─ plan",
                "               └─ report",
            ],
            diagram_32: &[
                "Frames ─┐",
                "Pipes ──┼─► graph solver",
                "Wires ──┘      ├─ plan",
                "              └─ report",
            ],
            highlights: &[
                "Replaces heavy CAD kernels with a fast, deterministic graph solver",
                "Emits exact cut lists, joint mitigations, and weldment plans",
                "Byte-for-byte reproducible BLAKE3 artifact hashing on every build",
                "Direct MCP agent integration allows AI tools to design real physical frames",
            ],
        },
        "klen" => ProjectDetail {
            overview: "Klen models safety contracts and state machines in one typed IR. A Lean 4 evaluator defines the reference semantics; a Rust engine executes the same transitions. Differential tests compare the two so implementation drift is visible.",
            subsystems: &[
                "Canonical Typed IR: Common AST and serialized schema for all state transitions",
                "Lean 4 Semantics: Mechanized inductive invariant proofs and reference evaluator",
                "klen-core: Safe Rust evaluator, move generator, state reducer, and state hasher",
                "klen-diff: Automated differential fuzzing engine comparing Lean 4 vs Rust",
                "klen-wasm: WebAssembly compilation target for browser contract evaluation",
            ],
                        specs: &[
                ("Proof System", "Lean 4 theorem prover: 0 sorrys / 0 axioms"),
                ("Invariants", "Inductive safety preservation, exhaustive moves"),
                ("Differential Fuzz", "Millions of random transitions, zero drift"),
                ("Performance", ">500,000 state transitions/sec per core in Rust"),
                ("State Vault", "Deterministic replay traces with checkpoints"),
            ],
            diagram_80: &[
                "                 typed state-machine IR",
                "                  /                 \\",
                "                 ▼                   ▼",
                "       Lean reference             Rust engine",
                "                 \\                   /",
                "                  └─ differential check",
            ],
            diagram_40: &[
                "Typed state-machine IR",
                "      /         \\",
                "   Lean eval    Rust engine",
                "      \\         /",
                "    differential check",
            ],
            diagram_32: &[
                "State-machine IR",
                "    /       \\",
                " Lean eval   Rust engine",
                "    \\       /",
                "  compare transitions",
            ],
            highlights: &[
                "Dual-engine architecture eliminates human transcription bugs",
                "Seven complete verification phases proven mechanically in Lean 4",
                "Zero escape hatches: every state machine transition obeys verified rules",
                "Sub-microsecond state evaluation in the native Rust runtime",
            ],
        },
        "ftth-cad-lineage" => ProjectDetail {
            overview: "This project records four generations of fiber-network CAD tooling: FiberCAD, Gluon, Photon, and Muon. Gluon uses a deterministic network ontology spanning layers L0 through L5 as ground truth. LLMs inspect the result as advisory checkers rather than authoring the drawing.",
            subsystems: &[
                "gluon-core: Formal telco ontology and forward-only L0-L5 layer invariants",
                "gluon-manifest: Ed25519-signed cryptographic manifest chaining all design steps",
                "photon: Headless C# AutoCAD plugin attached as a deterministic DXF renderer",
                "fibercad: Algorithmic GIS/OSM route planner and cable topology solver",
                "muon: Next-generation topological fiber routing and splice cabinet organizer",
            ],
                        specs: &[
                ("Verification", "Forward-only L0→L5 pipeline with crypto signing"),
                ("Audit Trail", "Ed25519-signed artifact manifests for telco"),
                ("Deliverables", "Byte-identical AutoCAD DXF, Excel BOM, HTML log"),
                ("Paradigm", "Deterministic geometry core; zero LLM drift"),
                ("Pipeline Arc", "FiberCAD (algorithmic) -> Gluon -> Photon"),
            ],
            diagram_80: &[
                "Tool generations: FiberCAD -> Gluon -> Photon -> Muon",
                "                         │",
                "                         ▼",
                "              network ontology L0..L5",
                "               ├─► deterministic drawing",
                "               └─► advisory LLM check",
            ],
            diagram_40: &[
                "FiberCAD -> Gluon ->",
                "Photon -> Muon",
                "       │",
                "Ontology L0..L5",
                "  ├─ drawing  └─ LLM check",
            ],
            diagram_32: &[
                "FiberCAD -> Gluon ->",
                "Photon -> Muon",
                "       │",
                "L0..L5 ontology",
                "  ├─ draw  └─ check",
            ],
            highlights: &[
                "Eliminated CAD hallucinations by demoting LLMs from authors to advisors",
                "Cryptographic Ed25519 signing ensures tamper-evident audit trails",
                "Automated generation of complete bill of materials and splicing diagrams",
                "Proven across real telecommunication rollouts in municipal fiber networks",
            ],
        },
        "vpa" => ProjectDetail {
            overview: "VPA is a line-oriented assembly language for small, autonomous programs. Its compiler targets WebAssembly, RV64, and ARM64. Lean 4 proofs and compact witness certificates are used to check the safety properties that matter at those targets.",
            subsystems: &[
                "proof/VPAProof: 109 mechanized Lean 4 modules with 1,903 verified theorems",
                "vpa-compiler: Self-hosted compiler lowering VPA IR to multiple backends",
                "Bounded Replay: .vpaxc sub-millisecond witness verification certificates",
                "Target Backends: WebAssembly (WAT), RISC-V 64, ISO C11, ARM64, ESP32/Xtensa",
                "Runtime ABI: Formally verified scalar bounds and memory safety model",
            ],
                        specs: &[
                ("Formal Proofs", "1,903 mechanized theorems in Lean 4 (0 axioms)"),
                ("Witness Replay", "Microcontrollers verify execution certs < 1 ms"),
                ("Safety Limits", "Zero out-of-bounds memory accesses, provable det"),
                ("Compilation", "Direct bytecode emission to RV64, ARM64, WASM"),
                ("Verification", "Verified against formal operational semantics"),
            ],
            diagram_80: &[
                "VPA source",
                "   ├─► compiler lowering ───────────────► WASM / RV64 / ARM64",
                "   │",
                "   └─► Lean 4 proof obligations ──► witness certificate",
                "                                      │",
                "                                      └─► target check",
            ],
            diagram_40: &[
                "VPA source",
                " ├─ compiler -> WASM/RV/ARM",
                " └─ Lean proof -> witness",
                "                  └─ target check",
            ],
            diagram_32: &[
                "VPA source",
                " ├─ compiler -> targets",
                " └─ Lean -> witness",
                "             └─ verify",
            ],
            highlights: &[
                "1,903 mechanized Lean 4 theorems with 0 sorrys and 0 unverified axioms",
                "Sub-millisecond proof certificate validation on bare-metal microcontrollers",
                "Self-hosted line-oriented assembly compiler targeting 5 distinct architectures",
                "Guaranteed memory bounds safety for autonomous AI agent code execution",
            ],
        },
        "barvinca" => ProjectDetail {
            overview: "Barvinca is the hosted edge control plane for the RAPS ecosystem. It routes agent requests to specialized tools and Autodesk Platform Services through a Cloudflare-based gateway.",
            subsystems: &[
                "Cloudflare Edge Router: Multi-tenant worker architecture for global routing",
                "Universal MCP Hub: Automated installer for Claude Code, Codex, and Cursor",
                "rapsody Edge Binding: High-throughput Rust microkernel execution layer",
                "Edge Data Layer: D1 relational database, R2 binary storage, KV session cache",
                "Async Pipeline: Cloudflare Queues & Workflows orchestrating batch CAD jobs",
            ],
                        specs: &[
                ("Architecture", "Cloudflare Workers Monorepo (TypeScript, D1, R2)"),
                ("Tool Catalog", "17 production tools mounted under /app/<slug>/"),
                ("Protocol", "Model Context Protocol (MCP) Stream & HTTP"),
                ("Security & Auth", "Strict multi-tenant token isolation, zero leaks"),
                ("CAD Backend", "Automated batch processing on Autodesk APS Cloud"),
            ],
            diagram_80: &[
                "Claude / Codex / Cursor tool requests",
                "                   │",
                "                   ▼",
                "            Barvinca edge gateway",
                "             ├─► specialized tools",
                "             ├─► Autodesk APS",
                "             └─► RAPS integration",
            ],
            diagram_40: &[
                "Agent tool requests",
                "        ▼",
                "Barvinca edge gateway",
                " ├─ tools  ├─ APS",
                " └─ RAPS integration",
            ],
            diagram_32: &[
                "Agent requests",
                "      ▼",
                "Barvinca edge",
                " ├─ tools  ├─ APS",
                " └─ RAPS",
            ],
            highlights: &[
                "Single curl install script configures MCP tools across 3 agent frameworks",
                "Sub-50ms global response latency via Cloudflare edge compute network",
                "17 production CAD automation tools unified in one edge monorepo",
                "Zero client-side secrets: all OAuth2 tokens encrypted at the edge",
            ],
        },
        "upyr" => ProjectDetail {
            overview: "Upyr fixes text typed under the wrong English or Ukrainian keyboard layout. It runs locally in Rust, including the Ukrainian punctuation row, and makes no network requests.",
            subsystems: &[
                "upyr-core: Bidirectional layout mapping tables & dictionary n-gram heuristics",
                "upyr-macos: Accessibility event tap hook, CGEvent synthesis, menu-bar tray",
                "upyr-clipboard: Transient clipboard format snapshot, concealing, & restore",
                "Audio Synthesizer: 3 algorithmic local click/feedback packs (no WAV assets)",
                "Configuration: Searchable tabbed preference panel with press-to-record keys",
            ],
                        specs: &[
                ("Language & Stack", "100% Pure Safe Rust (Zero Obj-C dependencies)"),
                ("Privacy Model", "100% offline, zero network sockets, zero metrics"),
                ("Clipboard Safety", "Concealed from third-party clipboard managers"),
                ("Latency Target", "Sub-5ms global key interception & text replace"),
                ("Protected Words", "Protects code keywords, URLs, and terms: FAANG"),
            ],
            diagram_80: &[
                "mistyped key sequence ──► local layout map ──► corrected text",
                "          ▲                                    │",
                "          └──────── restore / edit ─────────┘",
                "No network path: conversion stays on the device.",
            ],
            diagram_40: &[
                "Wrong-layout keys",
                "       │",
                "       ▼",
                "Local map -> corrected text",
                "       └─ restore/edit",
            ],
            diagram_32: &[
                "Wrong-layout text",
                "        ▼",
                "Local key map",
                "        ▼",
                "Corrected text / restore",
            ],
            highlights: &[
                "100% offline, private, zero telemetry, zero background network calls",
                "Handles full physical punctuation row: [];',./ <-> хїжєґбю.",
                "Transient clipboard concealing prevents leaking passwords to clipboard history",
                "Native sub-5ms latency with algorithmic procedural click sound packs",
            ],
        },
        "dryer" => ProjectDetail {
            overview: "Dryer starts with a typed machine graph for 3D printers and motion-control hardware. An eleven-phase resolver checks components and wiring, then emits a locked configuration and firmware artifacts. The lock file makes the chosen machine setup explainable.",
            subsystems: &[
                "dryer-graph: Machine Graph v0.1 schema, AST parser, and source location tracker",
                "dryer-resolver: 11 deterministic phases (dependency closure, pin allocation)",
                "dryer-safety: Thermal runaway and electrical safety-coverage partitioning",
                "dryer-host: Host daemon, workflow execution runner, and controller simulator",
                "dryer-usb: Cross-platform USB/serial discovery and hardware flashing pipeline",
            ],
                        specs: &[
                ("Resolver Phases", "11 distinct deterministic phases, no backtrack"),
                ("Reproducibility", "Bit-for-bit reproducible machine.lock & firmware"),
                ("Safety Coverage", "100% heater-to-thermistor fail-safe coverage"),
                ("Hardware Targets", "Klipper MCU, Marlin firmware, STM32 & RP2040"),
                ("Resource Model", "Constraint satisfaction for MCU hardware GPIO"),
            ],
            diagram_80: &[
                "typed Machine Graph",
                "        │",
                "        ▼",
                "11-phase resolver: components -> pins -> safety rules",
                "        │",
                "        ├─► machine.lock",
                "        └─► firmware configuration",
            ],
            diagram_40: &[
                "Machine Graph",
                "      ▼",
                "11-phase resolver",
                " ├─ machine.lock",
                " └─ firmware config",
            ],
            diagram_32: &[
                "Machine Graph",
                "      ▼",
                "11-phase resolver",
                " ├─ lock file",
                " └─ firmware",
            ],
            highlights: &[
                "11-phase deterministic resolver produces bit-for-bit reproducible machine.lock",
                "Automated thermal runaway safety verification partitions controllers safely",
                "Zero unrouted pin allocations: verifies GPIO, PWM, and timer conflict graphs",
                "Modular cross-platform execution supporting Klipper, Marlin, and STM32 MCUs",
            ],
        },
        "verbacorpus" => ProjectDetail {
            overview: "Verbacorpus turns source-attributed Ukrainian proverbs from historical texts into a searchable corpus. It normalizes spelling, deduplicates entries, and serves the result through an API and offline PWA.",
            subsystems: &[
                "Linguistic Pipeline: Historical text normalization, lemmatization, semantic tags",
                "Edge API (/api/v1): Content-negotiated REST endpoints serving 5 data formats",
                "Vector Search Engine: Semantic similarity matching proverbs by conceptual meaning",
                "Progressive Web App: Offline-first client with instant client-side fuzzy search",
                "Export Engine: Bulk streaming data export (JSONL, XML, CSV, TSV)",
            ],
                        specs: &[
                ("Supported Data", "JSON, JSONL, XML, CSV, TSV via Accept header"),
                ("Edge Deployment", "Cloudflare Workers edge with global D1 database"),
                ("API Standard", "OpenAPI 3.0 specification, contract testing"),
                ("Attribution", "100% provenance tracked to ethnographic prints"),
                ("Offline Capable", "Service Worker cached PWA with local index cache"),
            ],
            diagram_80: &[
                "Historical manuscripts ─┐",
                "Digitized collections ──┼─► normalize ─► deduplicate",
                "Source attribution ─────┘                  │",
                "                                        ├─► API",
                "                                        └─► offline PWA",
            ],
            diagram_40: &[
                "Manuscripts ─┐",
                "Collections ──┼─► normalize",
                "Sources ─────┘      ▼",
                "             deduplicate",
                "             ├─ API └─ PWA",
            ],
            diagram_32: &[
                "Texts ─────┐",
                "Sources ───┼─► normalize",
                "           ▼",
                "       deduplicate",
                "       ├─ API └─ PWA",
            ],
            highlights: &[
                "Full content negotiation across 5 data formats (JSON, JSONL, XML, CSV, TSV)",
                "Semantic vector search connects folklore proverbs by underlying concept",
                "Rigorous historical source attribution preserved for every proverb entry",
                "Live production deployment on Cloudflare Workers edge (verbacorpus.org)",
            ],
        },
        "ukrainian-pattern-archive" => ProjectDetail {
            overview: "The archive extracts stitch grids, symbols, and colors from scanned Ukrainian embroidery charts. OCR and geometric rectification convert the images into structured patterns that can be searched and reused.",
            subsystems: &[
                "upa.core: Pydantic schemas, discrete NumPy lattice matrices, color spaces",
                "upa.ocr: Perspective dewarping, grid line detection, stitch symbol classification",
                "upa.dedup: Structural perceptual hashing & symmetry-aware similarity comparison",
                "upa.cli: Command-line interface built with Click and Hatchling packaging",
                "Web Viewer: Zero-dependency interactive SVG and Canvas stitch pattern inspector",
            ],
                        specs: &[
                ("Computer Vision", "Perspective correction & non-linear warp removal"),
                ("Grid Resolution", "Discrete cross-stitch coordinate lattice units"),
                ("Palette Modeling", "Historical thread clustering (DMC & Anchor sets)"),
                ("Export Formats", "Lossless Vector SVG, JSON stitch matrix, PDF"),
                ("Corpus Archival", "Cataloged historical regional ornament archive"),
            ],
            diagram_80: &[
                "Scanned embroidery charts",
                "           │ OCR + grid rectification",
                "           ▼",
                "symbol matrix + stitch coordinates + color set",
                "           │",
                "           └─► structured pattern corpus",
            ],
            diagram_40: &[
                "Scanned patterns",
                "    ▼ OCR",
                "Rectify stitch grid",
                "    ▼",
                "Symbols / colors -> corpus",
            ],
            diagram_32: &[
                "Pattern scans",
                "    ▼ OCR",
                "Stitch grid",
                "    ▼",
                "Symbols + colors",
            ],
            highlights: &[
                "Reconstructs exact discrete cross-stitch lattices from distorted scans",
                "Automated color quantization into standardized historical thread codes",
                "Symmetry-aware deduplication clusters rotated and mirrored variations",
                "Preserves endangered cultural heritage via open machine-readable formats",
            ],
        },
        "nasinnia" => ProjectDetail {
            overview: "Nasinnia stores bitmap glyphs as raw one-bit matrices. Its rendering and PNG API preserve the source pixels for historical Ukrainian type and retro fonts instead of treating vector outlines as the source of truth.",
            subsystems: &[
                "Font Catalog: 1-bit binary glyph matrix catalog and character code mapping",
                "Rasterizer: Pure TypeScript integer scaling and bitmap composition engine",
                "PNG Streamer: In-memory streaming PNG encoder utilizing node:zlib",
                "Cloudflare Worker: Serverless edge API serving glyph bitmaps and raster previews",
                "Font Explorer: Interactive web UI for inspecting glyph dimensions and baselines",
            ],
                        specs: &[
                ("Deployment", "Cloudflare Workers, D1 catalog & R2 glyph store"),
                ("Source Format", "Pure 1-bit binary raster matrices (no blur)"),
                ("Scaling Engine", "Nearest-neighbor integer scaling (1x to 16x)"),
                ("Endpoints", "/v1/fonts/:id/glyphs, /v1/fonts/:id/preview"),
                ("Orthography", "Full Unicode mapping for Ukrainian Cyrillic ROM"),
            ],
            diagram_80: &[
                "raw 1-bit glyph matrices",
                "         │ integer scaling",
                "         ▼",
                "bitmap renderer ──► PNG API",
                "         └────────► web explorer",
            ],
            diagram_40: &[
                "1-bit glyphs",
                "     ▼",
                "Integer scaling",
                " ├─ PNG API",
                " └─ web explorer",
            ],
            diagram_32: &[
                "1-bit glyphs",
                "      ▼",
                "Pixel renderer",
                " ├─ PNG API",
                " └─ explorer",
            ],
            highlights: &[
                "Raster-first philosophy guarantees zero vector rasterization distortion",
                "Pure TypeScript streaming PNG encoder running within Workers memory limits",
                "Pixel-accurate historical Ukrainian typography preserved at original resolutions",
                "Sub-10ms response time on Cloudflare Workers edge network",
            ],
        },
        "criticality-lab" => ProjectDetail {
            overview: "Criticality Lab runs sandpile, Ising, and forest-fire lattice models on the GPU. Double-buffered state feeds live plots and phase views, while CPU reference loops provide a way to compare results. The simulation is interactive, so parameter changes feed back into the next state.",
            subsystems: &[
                "Simulation Kernels: Parallel GPU compute kernels and optimized CPU reference loops",
                "Rendering Engine: Hardware-accelerated wgpu graphics pipeline (Metal on macOS)",
                "State Buffers: Double-buffered compute storage textures for ping-pong iteration",
                "UI Layer: Real-time critical exponent plotting and phase diagram display",
                "Native Packager: Scripted macOS application bundle generator",
            ],
                        specs: &[
                ("Graphics Engine", "wgpu (Metal macOS, Vulkan Linux, DX12 Windows)"),
                ("Physics Models", "Abelian Sandpile (SOC), 2D Ising, Forest-Fire"),
                ("Grid Resolution", "Up to 2048x2048 cellular automata at 60 FPS"),
                ("Conservation", "Exact energy conservation & spin-flip invariance"),
                ("GPU Memory", "Zero-copy storage buffers, negligible CPU load"),
            ],
            diagram_80: &[
                "Sandpile / Ising / forest-fire state",
                "                ▼",
                "        GPU compute step",
                "           ↕ ping-pong buffers",
                "        next lattice state ─────────┐",
                "                └────► plots / phase view",
                "        ▲───────────────────────────┘ parameter tuning",
            ],
            diagram_40: &[
                "Lattice state",
                "      ▼",
                "GPU compute ↔ buffers",
                "      ▼          │",
                "Plots / phase view",
                "      └─ tune -> state",
            ],
            diagram_32: &[
                "Lattice state",
                "      ▼",
                "GPU ↔ buffers",
                "      ▼",
                "Plot / phase view",
                "  └─ tune -> state",
            ],
            highlights: &[
                "Real-time 2048x2048 cellular automata simulation running at 60 FPS",
                "Direct Apple Metal compute pipeline through wgpu abstraction layer",
                "Accurate numerical validation of self-organized criticality power laws",
                "Zero frame drops and zero CPU stalling via double-buffered GPU ping-pong",
            ],
        },
        "tawara" => ProjectDetail {
            overview: "Tawara executes untrusted WebAssembly guests inside a bounded runtime. It records memory snapshots, traps, and BLAKE3 state commitments so a failed assertion can be replayed and inspected.",
            subsystems: &[
                "tawara-core: Linear memory bounds auditor, serde memory state snapshots",
                "tawara-cli: Headless assertion runner and verification report generator",
                "tawara-web: Browser harness for evaluating WebAssembly sandbox contracts",
                "Cryptographic Hasher: BLAKE3 state commitments for memory page modifications",
                "Trap Interceptor: Deterministic execution trap capture and failure diagnostics",
            ],
                        specs: &[
                ("Core Language", "Rust 2021 (no-std compatible core with alloc)"),
                ("Verification", "BLAKE3 state hashing before & after guest calls"),
                ("Memory Model", "Strict W3C WebAssembly linear memory bounds"),
                ("Determinism", "Deterministic replay traces with identical seeds"),
                ("Runtime Overhead", "Zero-instrumentation direct memory inspection"),
            ],
            diagram_80: &[
                "┌──────────── bounded WASM sandbox ────────────┐",
                "│ guest call -> linear-memory checks -> trap log │",
                "│       │                              │         │",
                "│       └─► before/after BLAKE3 snapshots       │",
                "└──────────────────┬────────────────────────────┘",
                "                   └─► replayable report",
            ],
            diagram_40: &[
                "┌─ bounded WASM sandbox ─┐",
                "│ guest -> memory checks │",
                "│    └─ BLAKE3 snapshots  │",
                "└──────────┬─────────────┘",
                "       replay report",
            ],
            diagram_32: &[
                "┌─ WASM sandbox ─┐",
                "│ guest / memory │",
                "│ BLAKE3 states  │",
                "└──────┬─────────┘",
                "    replay report",
            ],
            highlights: &[
                "Cryptographically proves guest WebAssembly memory state with BLAKE3",
                "Intercepts untrusted guest panics and traps without taking down host",
                "No-std core library enables verification in embedded and browser sandboxes",
                "Deterministic replay logs allow bit-for-bit forensic debugging of plugin bugs",
            ],
        },
        "media-glitch" => ProjectDetail {
            overview: "Media Glitch mutates compressed image payloads from a repeatable seed. Format-aware guards preserve required headers and checksums so the result remains decodable.",
            subsystems: &[
                "crates/media_glitch_wasm: Browser-based glitch engine compiled to WebAssembly",
                "src/media_glitch: Python CLI prototype with batch processing and backup options",
                "JPEG Engine: Scans EXIF/JFIF and SOS markers; mutates entropy scans safely",
                "PNG Engine: Deflates IDAT scanlines, mutates raw bytes, recalculates chunk CRCs",
                "Seeded PRNG: Deterministic mutation generators for repeatable glitch aesthetics",
            ],
                        specs: &[
                ("Dual Core", "High-performance Rust/WASM library + Python CLI"),
                ("Supported Media", "JPEG (SOS-aware), PNG (IDAT CRC rebuild), GIF"),
                ("Safety Protocol", "Non-destructive by default; automated .bak files"),
                ("Mutation Control", "Bit-level mutation masks, byte substitutions"),
                ("Browser Preview", "Zero-latency real-time preview in HTML5 Canvas"),
            ],
            diagram_80: &[
                "image payload + deterministic seed",
                "                 ▼",
                "       format-aware guard",
                "        ├─ protect headers",
                "        └─ recalculate checksums",
                "                 ▼",
                "         decodable glitch art",
            ],
            diagram_40: &[
                "Image + seed",
                "      ▼",
                "Format guard",
                " ├─ headers  └─ CRC",
                "      ▼",
                "Decodable glitch art",
            ],
            diagram_32: &[
                "Image + seed",
                "      ▼",
                "Header/CRC guard",
                "      ▼",
                "Glitch payload",
                "      ▼",
                "Decodable image",
            ],
            highlights: &[
                "Format-aware parser recalculates PNG CRCs and avoids breaking JPEG headers",
                "100% of glitched images open in standard OS image viewers and web browsers",
                "Deterministic pseudo-random seeds allow exact recreation of visual glitch art",
                "Compiles to WebAssembly for instant in-browser interactive databending",
            ],
        },
        "emoji-madness" => ProjectDetail {
            overview: "Emoji Madness combines cellular rules, interactive force fields, and Metal particle rendering in a native macOS app. User input changes the force field; the resulting motion is fed into the next frame.",
            subsystems: &[
                "Metal Render Pipeline: Compute shaders for particle physics and grid collisions",
                "Swift App Core: Native AppKit menu-bar and fullscreen window orchestration",
                "Cellular Automata: Grid-based life rules driving emoji population dynamics",
                "Force Fields: Interactive gravity wells, vortex attractors, and cursor deflection",
                "Display Sync: CADisplayLink and CVDisplayLink integration for jitter-free 120 Hz",
            ],
                        specs: &[
                ("Language & APIs", "Swift 5.9 + Metal Shading Language compute"),
                ("Target OS", "macOS 14.0+ (Apple Silicon & Intel universal)"),
                ("Particle Budget", "Up to 100,000 active particles with collision"),
                ("Frame Cadence", "Locked 120 FPS on Apple ProMotion; zero drop"),
                ("Memory Arch", "Unified memory arch with direct Metal buffers"),
            ],
            diagram_80: &[
                "cellular rules ─┐",
                "user force field ┼─► Metal particle step ──► display frame",
                "sprite state ─┘             ▲                   │",
                "                            └──── next state ───┘",
            ],
            diagram_40: &[
                "Rules ─┐",
                "Input ──┼─► Metal particles",
                "State ─┘        │",
                "    ▲      display frame",
                "    └──── next state",
            ],
            diagram_32: &[
                "Rules ─┐",
                "Input ──┼─► Metal step",
                "State ─┘      │",
                "  ▲       frame",
                "  └─ next state",
            ],
            highlights: &[
                "Locked 120 FPS ProMotion performance with up to 100,000 active GPU particles",
                "Custom Metal compute shaders calculate pairwise spatial grid collisions",
                "Zero-copy unified memory eliminates CPU-to-GPU transfer bottlenecks",
                "Native Swift 5.9 implementation with hardened macOS runtime sandbox",
            ],
        },
        unknown => panic!("No project detail for slug: {unknown}"),
    }
}

fn truncate_chars(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        Some((idx, _)) => &s[..idx],
        None => s,
    }
}

fn wrap_table_cell(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();

    for word in text.split_whitespace() {
        let chars: Vec<char> = word.chars().collect();
        let mut offset = 0;
        while offset < chars.len() {
            if !current.is_empty() {
                if current.chars().count() + 1 + chars.len() - offset <= width {
                    current.push(' ');
                    current.extend(&chars[offset..]);
                    break;
                }
                lines.push(std::mem::take(&mut current));
            }

            let end = (offset + width).min(chars.len());
            current.extend(&chars[offset..end]);
            offset = end;
            if offset < chars.len() {
                lines.push(std::mem::take(&mut current));
            }
        }
    }

    if !current.is_empty() {
        lines.push(current);
    }
    lines
}

fn compact_spec_rows(key: &str, value: &str, key_width: usize, value_width: usize) -> Vec<String> {
    let key_lines = wrap_table_cell(key, key_width);
    let value_lines = wrap_table_cell(value, value_width);
    (0..key_lines.len().max(value_lines.len()))
        .map(|row| {
            let key = key_lines.get(row).map(String::as_str).unwrap_or("");
            let value = value_lines.get(row).map(String::as_str).unwrap_or("");
            format!("│ {key:<key_width$} │ {value:<value_width$} │")
        })
        .collect()
}

pub fn project_detail_lines(p: &Project, cols: usize, tick: usize) -> Vec<(String, u8, bool)> {
    let mut lines = Vec::new();
    let detail = get_project_detail(p.slug);

    if cols >= 80 {
        let max_w = cols.saturating_sub(6);

        // 1. Overview Section
        lines.push((
            "ARCHITECTURAL OVERVIEW & PROBLEM DOMAIN:".to_string(),
            6,
            true,
        ));
        for l in word_wrap(detail.overview, max_w) {
            lines.push((l, 5, false));
        }

        lines.push(("".to_string(), 5, false));

        // 2. Subsystems Section
        lines.push(("CORE SUBSYSTEMS & MODULAR CRATES:".to_string(), 11, true));
        for sub in detail.subsystems {
            let bullet = format!("• {sub}");
            for l in word_wrap(&bullet, max_w) {
                lines.push((l, 5, false));
            }
        }

        lines.push(("".to_string(), 5, false));

        // 3. Dataflow Diagram Section
        lines.push(("DATAFLOW & STRUCTURAL PIPELINE:".to_string(), 7, true));
        for d in detail.diagram_80 {
            lines.push((d.to_string(), 7, false));
        }

        lines.push(("".to_string(), 5, false));

        // 4. Specifications Table fills the available content width.
        let right_col = max_w.saturating_sub(24);
        let rule = |left: char, middle: char, right: char| {
            format!(
                "{left}{}{middle}{}{right}",
                "─".repeat(21),
                "─".repeat(right_col)
            )
        };
        lines.push((
            "TECHNICAL SPECIFICATIONS & INVARIANTS:".to_string(),
            9,
            true,
        ));
        lines.push((rule('┌', '┬', '┐'), 7, false));
        lines.push((
            format!(
                "│ {:<19} │ {:<width$} │",
                "Attribute / Metric",
                "Implementation Specification & Guarantees",
                width = right_col - 2
            ),
            6,
            true,
        ));
        lines.push((rule('├', '┼', '┤'), 7, false));
        for (k, v) in detail.specs {
            lines.push((
                format!("│ {k:<19} │ {v:<width$} │", width = right_col - 2),
                5,
                false,
            ));
        }
        lines.push((rule('└', '┴', '┘'), 7, false));

        lines.push(("".to_string(), 5, false));

        // 5. Key Highlights Section
        lines.push(("KEY ENGINEERING HIGHLIGHTS:".to_string(), 14, true));
        for hl in detail.highlights {
            let bullet = format!("• {hl}");
            for l in word_wrap(&bullet, max_w) {
                lines.push((l, 5, false));
            }
        }

        lines.push(("".to_string(), 5, false));
        lines.push((
            "< [ESC] or [q] Return to Project List | [h/l] Prev/Next | [k/j] Scroll >".to_string(),
            9,
            true,
        ));
    } else if cols == 40 {
        let max_w = 38;

        // 1. Scrolled Title
        let full_title = format!("PROJECT // {}", p.title);
        lines.push((horizontal_scroll(&full_title, max_w, tick), 12, true));

        // 2. Status
        let status_text = if p.wip {
            "STATUS: In Active Development"
        } else {
            "STATUS: Production-Ready / Stable Open"
        };
        lines.push((status_text.to_string(), if p.wip { 10 } else { 8 }, false));

        // 3. Tags
        let tag_header = format!("TAGS: {}", p.tags.join(", "));
        for l in word_wrap(&tag_header, max_w) {
            lines.push((l, 11, false));
        }

        // 4. Repo
        let repo_line = {
            let full_gh = format!("[ Repo: github.com/.../{} ]", p.slug);
            if full_gh.chars().count() <= max_w {
                full_gh
            } else {
                let full = format!("[ Repo: .../{} ]", p.slug);
                if full.chars().count() <= max_w {
                    full
                } else {
                    let max_slug = max_w.saturating_sub(15);
                    format!("[ Repo: .../{}~ ]", truncate_chars(p.slug, max_slug))
                }
            }
        };
        lines.push((repo_line, 7, false));

        // 5. Divider
        lines.push((
            "──────────────────────────────────────".to_string(),
            3,
            false,
        ));

        // 6. Overview
        lines.push(("ARCHITECTURAL OVERVIEW:".to_string(), 6, true));
        for l in word_wrap(detail.overview, max_w) {
            lines.push((l, 5, false));
        }

        lines.push(("".to_string(), 5, false));

        // 7. Subsystems
        lines.push(("SUBSYSTEMS & CRATES:".to_string(), 11, true));
        for sub in detail.subsystems {
            let bullet = format!("• {sub}");
            for l in word_wrap(&bullet, max_w) {
                lines.push((l, 5, false));
            }
        }

        lines.push(("".to_string(), 5, false));

        // 8. Dataflow Diagram
        lines.push(("DATAFLOW PIPELINE:".to_string(), 7, true));
        for d in detail.diagram_40 {
            lines.push((d.to_string(), 7, false));
        }

        lines.push(("".to_string(), 5, false));

        // 9. Specifications Table (Exact 38 chars wide)
        lines.push(("CORE CONTRACTS & SPECS:".to_string(), 9, true));
        lines.push((
            "┌────────────────┬───────────────────┐".to_string(),
            7,
            false,
        ));
        lines.push((
            "│ Metric         │ Specification     │".to_string(),
            6,
            true,
        ));
        lines.push((
            "├────────────────┼───────────────────┤".to_string(),
            7,
            false,
        ));
        for (k, v) in detail.specs {
            for row in compact_spec_rows(k, v, 14, 17) {
                lines.push((row, 5, false));
            }
        }
        lines.push((
            "└────────────────┴───────────────────┘".to_string(),
            7,
            false,
        ));

        lines.push(("".to_string(), 5, false));

        // 10. Key Highlights
        lines.push(("KEY HIGHLIGHTS:".to_string(), 14, true));
        for hl in detail.highlights {
            let bullet = format!("• {hl}");
            for l in word_wrap(&bullet, max_w) {
                lines.push((l, 5, false));
            }
        }

        lines.push(("".to_string(), 5, false));
        lines.push(("< [ESC] or [q] Return to List >".to_string(), 9, true));
    } else {
        // 32-col (ZX Spectrum: max 29 chars)
        let max_w = 29;

        // 1. Scrolled Title
        let full_title = format!("PRJ // {}", p.title);
        lines.push((horizontal_scroll(&full_title, max_w, tick), 12, true));

        // 2. Status
        let status_text = if p.wip {
            "STATUS: WIP / In Dev"
        } else {
            "STATUS: Production / Stable"
        };
        lines.push((status_text.to_string(), if p.wip { 10 } else { 8 }, false));

        // 3. Tags
        let tag_header = format!("TAGS: {}", p.tags.join(", "));
        for l in word_wrap(&tag_header, max_w) {
            lines.push((l, 11, false));
        }

        // 4. Repo
        let repo_line = {
            let full = format!("[ Repo: .../{} ]", p.slug);
            if full.chars().count() > max_w {
                let max_slug = max_w.saturating_sub(15);
                format!("[ Repo: .../{}~ ]", truncate_chars(p.slug, max_slug))
            } else {
                full
            }
        };
        lines.push((repo_line, 7, false));

        // 5. Divider
        lines.push(("─────────────────────────────".to_string(), 3, false));

        // 6. Overview
        lines.push(("OVERVIEW:".to_string(), 6, true));
        for l in word_wrap(detail.overview, max_w) {
            lines.push((l, 5, false));
        }

        lines.push(("".to_string(), 5, false));

        // 7. Subsystems
        lines.push(("SUBSYSTEMS:".to_string(), 11, true));
        for sub in detail.subsystems {
            let bullet = format!("• {sub}");
            for l in word_wrap(&bullet, max_w) {
                lines.push((l, 5, false));
            }
        }

        lines.push(("".to_string(), 5, false));

        // 8. Dataflow Diagram
        lines.push(("DATAFLOW:".to_string(), 7, true));
        for d in detail.diagram_32 {
            lines.push((d.to_string(), 7, false));
        }

        lines.push(("".to_string(), 5, false));

        // 9. Specifications Table (Exact 29 chars wide)
        lines.push(("SPECS & CONTRACTS:".to_string(), 9, true));
        lines.push(("┌────────────┬──────────────┐".to_string(), 7, false));
        lines.push(("│ Metric     │ Specification│".to_string(), 6, true));
        lines.push(("├────────────┼──────────────┤".to_string(), 7, false));
        for (k, v) in detail.specs {
            for row in compact_spec_rows(k, v, 10, 12) {
                lines.push((row, 5, false));
            }
        }
        lines.push(("└────────────┴──────────────┘".to_string(), 7, false));

        lines.push(("".to_string(), 5, false));

        // 10. Key Highlights
        lines.push(("HIGHLIGHTS:".to_string(), 14, true));
        for hl in detail.highlights {
            let bullet = format!("• {hl}");
            for l in word_wrap(&bullet, max_w) {
                lines.push((l, 5, false));
            }
        }

        lines.push(("".to_string(), 5, false));
        lines.push(("< [ESC] Return to List >".to_string(), 9, true));
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::PROJECTS;

    #[test]
    fn compact_spec_cells_keep_all_text_at_both_widths() {
        for project in PROJECTS {
            for (key, value) in get_project_detail(project.slug).specs {
                for (key_width, value_width) in [(14, 17), (10, 12)] {
                    let rows = compact_spec_rows(key, value, key_width, value_width);
                    assert!(rows
                        .iter()
                        .all(|row| row.chars().count() == key_width + value_width + 7));
                    let rendered_key: String = rows
                        .iter()
                        .flat_map(|row| row.chars().skip(2).take(key_width))
                        .filter(|ch| !ch.is_whitespace())
                        .collect();
                    let rendered_value: String = rows
                        .iter()
                        .flat_map(|row| row.chars().skip(key_width + 5).take(value_width))
                        .filter(|ch| !ch.is_whitespace())
                        .collect();
                    assert_eq!(rendered_key, key.split_whitespace().collect::<String>());
                    assert_eq!(rendered_value, value.split_whitespace().collect::<String>());
                }
            }
        }
    }
}
