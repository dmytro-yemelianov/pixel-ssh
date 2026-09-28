# Pixel-First Rust Web + SSH Interface

A minimal interactive application exposing the **exact same application and session model** through two distinct interfaces:
1. **Web Interface**: Lightweight pixel-based UI written in Rust/WASM, rendering to a WebGL2 8-bit indexed framebuffer (320 × 200) with nearest-neighbor scaling, an authentic IBM VGA 8×8 bitmap font, and zero DOM UI widgets.
2. **SSH Interface**: Native Rust SSH server (powered by `russh`) with terminal ANSI rendering, supporting anonymous `guest` sessions (no passwords, no keys, no shell).
3. **Web Gateway**: WebSocket (WSS) ↔ SSH TCP bridge.

The content presents an interactive portfolio, resume, and project browser for **Dmytro Yemelianov** (Systems & AI Workflows Engineer).

---

## Architecture Overview

```text
                         ┌──────────────────┐
                         │    Rust Core     │
                         │                  │
                         │  Application     │
                         │  Session State   │
                         │  View Model      │
                         └────────┬─────────┘
                                  │
                    ┌─────────────┴─────────────┐
                    │                           │
                    ▼                           ▼
            Web Pixel Renderer           ANSI Renderer
                    │                           │
               Rust → WASM                 Rust native
                    │                           │
                 WebGL2                         SSH
                    │                           │
                 Canvas                      Terminal
```

### Clean Separation of Concerns
- `crates/view`: Platform-agnostic `View`, `Element` (`Text`, `Rect`, `Sprite`), styling, and normalized `InputEvent` abstractions. Zero dependencies.
- `crates/core`: Application business logic, state management (`App`), 16 project definitions from `yemelianov.dev`, tabs (`Projects`, `Resume`, `Contact`, `Help`), and view generation. Never imports rendering primitives.
- `crates/framebuffer`: 320×200 8-bit indexed framebuffer (64 KB display memory), 256-color palette uniform, and IBM VGA 8×8 bitmap font rasterizer.
- `crates/render-ansi`: Terminal ANSI renderer translating `View` elements into differential ANSI escape sequences, box borders, 256-color codes, and text sanitization.
- `crates/render-web`: WebGL2 R8 texture + 256-color palette lookup uniform fragment shader, fullscreen quad, nearest neighbor filtering.
- `crates/web-client`: WebAssembly browser client, canvas binding, keyboard event listener, and dirty-flag event-driven `requestAnimationFrame` loop.
- `crates/ssh-server`: Native Rust SSH server on port 2222 with anonymous `guest` auth, PTY/window-change support, terminal keyboard decoding, and ANSI streaming.
- `crates/web-gateway`: WebSocket (WSS) ↔ SSH TCP anonymous gateway for browser-based terminal access.

---

## Technical Specifications Satisfied

- **Logical Resolution**: 320 × 200 8-bit indexed framebuffer (64,000 bytes).
- **Zero DOM UI**: Single `<canvas id="screen">`, no `<div>` grids, no browser font rendering.
- **Rendering Pipeline**: WebGL2 R8 texture with palette uniform lookup fragment shader. Nearest neighbor filtering (`GL_NEAREST`).
- **Binary Size**: WebAssembly client is ~66 KB (`.wasm`) and ~22 KB bootstrap JS.
- **Anonymous SSH**: Accessible via `ssh -p 2222 guest@<host>` without password or keys.
- **Security**: Strictly application-only; no shell prompt, SCP, or exec capability is exposed.

---

## Quickstart

### 1. Run the Native SSH Server
```bash
cargo run --bin pixel-ssh-server
```
In another terminal:
```bash
ssh -p 2222 guest@127.0.0.1
```

### 2. Run the Web Interface
Compile the WebAssembly client:
```bash
cargo build -p pixel-ssh-web-client --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/pixel_ssh_web_client.wasm --out-dir web/pkg --target web --no-typescript
```

Serve the web client locally:
```bash
python3 -m http.server 8085 --directory web
```
Open `http://localhost:8085` in any modern browser.

### 3. Run the Web-to-SSH Gateway
```bash
GATEWAY_PORT=8080 SSH_TARGET=127.0.0.1:2222 cargo run --bin pixel-ssh-web-gateway
```

---

## Keyboard Controls
- `1` / `2` / `3` / `4` or `Tab`: Switch tabs (`Projects`, `Resume`, `Contact`, `Help`)
- `Up` / `Down` (or `k` / `j`): Select project in list
- `Enter`: View detailed project metadata, description, and tags
- `Escape` (or `q`): Return to project list
