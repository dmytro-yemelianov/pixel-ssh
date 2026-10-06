mod input;
#[cfg(test)]
mod tests;

use input::InputParser;
use log::info;
use rand::rngs::OsRng;
use russh::keys::{Algorithm, PrivateKey};
use russh::server::{Auth, Handler, Msg, Server, Session};
use russh::*;
use std::collections::HashMap;
use std::io::Write;
use std::sync::Arc;
use tokio::sync::mpsc::{error::TrySendError, Sender};
use tokio::sync::Mutex;

use pixel_ssh_core::{App, Tab};
use pixel_ssh_render_ansi::AnsiRenderer;
use pixel_ssh_view::{ActiveModal, InputEvent, Key};

struct ClientHandler {
    app: App,
    renderer: AnsiRenderer,
    cols: u16,
    rows: u16,
    tx: Option<Sender<Vec<u8>>>,
    input: InputParser,
    pty_allocated: bool,
}

type ClientKey = (usize, ChannelId);
type SharedClients = Arc<Mutex<HashMap<ClientKey, Arc<Mutex<ClientHandler>>>>>;

impl ClientHandler {
    fn new() -> Self {
        Self {
            app: App::new_terminal(),
            renderer: AnsiRenderer::new(80, 25),
            cols: 80,
            rows: 25,
            tx: None,
            input: InputParser::default(),
            pty_allocated: false,
        }
    }

    fn resize(&mut self, cols: u32, rows: u32) {
        self.cols = if cols == 0 {
            80
        } else {
            cols.clamp(40, 300) as u16
        };
        self.rows = if rows == 0 {
            25
        } else {
            rows.clamp(20, 150) as u16
        };
        self.app.set_terminal_size(self.cols, self.rows);
        self.renderer.resize(self.cols, self.rows);
    }
}

struct AppServer {
    clients: SharedClients,
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
    clients: SharedClients,
}

impl Drop for AppSession {
    fn drop(&mut self) {
        let clients = self.clients.clone();
        let id = self.id;
        tokio::spawn(async move {
            clients
                .lock()
                .await
                .retain(|(connection, _), _| *connection != id);
        });
    }
}

impl Handler for AppSession {
    type Error = russh::Error;

    async fn auth_none(&mut self, user: &str) -> Result<Auth, Self::Error> {
        info!("Anonymous SSH auth requested for user: '{user}'");
        // Only the anonymous guest account can open an application session.
        Ok(if user == "guest" {
            Auth::Accept
        } else {
            Auth::Reject {
                proceed_with_methods: None,
            }
        })
    }

    async fn auth_password(&mut self, user: &str, _pass: &str) -> Result<Auth, Self::Error> {
        info!("Password auth requested for user: '{user}'");
        Ok(Auth::Reject {
            proceed_with_methods: None,
        })
    }

    async fn auth_publickey(
        &mut self,
        user: &str,
        _publickey: &russh::keys::PublicKey,
    ) -> Result<Auth, Self::Error> {
        info!("Publickey auth requested for user: '{user}'");
        Ok(Auth::Reject {
            proceed_with_methods: None,
        })
    }

    async fn channel_open_session(
        &mut self,
        channel: Channel<Msg>,
        _session: &mut Session,
    ) -> Result<bool, Self::Error> {
        info!("Channel open session for client {}", self.id);
        self.clients.lock().await.insert(
            (self.id, channel.id()),
            Arc::new(Mutex::new(ClientHandler::new())),
        );
        Ok(true)
    }

    async fn channel_close(
        &mut self,
        channel: ChannelId,
        _session: &mut Session,
    ) -> Result<(), Self::Error> {
        self.clients.lock().await.remove(&(self.id, channel));
        Ok(())
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
        info!("PTY request: term={term}, cols={col_width}, rows={row_height}");
        let client = self.clients.lock().await.get(&(self.id, channel)).cloned();
        if let Some(client) = client {
            let mut client = client.lock().await;
            if !client.pty_allocated && client.tx.is_none() {
                client.resize(col_width, row_height);
                client.pty_allocated = true;
                return session.channel_success(channel);
            }
        }
        session.channel_failure(channel)
    }

    async fn shell_request(
        &mut self,
        channel: ChannelId,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        let client = self.clients.lock().await.get(&(self.id, channel)).cloned();
        if let Some(client) = client {
            let mut client = client.lock().await;
            if client.tx.is_some() {
                return session.channel_failure(channel);
            }
            session.channel_success(channel)?;
            let (tx, mut rx) = tokio::sync::mpsc::channel::<Vec<u8>>(2);
            let handle = session.handle();
            tokio::spawn(async move {
                while let Some(msg) = rx.recv().await {
                    if handle.data(channel, msg.into()).await.is_err() {
                        break;
                    }
                }
            });
            let mut init_seq = String::from("\x1b[?1049h\x1b[2J\x1b[H");
            let view = client.app.render();
            init_seq.push_str(&client.renderer.render_view(&view));
            let _ = tx.try_send(init_seq.into_bytes());
            client.tx = Some(tx);
            return Ok(());
        }
        session.channel_failure(channel)
    }

    async fn exec_request(
        &mut self,
        channel: ChannelId,
        _command: &[u8],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        session.channel_failure(channel)
    }

    async fn subsystem_request(
        &mut self,
        channel: ChannelId,
        _name: &str,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        session.channel_failure(channel)
    }

