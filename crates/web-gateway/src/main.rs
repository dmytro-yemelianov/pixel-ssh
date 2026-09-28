use futures_util::{SinkExt, StreamExt};
use log::{error, info};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_hdr_async;
use tokio_tungstenite::tungstenite::handshake::server::{ErrorResponse, Request, Response};
use tokio_tungstenite::tungstenite::http::StatusCode;

fn origin_allowed(origin: &str, allowed_origins: &str) -> bool {
    allowed_origins
        .split(',')
        .any(|entry| entry.trim() == origin)
}
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let ws_port = std::env::var("GATEWAY_PORT").unwrap_or_else(|_| "8080".to_string());
    let ssh_addr = std::env::var("SSH_TARGET").unwrap_or_else(|_| "127.0.0.1:2222".to_string());
    let bind_addr = std::env::var("GATEWAY_BIND").unwrap_or_else(|_| "127.0.0.1".to_string());
    let ws_addr = format!("{bind_addr}:{ws_port}");
    let allowed_origins = std::env::var("GATEWAY_ORIGINS")
        .unwrap_or_else(|_| "http://localhost:8085,http://127.0.0.1:8085".to_string());

    let listener = TcpListener::bind(&ws_addr).await?;
    info!("WebSocket ↔ SSH Anonymous Gateway listening on ws://{ws_addr}");
    info!("Bridging traffic to SSH server at {ssh_addr}");

    loop {
        let (stream, peer_addr) = listener.accept().await?;
        let ssh_addr = ssh_addr.clone();
        let allowed_origins = allowed_origins.clone();
        tokio::spawn(async move {
            info!("Accepted WebSocket connection from {peer_addr}");
            if let Err(e) = handle_client(stream, ssh_addr, allowed_origins).await {
                error!("Connection error for {peer_addr}: {e:?}");
            }
            info!("Connection closed for {peer_addr}");
        });
    }
}

async fn handle_client(
    stream: TcpStream,
    ssh_addr: String,
    allowed_origins: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ws_stream = accept_hdr_async(stream, move |request: &Request, response: Response| {
        if let Some(origin) = request.headers().get("origin") {
            let allowed = origin
                .to_str()
                .ok()
                .is_some_and(|origin| origin_allowed(origin, &allowed_origins));
            if !allowed {
                let mut error = ErrorResponse::new(Some("Forbidden origin".to_string()));
                *error.status_mut() = StatusCode::FORBIDDEN;
                return Err(error);
            }
        }
        Ok(response)
    })
    .await?;
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    // Connect to target SSH server
    let ssh_stream = TcpStream::connect(&ssh_addr).await?;
    let (mut ssh_read, mut ssh_write) = ssh_stream.into_split();

    // Task 1: WebSocket -> SSH TCP
    let client_to_ssh = async {
        while let Some(msg) = ws_receiver.next().await {
            let msg = msg?;
            if msg.is_binary() || msg.is_text() {
                let data = msg.into_data();
                ssh_write.write_all(&data).await?;
                ssh_write.flush().await?;
            } else if msg.is_close() {
                break;
            }
        }
        Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
    };

    // Task 2: SSH TCP -> WebSocket
    let ssh_to_client = async {
        let mut buf = [0u8; 4096];
        loop {
            let n = ssh_read.read(&mut buf).await?;
            if n == 0 {
                break;
            }
            ws_sender
                .send(Message::Binary(buf[..n].to_vec().into()))
                .await?;
        }
        Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
    };

    tokio::select! {
        res1 = client_to_ssh => res1?,
        res2 = ssh_to_client => res2?,
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::origin_allowed;

    #[test]
    fn browser_origins_require_an_exact_allowlist_match() {
        let allowed = "http://localhost:8085, http://127.0.0.1:8085";
        assert!(origin_allowed("http://localhost:8085", allowed));
        assert!(origin_allowed("http://127.0.0.1:8085", allowed));
        assert!(!origin_allowed(
            "http://localhost:8085.evil.example",
            allowed
        ));
        assert!(!origin_allowed("https://localhost:8085", allowed));
    }
}
