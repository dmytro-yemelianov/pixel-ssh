//! In-depth architectural overviews, subsystem breakdowns, technical specifications,
//! and bespoke ASCII dataflow diagrams for all 16 portfolio projects.

use pixel_ssh_view::{horizontal_scroll, word_wrap};
use crate::data::Project;

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
            overview: "An open-core Rust workspace (Barvinca/rapsody) building the raps CLI, microkernel vinca-kernel, and local Model Context Protocol (MCP) server. Exposes 16 Autodesk Platform Services (APS) APIs to AI coding agents via typed JSON-RPC contracts. Solves enterprise CAD cloud provisioning by replacing error-prone manual UI workflows with automated, reproducible CLI and agentic execution.",
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
                "┌──────────────────┐     ┌──────────────────────┐     ┌──────────────────┐",
                "│  AI Agent / LLM  │ ──► │ RAPS MCP Tool Router │ ──► │  Autodesk Cloud  │",
                "│ (Claude / Codex) │ ◄── │ • 51 Typed Contracts │ ◄── │ Data Mgmt / OSS  │",
                "│  JSON-RPC Stdio  │     │ • AES-GCM Auth Vault │     │ Model Derivative │",
                "└──────────────────┘     └──────────────────────┘     └──────────────────┘",
            ],
            diagram_40: &[
                "┌──────────────┐      ┌──────────────┐",
                "│ AI Agent/LLM │ ──►  │ RAPS MCP Hub │",
                "│ (Claude Code)│ ◄──  │ 51 APIs/JSON │",
                "└──────────────┘      └──────┬───────┘",
                "  Autodesk Platform APIs ◄───┘        ",
            ],
            diagram_32: &[
                "┌──────────┐  ┌─────────────┐",
                "│ AI Agent │─►│ RAPS MCP Hub│",
                "│ (Claude) │◄─│ 51 APIs/JSON│",
                "└──────────┘  └──────┬──────┘",
                "  Autodesk Platform ◄┘       ",
            ],
            highlights: &[
                "51 typed Model Context Protocol tools covering 16 Autodesk APIs",
                "AES-GCM hardware-accelerated local secret vault with zero disk leaks",
                "Automated 2-legged and 3-legged OAuth2 token refresh and scope auditing",
                "Official recognition by Autodesk Developer Advocacy team (invited demo 2026)",
            ],
        },
        "dry" => ProjectDetail {
            overview: "A typed, units-aware multi-level intermediate representation (IR) and optimizing compiler engine for physical machine motion (FFF 3D printing, CNC milling, laser cutting, and multi-axis industrial robotics). Acts as the LLVM/MLIR for physical machines, lowering abstract parametric solid toolpaths into continuous-curvature trajectories and verified machine dialects.",
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
                "┌──────────────────┐     ┌──────────────────────┐     ┌──────────────────┐",
                "│  Parametric AST  │ ──► │ Dry Multi-Level IR   │ ──► │ Hardware Outputs │",
                "│ STEP / PyO3 / UI │ ◄── │ • Units verification │ ◄── │ • Klipper/Marlin │",
                "│ Clothoid Curves  │     │ • S-Curve kinematics │     │ • CNC G-Code 5-Ax│",
                "└──────────────────┘     └──────────────────────┘     └──────────────────┘",
            ],
            diagram_40: &[
                "┌──────────────┐      ┌──────────────┐",
                "│Parametric AST│ ──►  │ Dry Engine IR│",
                "│Kinematics/G43│      │ Units/Motion │",
                "└──────────────┘      └──────┬───────┘",
                "  CNC / 3D G-Code Output ◄───┘        ",
            ],
            diagram_32: &[
                "┌──────────┐  ┌─────────────┐",
                "│Param AST │─►│ Dry Engine  │",
                "│Kinematics│  │ Units/Motion│",
                "└──────────┘  └──────┬──────┘",
                "  CNC / 3D G-Code ◄──┘       ",
            ],
            highlights: &[
                "Zero-overhead dimensional unit safety enforced at compile time",
                "Continuous C2 clothoid trajectory planning prevents machine vibration",
                "Unified multi-level IR lowers seamlessly to both 3D printers and CNCs",
                "Deterministic execution pipeline guarantees byte-identical G-Code emission",
            ],
        },
        "glueball" => ProjectDetail {
            overview: "A CAD-agnostic routed-assembly solver for frames, weldments, tube & pipe routing, and wire harnesses. Formulates structural framing as an attributed 3D graph and section catalog problem, compiling deterministically into build plans (plan.json) and manufacturing reports (report.json) without linking any proprietary CAD geometry kernel.",
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
                "┌──────────────────┐     ┌──────────────────────┐     ┌──────────────────┐",
                "│ Parametric Model │ ──► │ Glueball Solver Core │ ──► │ Build Output/BOM │",
                "│ Fusion / FreeCAD │ ◄── │ • Section catalogs   │ ◄── │ • plan.json file │",
                "│ Attributed Graph │     │ • BLAKE3 build plans │     │ • Cut-list report│",
                "└──────────────────┘     └──────────────────────┘     └──────────────────┘",
            ],
            diagram_40: &[
                "┌──────────────┐      ┌──────────────┐",
                "│3D Frame Graph│ ──►  │ Solvers Core │",
                "│Weld & Routes │      │ Zero-Kernel  │",
                "└──────────────┘      └──────┬───────┘",
                "  Deterministic Assembly ◄───┘        ",
            ],
            diagram_32: &[
                "┌──────────┐  ┌─────────────┐",
                "│Frame AST │─►│Solvers Core │",
                "│3D Rays   │  │Zero-Kernel  │",
                "└──────────┘  └──────┬──────┘",
                "  Assembly Plan ◄────┘       ",
            ],
            highlights: &[
                "Replaces heavy CAD kernels with a fast, deterministic graph solver",
                "Emits exact cut lists, joint mitigations, and weldment plans",
                "Byte-for-byte reproducible BLAKE3 artifact hashing on every build",
                "Direct MCP agent integration allows AI tools to design real physical frames",
            ],
        },
        "klen" => ProjectDetail {
            overview: "A universal kernel for describing, executing, analyzing, and formally verifying system safety contracts, rule-governed state machines, and discrete games. Driven by a single canonical typed IR, it enforces dual-engine differential testing between a machine-checked Lean 4 reference evaluator and a high-performance Rust execution engine.",
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
                "┌──────────────────┐     ┌──────────────────────┐     ┌──────────────────┐",
                "│ Codebase / Repos │ ──► │ Klen Analysis Engine │ ──► │ Verified Edits   │",
                "│ Rust / TS / Py   │ ◄── │ • Tree-sitter AST    │ ◄── │ • Semantic patch │",
                "│ File Trees / Git │     │ • Call-graph indexer │     │ • ADR / Linters  │",
                "└──────────────────┘     └──────────────────────┘     └──────────────────┘",
            ],
            diagram_40: &[
                "┌──────────────┐      ┌──────────────┐",
                "│Safety Invar. │ ──►  │Lean 4 Kernel │",
                "│State Machine │      │Formal Proofs │",
                "└──────────────┘      └──────┬───────┘",
                "  Safe Rust Verified Exec ◄──┘        ",
            ],
            diagram_32: &[
                "┌──────────┐  ┌─────────────┐",
                "│Safety In │─►│Lean 4 Kernel│",
                "│Invariant │  │Formal Proof │",
                "└──────────┘  └──────┬──────┘",
                "  Safe Rust Exec ◄───┘       ",
            ],
            highlights: &[
                "Dual-engine architecture eliminates human transcription bugs",
                "Seven complete verification phases proven mechanically in Lean 4",
                "Zero escape hatches: every state machine transition obeys verified rules",
                "Sub-microsecond state evaluation in the native Rust runtime",
            ],
        },
        "ftth-cad-lineage" => ProjectDetail {
            overview: "Four evolutionary generations of fiber-to-the-home (FTTH) telecommunications CAD engineering tools (FiberCAD -> Gluon -> Photon -> Muon). Replaces error-prone LLM drawing authoring with a deterministic pipeline (Gluon) where a formal network ontology (L0 to L5 layer chain) represents ground truth, and LLMs act solely as advisory checkers.",
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
                "┌──────────────────┐     ┌──────────────────────┐     ┌──────────────────┐",
                "│ Raw Telecom DWG  │ ──► │ Lineage Engine Core  │ ──► │ Network GIS / DB │",
                "│ DXF Fiber Layout │ ◄── │ • L0→L5 Invariants   │ ◄── │ • Excel Splicing │",
                "│ Splitter Blocks  │     │ • Ed25519 Signed Log │     │ • AutoCAD Map 3D │",
                "└──────────────────┘     └──────────────────────┘     └──────────────────┘",
            ],
            diagram_40: &[
                "┌──────────────┐      ┌──────────────┐",
                "│GIS / OSM Data│ ──►  │Gluon Ontology│",
                "│Fiber Routing │      │L0->L5 Layers │",
                "└──────────────┘      └──────┬───────┘",
                "  Signed DXF & Excel BOM ◄───┘        ",
            ],
            diagram_32: &[
                "┌──────────┐  ┌─────────────┐",
                "│GIS / OSM │─►│Gluon L0-L5  │",
                "│Network   │  │Signed Chain │",
                "└──────────┘  └──────┬──────┘",
                "  DXF & Excel BOM ◄──┘       ",
            ],
            highlights: &[
                "Eliminated CAD hallucinations by demoting LLMs from authors to advisors",
                "Cryptographic Ed25519 signing ensures tamper-evident audit trails",
                "Automated generation of complete bill of materials and splicing diagrams",
                "Proven across real telecommunication rollouts in municipal fiber networks",
            ],
        },
        "vpa" => ProjectDetail {
            overview: "Verified Portable Assembly (VPA): A formally verified, multi-target, self-hosted assembly programming language and optimizing compiler designed for trust-zero autonomous AI micro-agents, bare-metal microcontrollers, and edge environments. Combines line-oriented syntax with strict static safety verified by 1,903 mechanized theorems in Lean 4.",
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
                "┌──────────────────┐     ┌──────────────────────┐     ┌──────────────────┐",
                "│ VPA Source Lang  │ ──► │ Lean 4 Verified Core │ ──► │ Native Targets   │",
                "│ Line-Oriented IR │ ◄── │ • 1,903 Theorems/0Ax │ ◄── │ • WASM Core (WAT)│",
                "│ Monadic Sums     │     │ • Witness Cert .vpxc │     │ • RV64 / ARM64   │",
                "└──────────────────┘     └──────────────────────┘     └──────────────────┘",
            ],
            diagram_40: &[
                "┌──────────────┐      ┌──────────────┐",
                "│VPA Source IR │ ──►  │Lean 4 Kernel │",
                "│Line Assembly │      │1,903 Theorems│",
                "└──────────────┘      └──────┬───────┘",
                "  Target Emitters (WASM/RV64) ◄┘      ",
            ],
            diagram_32: &[
                "┌──────────┐  ┌─────────────┐",
                "│VPA Source│─►│Lean 4 Proof │",
                "│Line Asm  │  │1,903 Theore.│",
                "└──────────┘  └──────┬──────┘",
                "  WASM / RV64 Output ◄┘      ",
            ],
            highlights: &[
                "1,903 mechanized Lean 4 theorems with 0 sorrys and 0 unverified axioms",
                "Sub-millisecond proof certificate validation on bare-metal microcontrollers",
                "Self-hosted line-oriented assembly compiler targeting 5 distinct architectures",
                "Guaranteed memory bounds safety for autonomous AI agent code execution",
            ],
        },
        "barvinca" => ProjectDetail {
            overview: "The hosted SaaS and edge control plane for the RAPS ecosystem deployed across Cloudflare's global edge network (barvinca.com). Serves 17 specialized tools building on Autodesk Platform Services (APS), providing an enterprise MCP gateway connecting AI coding agents directly to cloud CAD infrastructure.",
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
                "┌──────────────────┐     ┌──────────────────────┐     ┌──────────────────┐",
                "│ Agent Host Orche │ ──► │ Barvinca Edge Router │ ──► │ Autodesk Cloud   │",
                "│ Claude / Codex / │ ◄── │ • Cloudflare D1 & R2 │ ◄── │ • 16 REST APIs   │",
                "│ Cursor Tool Call │     │ • Microkernel Vinca  │     │ • Batch CAD Jobs │",
                "└──────────────────┘     └──────────────────────┘     └──────────────────┘",
            ],
            diagram_40: &[
                "┌──────────────┐      ┌──────────────┐",
                "│AI Code Agents│ ──►  │Barvinca Edge │",
                "│(Cursor/Claude│ ◄──  │Workers / D1  │",
                "└──────────────┘      └──────┬───────┘",
                "  Autodesk Platform Services ◄┘       ",
            ],
            diagram_32: &[
                "┌──────────┐  ┌─────────────┐",
                "│AI Agents │─►│Barvinca Edge│",
                "│Claude/Cod│◄─│Workers / D1 │",
                "└──────────┘  └──────┬──────┘",
                "  Autodesk Cloud APIs ◄┘     ",
            ],
            highlights: &[
                "Single curl install script configures MCP tools across 3 agent frameworks",
                "Sub-50ms global response latency via Cloudflare edge compute network",
                "17 production CAD automation tools unified in one edge monorepo",
                "Zero client-side secrets: all OAuth2 tokens encrypted at the edge",
            ],
        },
        "upyr" => ProjectDetail {
            overview: "A private, native English <-> Ukrainian keyboard layout fixer written in pure Rust for macOS (with preview builds for Linux and Windows). Transparently intercepts mistyped text across keyboard layout boundaries (e.g. ghbdsn -> привіт, руддщ -> hello), handles the non-standard Ukrainian punctuation row, and operates with zero network access.",
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
                "┌──────────────────┐     ┌──────────────────────┐     ┌──────────────────┐",
                "│ Keyboard Events  │ ──► │ Upyr Hotkey Engine   │ ──► │ Corrected Stream │",
                "│ macOS Access API │ ◄── │ • Bi-directional map │ ◄── │ • Punctuation row│",
                "│ Raw Key Codes    │     │ • Term whitelist     │     │ • Hidden restore │",
                "└──────────────────┘     └──────────────────────┘     └──────────────────┘",
            ],
            diagram_40: &[
                "┌──────────────┐      ┌──────────────┐",
                "│Global Keyhook│ ──►  │Upyr Core Rust│",
                "│macOS Access. │      │Layout / Dict │",
                "└──────────────┘      └──────┬───────┘",
                "  Corrected Text & Sound Packs ◄┘     ",
            ],
            diagram_32: &[
                "┌──────────┐  ┌─────────────┐",
                "│Global Key│─►│Upyr Core    │",
                "│Raw Codes │  │Rust Layout  │",
                "└──────────┘  └──────┬──────┘",
                "  Corrected Ukrainian ◄┘     ",
            ],
            highlights: &[
                "100% offline, private, zero telemetry, zero background network calls",
                "Handles full physical punctuation row: [];',./ <-> хїжєґбю.",
                "Transient clipboard concealing prevents leaking passwords to clipboard history",
                "Native sub-5ms latency with algorithmic procedural click sound packs",
            ],
        },
        "dryer" => ProjectDetail {
            overview: "A modular, cross-platform control platform and firmware generator for 3D printers and motion control machines. Core abstraction is a typed, validated Machine Graph v0.1; an 11-phase deterministic resolver turns that graph into locked, explainable firmware and runtime configuration (machine.lock).",
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
                "┌──────────────────┐     ┌──────────────────────┐     ┌──────────────────┐",
                "│ Machine Graph    │ ──► │ 11-Phase Resolver    │ ──► │ Runtime Firmware │",
                "│ Components / MCU │ ◄── │ • Pin allocation     │ ◄── │ • machine.lock   │",
                "│ Electrical Rules │     │ • Safety partitions  │     │ • Flash Images   │",
                "└──────────────────┘     └──────────────────────┘     └──────────────────┘",
            ],
            diagram_40: &[
                "┌──────────────┐      ┌──────────────┐",
                "│Machine Graph │ ──►  │11-Phase Res. │",
                "│Hardware Spec │      │Deterministic │",
                "└──────────────┘      └──────┬───────┘",
                "  Locked Firmware & Safety Map ◄┘     ",
            ],
            diagram_32: &[
                "┌──────────┐  ┌─────────────┐",
                "│Machine G.│─►│11-Phase Res.│",
                "│Hardware  │  │Deterministic│",
                "└──────────┘  └──────┬──────┘",
                "  Locked machine.lock ◄┘     ",
            ],
            highlights: &[
                "11-phase deterministic resolver produces bit-for-bit reproducible machine.lock",
                "Automated thermal runaway safety verification partitions controllers safely",
                "Zero unrouted pin allocations: verifies GPIO, PWM, and timer conflict graphs",
                "Modular cross-platform execution supporting Klipper, Marlin, and STM32 MCUs",
            ],
        },
        "verbacorpus" => ProjectDetail {
            overview: "Canonical, deduplicated, source-attributed digital corpus and linguistic database of Ukrainian proverbs and adages. Unified from digitized historical ethnographic manuscripts, enriched with modern orthography and thematic categories, and served globally via Cloudflare Workers as a multi-format API and offline PWA.",
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
                "┌──────────────────┐     ┌──────────────────────┐     ┌──────────────────┐",
                "│ Historical Texts │ ──► │ Normalization Engine │ ──► │ Corpus Explorer  │",
                "│ Folk Manuscripts │ ◄── │ • Modern orthography │ ◄── │ • /api/v1 (5 fmt)│",
                "│ Ethnographic Rec │     │ • Semantic cluster   │     │ • PWA / Offline  │",
                "└──────────────────┘     └──────────────────────┘     └──────────────────┘",
            ],
            diagram_40: &[
                "┌──────────────┐      ┌──────────────┐",
                "│Ethnographic  │ ──►  │Linguistic NLP│",
                "│Historical MSS│      │Normalization │",
                "└──────────────┘      └──────┬───────┘",
                "  Cloudflare Workers REST / PWA ◄┘    ",
            ],
            diagram_32: &[
                "┌──────────┐  ┌─────────────┐",
                "│Folk Texts│─►│LinguisticNLP│",
                "│1800s MSS │  │Normalization│",
                "└──────────┘  └──────┬──────┘",
                "  REST API & PWA ◄───┘       ",
            ],
            highlights: &[
                "Full content negotiation across 5 data formats (JSON, JSONL, XML, CSV, TSV)",
                "Semantic vector search connects folklore proverbs by underlying concept",
                "Rigorous historical source attribution preserved for every proverb entry",
                "Live production deployment on Cloudflare Workers edge (verbacorpus.org)",
            ],
        },
        "ukrainian-pattern-archive" => ProjectDetail {
            overview: "A computational geometry and computer vision digitization pipeline for traditional Ukrainian folk embroidery charts (vyshyvanka). Extracts discrete grid coordinates, stitch geometries, symbol matrices, and color palettes from historical scanned manuscripts and documents, creating a structured, deduplicated cultural corpus.",
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
                "┌──────────────────┐     ┌──────────────────────┐     ┌──────────────────┐",
                "│ Scanned Patterns │ ──► │ Pattern Vectorizer   │ ──► │ Vector Models    │",
                "│ Historical Books │ ◄── │ • Grid rectification │ ◄── │ • Stitch Lattice │",
                "│ Folk Embroidery  │     │ • Color clustering   │     │ • SVG / JSON/Web │",
                "└──────────────────┘     └──────────────────────┘     └──────────────────┘",
            ],
            diagram_40: &[
                "┌──────────────┐      ┌──────────────┐",
                "│Scanned Charts│ ──►  │Vision Grid CV│",
                "│Historical MSS│      │Dewarp & OCR  │",
                "└──────────────┘      └──────┬───────┘",
                "  Structured SVG & Stitch Lattice ◄┘  ",
            ],
            diagram_32: &[
                "┌──────────┐  ┌─────────────┐",
                "│Scanned   │─►│Vision OCR   │",
                "│Charts    │  │Grid Dewarp  │",
                "└──────────┘  └──────┬──────┘",
                "  Stitch Matrix & SVG ◄┘     ",
            ],
            highlights: &[
                "Reconstructs exact discrete cross-stitch lattices from distorted scans",
                "Automated color quantization into standardized historical thread codes",
                "Symmetry-aware deduplication clusters rotated and mirrored variations",
                "Preserves endangered cultural heritage via open machine-readable formats",
            ],
        },
        "nasinnia" => ProjectDetail {
            overview: "A raster-first typography engine and REST API designed specifically for historical Ukrainian typography, retro computing fonts, and pixel-art bitmap glyph matrices. Never treats vector outlines or font previews as primary data; treats raw 1-bit binary pixel arrays as the immutable ground truth.",
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
                "┌──────────────────┐     ┌──────────────────────┐     ┌──────────────────┐",
                "│ 1-Bit Raw Glyphs │ ──► │ Typography Engine    │ ──► │ Font Service API │",
                "│ Bitmap ROM Fonts │ ◄── │ • Integer scaling    │ ◄── │ • /v1/fonts API  │",
                "│ Historical Matrix│     │ • In-memory PNG enc  │     │ • Web Explorer   │",
                "└──────────────────┘     └──────────────────────┘     └──────────────────┘",
            ],
            diagram_40: &[
                "┌──────────────┐      ┌──────────────┐",
                "│1-Bit ROM Font│ ──►  │Raster Engine │",
                "│Bitmap Matrix │      │Integer Scaler│",
                "└──────────────┘      └──────┬───────┘",
                "  Streaming PNG / Glyph API ◄┘        ",
            ],
            diagram_32: &[
                "┌──────────┐  ┌─────────────┐",
                "│1-Bit ROM │─►│Raster Engine│",
                "│Glyphs    │  │Integer Scale│",
                "└──────────┘  └──────┬──────┘",
                "  PNG & Glyph API ◄──┘       ",
            ],
            highlights: &[
                "Raster-first philosophy guarantees zero vector rasterization distortion",
                "Pure TypeScript streaming PNG encoder running within Workers memory limits",
                "Pixel-accurate historical Ukrainian typography preserved at original resolutions",
                "Sub-10ms response time on Cloudflare Workers edge network",
            ],
        },
        "criticality-lab" => ProjectDetail {
            overview: "A native, high-performance GPU-accelerated lattice physics simulation lab. Implements Bak-Tang-Wiesenfeld Abelian sandpiles, 2D Ising ferromagnetic phase transitions, and Drossel–Schwabl forest-fire percolation models with real-time parameter tuning and critical exponent analysis.",
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
                "┌──────────────────┐     ┌──────────────────────┐     ┌──────────────────┐",
                "│ Simulation Model │ ──► │ wgpu Compute Kernel  │ ──► │ Real-Time Canvas │",
                "│ • Sandpile (SOC) │ ◄── │ • Ping-pong buffers  │ ◄── │ • Metal / 60 FPS │",
                "│ • 2D Ising spins │     │ • Parallel toppling  │     │ • Phase Exponent │",
                "└──────────────────┘     └──────────────────────┘     └──────────────────┘",
            ],
            diagram_40: &[
                "┌──────────────┐      ┌──────────────┐",
                "│Physics Models│ ──►  │wgpu Compute  │",
                "│Sandpile/Ising│      │Metal Pipeline│",
                "└──────────────┘      └──────┬───────┘",
                "  60 FPS Lattice Simulation ◄┘        ",
            ],
            diagram_32: &[
                "┌──────────┐  ┌─────────────┐",
                "│Simulation│─►│wgpu Compute │",
                "│Sandpile  │  │Metal Shaders│",
                "└──────────┘  └──────┬──────┘",
                "  60 FPS 2048x2048 ◄─┘       ",
            ],
            highlights: &[
                "Real-time 2048x2048 cellular automata simulation running at 60 FPS",
                "Direct Apple Metal compute pipeline through wgpu abstraction layer",
                "Accurate numerical validation of self-organized criticality power laws",
                "Zero frame drops and zero CPU stalling via double-buffered GPU ping-pong",
            ],
        },
        "tawara" => ProjectDetail {
            overview: "A lightweight WebAssembly runtime verification harness and memory assertion framework written in pure Rust. Built for running untrusted guest plugins in strict sandboxes, validating linear memory states with BLAKE3 cryptographic hashes, and generating replayable execution proofs.",
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
                "┌──────────────────┐     ┌──────────────────────┐     ┌──────────────────┐",
                "│ Untrusted WASM   │ ──► │ Tawara Sandbox Guard │ ──► │ Verified State   │",
                "│ Guest Plugin Byte│ ◄── │ • Memory bound audit │ ◄── │ • BLAKE3 Hash    │",
                "│ Linear Memory    │     │ • Trap interception  │     │ • Replayable Log │",
                "└──────────────────┘     └──────────────────────┘     └──────────────────┘",
            ],
            diagram_40: &[
                "┌──────────────┐      ┌──────────────┐",
                "│Untrusted WASM│ ──►  │Tawara Sandbox│",
                "│Guest Plugins │      │BLAKE3 State  │",
                "└──────────────┘      └──────┬───────┘",
                "  Replayable Execution Proofs ◄┘      ",
            ],
            diagram_32: &[
                "┌──────────┐  ┌─────────────┐",
                "│WASM Guest│─►│Tawara Guard │",
                "│Plugin    │  │BLAKE3 State │",
                "└──────────┘  └──────┬──────┘",
                "  Verified Execution ◄┘      ",
            ],
            highlights: &[
                "Cryptographically proves guest WebAssembly memory state with BLAKE3",
                "Intercepts untrusted guest panics and traps without taking down host",
                "No-std core library enables verification in embedded and browser sandboxes",
                "Deterministic replay logs allow bit-for-bit forensic debugging of plugin bugs",
            ],
        },
        "media-glitch" => ProjectDetail {
            overview: "A format-aware binary glitch engine and algorithmic databending tool. Directly mutates raw compressed media payloads while safeguarding critical container headers and checksums, ensuring corrupted files produce visual glitch art without failing to decode in standard image viewers.",
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
                "┌──────────────────┐     ┌──────────────────────┐     ┌──────────────────┐",
                "│ Source Image     │ ──► │ Format-Aware Mutator │ ──► │ Synthesized Art  │",
                "│ JPEG / PNG / GIF │ ◄── │ • Header protection  │ ◄── │ • Valid Streams  │",
                "│ PRNG Seed & Rate │     │ • CRC recalculation  │     │ • Canvas Shaders │",
                "└──────────────────┘     └──────────────────────┘     └──────────────────┘",
            ],
            diagram_40: &[
                "┌──────────────┐      ┌──────────────┐",
                "│Source Image  │ ──►  │Glitch Mutator│",
                "│JPEG/PNG/WebP │      │Format-Aware  │",
                "└──────────────┘      └──────┬───────┘",
                "  Valid, Viewable Glitch Art ◄┘       ",
            ],
            diagram_32: &[
                "┌──────────┐  ┌─────────────┐",
                "│Source Img│─►│Glitch Engine│",
                "│PNG / JPEG│  │CRC Rebuilder│",
                "└──────────┘  └──────┬──────┘",
                "  Viewable Glitch Art ◄┘     ",
            ],
            highlights: &[
                "Format-aware parser recalculates PNG CRCs and avoids breaking JPEG headers",
                "100% of glitched images open in standard OS image viewers and web browsers",
                "Deterministic pseudo-random seeds allow exact recreation of visual glitch art",
                "Compiles to WebAssembly for instant in-browser interactive databending",
            ],
        },
        _ => ProjectDetail {
            overview: "A native macOS dynamic simulation engine and cellular automata system built with Swift 5.9 and Apple Metal. Renders tens of thousands of interacting emoji sprites and physical particles at native retina resolutions and 120 FPS display refresh rates with interactive gravity wells and cellular life rules.",
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
                "┌──────────────────┐     ┌──────────────────────┐     ┌──────────────────┐",
                "│ User Interaction │ ──► │ Apple Metal Shaders  │ ──► │ ProMotion Output │",
                "│ Gravity Wells    │ ◄── │ • 100k GPU particles │ ◄── │ • 120 FPS locked │",
                "│ Cellular Rules   │     │ • Spatial grid bins  │     │ • Zero drop frame│",
                "└──────────────────┘     └──────────────────────┘     └──────────────────┘",
            ],
            diagram_40: &[
                "┌──────────────┐      ┌──────────────┐",
                "│User Inputs   │ ──►  │Metal Compute │",
                "│Gravity Wells │      │100k Particles│",
                "└──────────────┘      └──────┬───────┘",
                "  120 FPS Retina ProMotion Display ◄┘ ",
            ],
            diagram_32: &[
                "┌──────────┐  ┌─────────────┐",
                "│User Input│─►│Metal Compute│",
                "│Gravity   │  │100k Particle│",
                "└──────────┘  └──────┬──────┘",
                "  120 FPS Metal Display ◄┘   ",
            ],
            highlights: &[
                "Locked 120 FPS ProMotion performance with up to 100,000 active GPU particles",
                "Custom Metal compute shaders calculate pairwise spatial grid collisions",
                "Zero-copy unified memory eliminates CPU-to-GPU transfer bottlenecks",
                "Native Swift 5.9 implementation with hardened macOS runtime sandbox",
            ],
        },
    }
}

