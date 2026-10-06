use super::*;
use russh::client;
use std::time::Duration;
use tokio::time::timeout;

struct TestClient;

impl client::Handler for TestClient {
    type Error = russh::Error;

    async fn check_server_key(
        &mut self,
        _key: &russh::keys::PublicKey,
    ) -> Result<bool, Self::Error> {
        Ok(true)
    }
}

struct TestServer {
    clients: SharedClients,
    task: tokio::task::JoinHandle<()>,
}

impl Drop for TestServer {
    fn drop(&mut self) {
        self.task.abort();
    }
}

impl TestServer {
    async fn connect() -> (Self, client::Handle<TestClient>) {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let clients: SharedClients = Arc::new(Mutex::new(HashMap::new()));
        let mut server = AppServer {
            clients: clients.clone(),
            next_id: 1,
        };
        let config = Arc::new(russh::server::Config {
            keys: vec![PrivateKey::random(&mut OsRng, Algorithm::Ed25519).unwrap()],
            ..Default::default()
        });
        let task = tokio::spawn(async move {
            server.run_on_socket(config, &listener).await.unwrap();
        });
        let mut client = timeout(
            Duration::from_secs(5),
            client::connect(Arc::new(client::Config::default()), address, TestClient),
        )
        .await
        .unwrap()
        .unwrap();
        assert!(client.authenticate_none("guest").await.unwrap().success());
        (Self { clients, task }, client)
    }

    async fn state(&self, channel: ChannelId) -> Arc<Mutex<ClientHandler>> {
        self.clients
            .lock()
            .await
            .get(&(1, channel))
            .cloned()
            .unwrap()
    }
}

async fn reply(channel: &mut Channel<client::Msg>) -> bool {
    timeout(Duration::from_secs(2), async {
        loop {
            match channel
                .wait()
                .await
                .expect("channel closed before request reply")
            {
                ChannelMsg::Success => return true,
                ChannelMsg::Failure => return false,
                _ => {}
            }
        }
    })
    .await
    .expect("server did not acknowledge request")
}

async fn start_shell(client: &client::Handle<TestClient>) -> Channel<client::Msg> {
    let mut channel = client.channel_open_session().await.unwrap();
    channel
        .request_pty(true, "xterm", 80, 25, 0, 0, &[])
        .await
        .unwrap();
    assert!(reply(&mut channel).await);
    channel.request_shell(true).await.unwrap();
    assert!(reply(&mut channel).await);
    timeout(Duration::from_secs(2), async {
        loop {
            if let Some(ChannelMsg::Data { .. }) = channel.wait().await {
                break;
            }
        }
    })
    .await
    .unwrap();
    channel
}

#[tokio::test]
async fn requests_receive_success_or_failure_without_hanging() {
    let (_server, client) = TestServer::connect().await;
    let mut channel = start_shell(&client).await;
    channel.request_shell(true).await.unwrap();
    assert!(!reply(&mut channel).await, "a channel can start only once");
    channel
        .request_pty(true, "xterm", 80, 25, 0, 0, &[])
        .await
        .unwrap();
    assert!(!reply(&mut channel).await, "PTY must precede shell startup");
    channel.exec(true, "true").await.unwrap();
    assert!(!reply(&mut channel).await);
    channel.request_subsystem(true, "sftp").await.unwrap();
    assert!(!reply(&mut channel).await);
    channel.set_env(true, "LANG", "en_US.UTF-8").await.unwrap();
    assert!(!reply(&mut channel).await);
    client
        .disconnect(Disconnect::ByApplication, "test complete", "")
        .await
        .unwrap();
}

#[tokio::test]
async fn channel_input_and_closing_do_not_change_another_channel() {
    let (server, client) = TestServer::connect().await;
    let mut first = start_shell(&client).await;
    let second = start_shell(&client).await;
    first.data(b"\x1b".as_slice()).await.unwrap();
    // A request reply is a barrier: earlier channel data has been processed.
    first.set_env(true, "BARRIER", "1").await.unwrap();
    assert!(!reply(&mut first).await);
    assert_eq!(
        server
            .state(first.id())
            .await
            .lock()
            .await
            .app
            .selected_list_item,
        0
    );
    first.data(b"[B".as_slice()).await.unwrap();
    first.set_env(true, "BARRIER", "2").await.unwrap();
    assert!(!reply(&mut first).await);
    assert_eq!(
        server
            .state(first.id())
            .await
            .lock()
            .await
            .app
            .selected_list_item,
        1
    );
    assert_eq!(
        server
            .state(second.id())
            .await
            .lock()
            .await
            .app
            .selected_list_item,
        0
    );
    first.data(b"\x03".as_slice()).await.unwrap();
    let farewell = timeout(Duration::from_secs(2), async {
        let mut data = Vec::new();
        while let Some(message) = first.wait().await {
            match message {
                ChannelMsg::Data { data: bytes } => data.extend_from_slice(&bytes),
                ChannelMsg::Close => break,
                _ => {}
            }
        }
        String::from_utf8_lossy(&data).contains("Connection closed.")
    })
    .await
    .unwrap();
    assert!(farewell, "final output reaches the channel being closed");
    let state = server.clients.lock().await;
    assert!(!state.contains_key(&(1, first.id())));
    assert!(state.contains_key(&(1, second.id())));
    drop(state);
    client
        .disconnect(Disconnect::ByApplication, "test complete", "")
        .await
        .unwrap();
}
