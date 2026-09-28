use std::sync::Arc;
use log::info;
use rand::rngs::OsRng;
use russh::keys::{Algorithm, PrivateKey};
use russh::server::{Auth, Handler, Msg, Server, Session};
use russh::*;
use tokio::sync::Mutex;

use pixel_ssh_core::App;
use pixel_ssh_render_ansi::AnsiRenderer;
use pixel_ssh_view::{InputEvent, Key};

struct ClientHandler {
    app: App,
    renderer: AnsiRenderer,
    cols: u16,
    rows: u16,
}

struct AppServer {
    clients: Arc<Mutex<std::collections::HashMap<usize, ClientHandler>>>,
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
    clients: Arc<Mutex<std::collections::HashMap<usize, ClientHandler>>>,
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
                app: App::new(),
                renderer: AnsiRenderer::new(80, 50),
                cols: 80,
                rows: 50,
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
            client.cols = if col_width >= 40 { (col_width as u16).clamp(40, 240) } else { 80 };
            client.rows = if row_height >= 10 { (row_height as u16).clamp(10, 100) } else { 50 };
            client.renderer.resize(client.cols, client.rows);

            // Render and show initial view
            let view = client.app.render();
            let ansi = client.renderer.render_view(&view);
            let _ = session.data(channel, ansi.into_bytes().into());
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
            client.cols = if col_width >= 40 { (col_width as u16).clamp(40, 240) } else { 80 };
            client.rows = if row_height >= 10 { (row_height as u16).clamp(10, 100) } else { 50 };
            client.renderer.resize(client.cols, client.rows);

            // Re-render and send
            let view = client.app.render();
            let ansi = client.renderer.render_view(&view);
            let _ = session.data(channel, ansi.into_bytes().into());
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
                let key = if data[i] == b'\x1b' && i + 2 < data.len() && data[i + 1] == b'[' {
                    match data[i + 2] {
                        b'A' => { i += 3; Some(Key::Up) }
                        b'B' => { i += 3; Some(Key::Down) }
                        b'C' => { i += 3; Some(Key::Right) }
                        b'D' => { i += 3; Some(Key::Left) }
                        _ => { i += 3; None }
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
                } else if data[i] == b'\x1b' {
                    i += 1;
                    Some(Key::Escape)
                } else if data[i] >= 32 {
                    let ch = data[i] as char;
                    i += 1;
                    Some(Key::Char(ch))
                } else {
                    i += 1;
                    None
                };

                if let Some(k) = key {
                    if client.app.update(InputEvent::KeyDown(k)) {
                        dirty = true;
                    }
                }
            }

            if dirty || data.is_empty() {
                let view = client.app.render();
                let ansi = client.renderer.render_view(&view);
                let _ = session.data(channel, ansi.into_bytes().into());
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

    let mut server = AppServer {
        clients: Arc::new(Mutex::new(std::collections::HashMap::new())),
        next_id: 1,
    };

    server.run_on_address(config, addr.as_str()).await?;

    Ok(())
}
