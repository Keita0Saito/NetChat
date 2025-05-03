use std::sync::Arc;
use tokio::{io::AsyncWriteExt, net::tcp::OwnedWriteHalf, sync::Mutex};

#[derive(Clone)]
pub struct ClientConnection {
    pub writer: Arc<Mutex<OwnedWriteHalf>>,
    pub user_token: String,
}

impl ClientConnection {
    /// Sends a UTF-8 encoded message to the connected client over TCP.
    pub async fn send(&self, msg: &str) -> tokio::io::Result<()> {
        let mut writer = self.writer.lock().await;
        writer.write_all(msg.as_bytes()).await
    }
}
