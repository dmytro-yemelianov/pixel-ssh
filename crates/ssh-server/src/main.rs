use std::sync::Arc;
use log::info;
use rand::rngs::OsRng;
use russh::keys::{Algorithm, PrivateKey};
use russh::server::{Auth, Handler, Msg, Server, Session};
use russh::*;
use std::collections::HashMap;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::Mutex;

use pixel_ssh_core::{App, Tab};
use pixel_ssh_render_ansi::AnsiRenderer;
use pixel_ssh_view::{InputEvent, Key};

struct ClientHandler {
    app: App,
    renderer: AnsiRenderer,
    cols: u16,
    rows: u16,
    tx: Option<UnboundedSender<Vec<u8>>>,
    last_rendered: String,
}

struct AppServer {
    clients: Arc<Mutex<HashMap<usize, ClientHandler>>>,
    next_id: usize,
}

impl Server for AppServer {
    type Handler = AppSession;

    fn new_client(&mut self, _peer_addr: Option<std::net::SocketAddr>) -> Self::Handler {
        let id = self.next_id;
        self.next_id += 1;
        AppSession {
            id,
            clients: self.clients.clone(),
        }
    }
}

struct AppSession {
    id: usize,
    clients: Arc<Mutex<HashMap<usize, ClientHandler>>>,
}

impl Drop for AppSession {
    fn drop(&mut self) {
        let clients = self.clients.clone();
        let id = self.id;
        tokio::spawn(async move {
            clients.lock().await.remove(&id);
        });
    }
}

impl Handler for AppSession {
    type Error = russh::Error;

    async fn auth_none(&mut self, user: &str) -> Result<Auth, Self::Error> {
        info!("Anonymous SSH auth requested for user: '{}'", user);
        // Allow anonymous guest access unconditionally
        Ok(Auth::Accept)
    }

    async fn auth_password(&mut self, user: &str, _pass: &str) -> Result<Auth, Self::Error> {
        info!("Password auth requested for user: '{}'", user);
        Ok(Auth::Accept)
    }

    async fn auth_publickey(&mut self, user: &str, _publickey: &russh::keys::PublicKey) -> Result<Auth, Self::Error> {
        info!("Publickey auth requested for user: '{}'", user);
        Ok(Auth::Accept)
    }

    async fn channel_open_session(&mut self, _channel: Channel<Msg>, _session: &mut Session) -> Result<bool, Self::Error> {
        info!("Channel open session for client {}", self.id);
        let mut clients = self.clients.lock().await;
        clients.insert(
            self.id,
            ClientHandler {
                app: App::new_terminal(),
                renderer: AnsiRenderer::new(80, 50),
                cols: 80,
                rows: 50,
                tx: None,
                last_rendered: String::new(),
            },
        );
        Ok(true)
    }

    async fn pty_request(
        &mut self,
        channel: ChannelId,
        term: &str,
        col_width: u32,
        row_height: u32,
        _pix_width: u32,
        _pix_height: u32,
        _modes: &[(Pty, u32)],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        info!("PTY request: term={}, cols={}, rows={}", term, col_width, row_height);
        let mut clients = self.clients.lock().await;
        if let Some(client) = clients.get_mut(&self.id) {
            client.cols = if col_width >= 40 { (col_width as u16).clamp(40, 300) } else { 80 };
            client.rows = if row_height >= 10 { (row_height as u16).clamp(10, 150) } else { 25 };
            client.app.set_terminal_size(client.cols, client.rows);
            client.renderer.resize(client.cols, client.rows);

            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Vec<u8>>();
            let handle = session.handle();
            tokio::spawn(async move {
                while let Some(msg) = rx.recv().await {
                    if handle.data(channel, msg.into()).await.is_err() {
                        break;
                    }
                }
            });

            // Enter alternate screen buffer (\x1b[?1049h), clear screen, move to (1,1)
            let mut init_seq = String::from("\x1b[?1049h\x1b[2J\x1b[H");
            let view = client.app.render();
            let ansi = client.renderer.render_view(&view);
            client.last_rendered = ansi.clone();
            init_seq.push_str(&ansi);
            let _ = tx.send(init_seq.into_bytes());
            client.tx = Some(tx);
        }
        Ok(())
    }