    async fn env_request(
        &mut self,
        channel: ChannelId,
        _name: &str,
        _value: &str,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        session.channel_failure(channel)
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
        let client = self.clients.lock().await.get(&(self.id, channel)).cloned();
        if let Some(client) = client {
            let mut client = client.lock().await;
            client.resize(col_width, row_height);
            if client.tx.is_none() {
                return session.channel_success(channel);
            }

            // Re-render and send with clear to avoid any artifacts on resize
            let mut redraw = String::from("\x1b[2J\x1b[H");
            let view = client.app.render();
            let ansi = client.renderer.render_view(&view);
            redraw.push_str(&ansi);
            if let Some(tx) = &client.tx {
                if tx.try_send(redraw.into_bytes()).is_err() {
                    client.renderer.force_full_redraw();
                }
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
        let client = self.clients.lock().await.get(&(self.id, channel)).cloned();
        if let Some(client) = client {
            let mut client = client.lock().await;
            if client.tx.is_none() {
                return Ok(());
            }
            let keys = client.input.feed(data, tokio::time::Instant::now());
            let mut dirty = false;
            for key in keys {
                let interrupt = key == Key::Char('\u{3}');
                let quit = matches!(key, Key::Char('q' | 'Q'))
                    && !client.app.show_detail
                    && client.app.current_tab == Tab::Projects
                    && client.app.active_modal == ActiveModal::None
                    && !client.app.screensaver_active;
                if interrupt || quit {
                    let farewell = if interrupt {
                        "Connection closed."
                    } else {
                        "Goodbye!"
                    };
                    // Queue the final output before closing, without dropping it when
                    // the background renderer's bounded queue is full.
                    session.data(
                        channel,
                        format!("\x1b[?1049l\x1b[?25h\r\n{farewell}\r\n")
                            .into_bytes()
                            .into(),
                    )?;
                    session.exit_status_request(channel, 0)?;
                    session.eof(channel)?;
                    session.close(channel)?;
                    drop(client);
                    self.clients.lock().await.remove(&(self.id, channel));
                    return Ok(());
                }
                dirty |= client.app.update(InputEvent::KeyDown(key));
                // Terminal users follow the rendered OSC 8 link in their own client.
                if let Some(url) = client.app.take_link_activation() {
                    client.app.status = format!("Open link in your terminal: {url}");
                }
            }

            if dirty || data.is_empty() {
                let view = client.app.render();
                let ansi = client.renderer.render_view(&view);
                if !ansi.is_empty() {
                    if let Some(tx) = &client.tx {
                        if tx.try_send(ansi.into_bytes()).is_err() {
                            client.renderer.force_full_redraw();
                        }
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

    // Keep a stable host identity across restarts so SSH clients can verify it.
    let host_key_path =
        std::env::var("SSH_HOST_KEY").unwrap_or_else(|_| "ssh_host_ed25519_key".to_string());
    let key = if std::path::Path::new(&host_key_path).exists() {
        russh::keys::load_secret_key(&host_key_path, None)?
    } else {
        let key = PrivateKey::random(&mut OsRng, Algorithm::Ed25519)?;
        let mut options = std::fs::OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let mut file = options.open(&host_key_path)?;
        russh::keys::encode_pkcs8_pem(&key, &mut file)?;
        file.flush()?;
        key
    };

    let config = russh::server::Config {
        inactivity_timeout: Some(std::time::Duration::from_secs(3600)),
        auth_rejection_time: std::time::Duration::from_secs(0),
        auth_rejection_time_initial: Some(std::time::Duration::from_secs(0)),
        keys: vec![key],
        ..Default::default()
    };
    let config = Arc::new(config);

    let port = std::env::var("SSH_PORT").unwrap_or_else(|_| "2222".to_string());
    let addr = format!("0.0.0.0:{port}");
    info!("SSH Server listening on ssh://guest@{addr} (No password required)");

    let clients: SharedClients = Arc::new(Mutex::new(HashMap::new()));
    let ticker_clients = clients.clone();

    // Spawn background marquee and clock ticker loop (10 Hz = 100ms interval, matching Web client)
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(100));
        loop {
            interval.tick().await;
            let snapshot: Vec<_> = ticker_clients
                .lock()
                .await
                .iter()
                .map(|(&id, client)| (id, client.clone()))
                .collect();
            let mut closed = Vec::new();
            for (id, client) in snapshot {
                let mut client = client.lock().await;
                if let Some(tx) = client.tx.clone() {
                    if let Some(key) = client.input.expire(tokio::time::Instant::now()) {
                        client.app.update(InputEvent::KeyDown(key));
                    }
                    client.app.tick();
                    let view = client.app.render();
                    let ansi = client.renderer.render_view(&view);
                    if !ansi.is_empty() {
                        match tx.try_send(ansi.into_bytes()) {
                            Ok(()) => {}
                            Err(TrySendError::Full(_)) => client.renderer.force_full_redraw(),
                            Err(TrySendError::Closed(_)) => closed.push(id),
                        }
                    }
                }
            }
            if !closed.is_empty() {
                let mut clients_guard = ticker_clients.lock().await;
                for id in closed {
                    clients_guard.remove(&id);
                }
            }
        }
    });

    let mut server = AppServer {
        clients,
        next_id: 1,
    };

    server.run_on_address(config, addr.as_str()).await?;

    Ok(())
}
