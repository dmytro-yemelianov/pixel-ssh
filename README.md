# Pixel SSH

An interactive portfolio with a shared Rust application core and two front ends: a WebAssembly/WebGL2 canvas and an ANSI terminal over SSH. Each connection has its own application state.

## Display and controls

The web client supports SVGA 800×600, SGA 640×480, VGA 640×400, EGA 640×350, CGA/C64 320×200, Atari 320×192, and ZX Spectrum 256×192 layouts. Each system selects its native resolution, font, and color palette. Visual effects can be adjusted separately. The controls for every grid size are specified in [the UI control matrix](docs/ui-controls.md).

The framebuffer uses 8-bit color indices. WebGL2 looks up the active 256-color palette from a 256×1 RGBA texture, then applies optional CRT effects. VGA-sized layouts use an 8×16 CP437-ordered bitmap and compact layouts use 8×8 bitmaps. EGA uses a separately sourced 8×14 CP437 bitmap, licensed under CC BY-SA 4.0; see [its attribution](crates/framebuffer/assets/README.md). The provenance status for every bundled font and named palette is documented in [the asset audit](docs/font-palette-provenance.md).

The Projects screen starts with CV, Contacts, and About, then the numbered portfolio entries. The bottom menu is `Prj Hlp Sys Vis Qut`: its first letters open Projects, Help, cycle display systems, Visuals, and Quit. Numbers have no global assignment; `1`–`6` select presets only inside Visuals. `v` opens Visuals. Arrow keys and `j`/`k` navigate lists, Enter activates the selected item, and Escape closes a dialog or returns to Projects. On touch screens, tap the canvas menu and swipe to navigate.

The terminal uses the same content and ANSI rendering. `S` cycles display systems, each with its own font and colors.

## Run locally

Build the browser client (requires the `wasm32-unknown-unknown` target and `wasm-bindgen` CLI):

```sh
cargo build -p pixel-ssh-web-client --target wasm32-unknown-unknown --release --locked
wasm-bindgen target/wasm32-unknown-unknown/release/pixel_ssh_web_client.wasm --out-dir web/pkg --target web --no-typescript
python3 -m http.server 8085 --directory web
```

Open `http://localhost:8085`. The generated `web/pkg` files are tracked for static hosting. Their size varies with the build; inspect the built files rather than relying on a fixed size claim.
CI regenerates the tracked browser bundle with Rust 1.88.0 and wasm-bindgen-cli 0.2.129 and fails if it differs from the source.

Run the SSH server in another terminal:

```sh
cargo run --bin pixel-ssh-server
ssh -p 2222 guest@127.0.0.1
```

The server accepts only the `guest` user with SSH `none` authentication. Password and public-key authentication are rejected. Sessions expose the portfolio application, not a shell, SCP, or remote commands. The server binds to port 2222 on all interfaces by default. Its host key is stored in `ssh_host_ed25519_key` in the working directory; set `SSH_HOST_KEY` to use another path and keep this private file between restarts.

## Optional raw WebSocket bridge

`pixel-ssh-web-gateway` forwards binary WebSocket traffic to the SSH TCP server. It does not implement an SSH client in the browser portfolio. It serves plain `ws://`, so use a TLS reverse proxy if exposing it remotely. By default it binds to `127.0.0.1:8080` and accepts browser origins `http://localhost:8085` and `http://127.0.0.1:8085`.

```sh
GATEWAY_PORT=8080 SSH_TARGET=127.0.0.1:2222 cargo run --bin pixel-ssh-web-gateway
```

`GATEWAY_BIND` changes the bind address, and `GATEWAY_ORIGINS` sets a comma-separated browser Origin allowlist. Clients without an Origin header can connect as non-browser WebSocket clients.

## Development checks

```sh
cargo test --workspace
cargo check -p pixel-ssh-web-client --target wasm32-unknown-unknown
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```