    async fn window_change_request(
        &mut self,
        channel: ChannelId,
        col_width: u32,
        row_height: u32,
        _pix_width: u32,
        _pix_height: u32,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        let mut clients = self.clients.lock().await;
        if let Some(client) = clients.get_mut(&self.id) {
            client.cols = if col_width >= 40 { (col_width as u16).clamp(40, 300) } else { 80 };
            client.rows = if row_height >= 10 { (row_height as u16).clamp(10, 150) } else { 25 };
            client.app.set_terminal_size(client.cols, client.rows);
            client.renderer.resize(client.cols, client.rows);

            // Re-render and send with clear to avoid any artifacts on resize
            let mut redraw = String::from("\x1b[2J\x1b[H");
            let view = client.app.render();
            let ansi = client.renderer.render_view(&view);
            client.last_rendered = ansi.clone();
            redraw.push_str(&ansi);
            if let Some(tx) = &client.tx {
                let _ = tx.send(redraw.into_bytes());
            } else {
                let _ = session.data(channel, redraw.into_bytes().into());
            }
        }
        Ok(())
    }

    async fn data(
        &mut self,
        channel: ChannelId,
        data: &[u8],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        let mut clients = self.clients.lock().await;
        if let Some(client) = clients.get_mut(&self.id) {
            // Parse ANSI terminal key sequences
            let mut i = 0;
            let mut dirty = false;

            while i < data.len() {
                // Ctrl+C: exit session cleanly
                if data[i] == 0x03 {
                    if let Some(tx) = &client.tx {
                        let _ = tx.send("\x1b[?1049l\x1b[?25h\r\nConnection closed.\r\n".as_bytes().to_vec());
                    } else {
                        let _ = session.data(channel, "\x1b[?1049l\x1b[?25h\r\nConnection closed.\r\n".as_bytes().to_vec().into());
                    }
                    let _ = session.close(channel);
                    return Ok(());
                }

                let key = if data[i] == b'\x1b' {
                    if i + 1 >= data.len() {
                        i += 1;
                        Some(Key::Escape)
                    } else if data[i + 1] == b'[' {
                        if i + 2 < data.len() {
                            match data[i + 2] {
                                b'A' => { i += 3; Some(Key::Up) }
                                b'B' => { i += 3; Some(Key::Down) }
                                b'C' => { i += 3; Some(Key::Right) }
                                b'D' => { i += 3; Some(Key::Left) }
                                b'H' => { i += 3; Some(Key::Home) }
                                b'F' => { i += 3; Some(Key::End) }
                                b'1'..=b'6' if i + 3 < data.len() && data[i + 3] == b'~' => {
                                    let digit = data[i + 2];
                                    i += 4;
                                    match digit {
                                        b'1' => Some(Key::Home),
                                        b'4' => Some(Key::End),
                                        b'5' => Some(Key::PageUp),
                                        b'6' => Some(Key::PageDown),
                                        b'3' => Some(Key::Backspace),
                                        _ => None,
                                    }
                                }
                                _ => {
                                    i += 3;
                                    while i < data.len() && (data[i] < 0x40 || data[i] > 0x7E) {
                                        i += 1;
                                    }
                                    if i < data.len() {
                                        i += 1;
                                    }
                                    None
                                }
                            }
                        } else {
                            i += 2;
                            None
                        }
                    } else if data[i + 1] == b'O' && i + 2 < data.len() {
                        match data[i + 2] {
                            b'H' => { i += 3; Some(Key::Home) }
                            b'F' => { i += 3; Some(Key::End) }
                            b'A' => { i += 3; Some(Key::Up) }
                            b'B' => { i += 3; Some(Key::Down) }
                            b'C' => { i += 3; Some(Key::Right) }
                            b'D' => { i += 3; Some(Key::Left) }
                            _ => { i += 3; None }
                        }
                    } else {
                        i += 1;
                        Some(Key::Escape)
                    }
                } else if data[i] == b'\r' || data[i] == b'\n' {
                    i += 1;
                    Some(Key::Enter)
                } else if data[i] == b'\t' {
                    i += 1;
                    Some(Key::Tab)
                } else if data[i] == 0x7f || data[i] == 0x08 {
                    i += 1;
                    Some(Key::Backspace)
                } else if data[i] >= 32 {
                    let ch = data[i] as char;
                    i += 1;
                    Some(Key::Char(ch))
                } else {
                    i += 1;
                    None
                };

                if let Some(k) = key {
                    // Check if user pressed 'q' on main screen (not in detail, not in help)
                    if (k == Key::Char('q') || k == Key::Char('Q')) && !client.app.show_detail && client.app.current_tab != Tab::Help {
                        if let Some(tx) = &client.tx {
                            let _ = tx.send("\x1b[?1049l\x1b[?25h\r\nGoodbye!\r\n".as_bytes().to_vec());
                        } else {
                            let _ = session.data(channel, "\x1b[?1049l\x1b[?25h\r\nGoodbye!\r\n".as_bytes().to_vec().into());
                        }
                        let _ = session.close(channel);
                        return Ok(());
                    }

                    if client.app.update(InputEvent::KeyDown(k)) {
                        dirty = true;
                    }
                }
            }

            if dirty || data.is_empty() {
                let view = client.app.render();
                let ansi = client.renderer.render_view(&view);
                if ansi != client.last_rendered {
                    client.last_rendered = ansi.clone();
                    if let Some(tx) = &client.tx {
                        let _ = tx.send(ansi.into_bytes());
                    } else {
                        let _ = session.data(channel, ansi.into_bytes().into());
                    }
                }
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    info!("Starting Pixel-First Native SSH Server...");

    // Generate ephemeral Ed25519 host key
    let key = PrivateKey::random(&mut OsRng, Algorithm::Ed25519).expect("Failed to generate Ed25519 host key");

    let config = russh::server::Config {
        inactivity_timeout: Some(std::time::Duration::from_secs(3600)),
        auth_rejection_time: std::time::Duration::from_secs(0),
        auth_rejection_time_initial: Some(std::time::Duration::from_secs(0)),
        keys: vec![key],
        ..Default::default()
    };
    let config = Arc::new(config);

    let port = std::env::var("SSH_PORT").unwrap_or_else(|_| "2222".to_string());
    let addr = format!("0.0.0.0:{}", port);
    info!("SSH Server listening on ssh://guest@{} (No password required)", addr);

    let clients: Arc<Mutex<HashMap<usize, ClientHandler>>> = Arc::new(Mutex::new(HashMap::new()));
    let ticker_clients = clients.clone();

    // Spawn background marquee and clock ticker loop (10 Hz = 100ms interval, matching Web client)
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(100));
        loop {
            interval.tick().await;
            let mut clients_guard = ticker_clients.lock().await;
            clients_guard.retain(|_, client| {
                if let Some(tx) = &client.tx {
                    client.app.tick();
                    let view = client.app.render();
                    let ansi = client.renderer.render_view(&view);
                    // Only push an ANSI frame when the rendered visual content actually changes
                    if ansi != client.last_rendered {
                        client.last_rendered = ansi.clone();
                        tx.send(ansi.into_bytes()).is_ok()
                    } else {
                        true
                    }
                } else {
                    true
                }
            });
        }
    });

    let mut server = AppServer {
        clients,
        next_id: 1,
    };

    server.run_on_address(config, addr.as_str()).await?;

    Ok(())
}
