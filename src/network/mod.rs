use std::sync::Arc;
use tokio::{io::AsyncWriteExt, net::tcp::OwnedWriteHalf, sync::Mutex};

#[derive(Clone)]
pub struct ClientConnection {
    pub writer: Arc<Mutex<OwnedWriteHalf>>,
    pub user_token: String,
}

impl ClientConnection {
    pub async fn send(&self, msg: &str) -> tokio::io::Result<()> {
        let mut writer = self.writer.lock().await;
        println!("Sending message to {}: {}", self.user_token, msg);
        writer.write_all(msg.as_bytes()).await
    }
}