fn truncate_chars(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        Some((idx, _)) => &s[..idx],
        None => s,
    }
}

pub fn project_detail_lines(p: &Project, cols: usize, tick: usize) -> Vec<(String, u8, bool)> {
    let mut lines = Vec::new();
    let detail = get_project_detail(p.slug);

    if cols >= 80 {
        let max_w = 74;

        // 1. Overview Section
        lines.push(("ARCHITECTURAL OVERVIEW & PROBLEM DOMAIN:".to_string(), 6, true));
        for l in word_wrap(detail.overview, max_w) {
            lines.push((l, 5, false));
        }

        lines.push(("".to_string(), 5, false));

        // 2. Subsystems Section
        lines.push(("CORE SUBSYSTEMS & MODULAR CRATES:".to_string(), 11, true));
        for sub in detail.subsystems {
            let bullet = format!("• {}", sub);
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

        // 4. Specifications Table (Exact 74 chars wide)
        lines.push(("TECHNICAL SPECIFICATIONS & INVARIANTS:".to_string(), 9, true));
        lines.push(("┌─────────────────────┬──────────────────────────────────────────────────┐".to_string(), 7, false));
        lines.push(("│ Attribute / Metric  │ Implementation Specification & Guarantees        │".to_string(), 6, true));
        lines.push(("├─────────────────────┼──────────────────────────────────────────────────┤".to_string(), 7, false));
        for (k, v) in detail.specs {
            lines.push((format!("│ {:<19} │ {:<48} │", k, v), 5, false));
        }
        lines.push(("└─────────────────────┴──────────────────────────────────────────────────┘".to_string(), 7, false));

        lines.push(("".to_string(), 5, false));

        // 5. Key Highlights Section
        lines.push(("KEY ENGINEERING HIGHLIGHTS:".to_string(), 14, true));
        for hl in detail.highlights {
            let bullet = format!("• {}", hl);
            for l in word_wrap(&bullet, max_w) {
                lines.push((l, 5, false));
            }
        }

        lines.push(("".to_string(), 5, false));
        lines.push(("< [ESC] or [q] Return to Project List | [h/l] Prev/Next | [k/j] Scroll >".to_string(), 9, true));
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
        lines.push(("──────────────────────────────────────".to_string(), 3, false));

        // 6. Overview
        lines.push(("ARCHITECTURAL OVERVIEW:".to_string(), 6, true));
        for l in word_wrap(detail.overview, max_w) {
            lines.push((l, 5, false));
        }

        lines.push(("".to_string(), 5, false));

        // 7. Subsystems
        lines.push(("SUBSYSTEMS & CRATES:".to_string(), 11, true));
        for sub in detail.subsystems {
            let bullet = format!("• {}", sub);
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
        lines.push(("┌────────────────┬───────────────────┐".to_string(), 7, false));
        lines.push(("│ Metric         │ Specification     │".to_string(), 6, true));
        lines.push(("├────────────────┼───────────────────┤".to_string(), 7, false));
        for (k, v) in detail.specs {
            let k_fmt = truncate_chars(k, 14);
            let v_fmt = truncate_chars(v, 17);
            lines.push((format!("│ {:<14} │ {:<17} │", k_fmt, v_fmt), 5, false));
        }
        lines.push(("└────────────────┴───────────────────┘".to_string(), 7, false));

        lines.push(("".to_string(), 5, false));

        // 10. Key Highlights
        lines.push(("KEY HIGHLIGHTS:".to_string(), 14, true));
        for hl in detail.highlights {
            let bullet = format!("• {}", hl);
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
            let bullet = format!("• {}", sub);
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
            let k_fmt = truncate_chars(k, 10);
            let v_fmt = truncate_chars(v, 12);
            lines.push((format!("│ {:<10} │ {:<12} │", k_fmt, v_fmt), 5, false));
        }
        lines.push(("└────────────┴──────────────┘".to_string(), 7, false));

        lines.push(("".to_string(), 5, false));

        // 10. Key Highlights
        lines.push(("HIGHLIGHTS:".to_string(), 14, true));
        for hl in detail.highlights {
            let bullet = format!("• {}", hl);
            for l in word_wrap(&bullet, max_w) {
                lines.push((l, 5, false));
            }
        }

        lines.push(("".to_string(), 5, false));
        lines.push(("< [ESC] Return to List >".to_string(), 9, true));
    }

    lines
}
