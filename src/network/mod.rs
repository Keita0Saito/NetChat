use tokio::{
    net::tcp::OwnedWriteHalf,
    sync::Mutex,
};
use std::sync::Arc;
use tokio::io::AsyncWriteExt; 

pub type ClientWriter = Arc<Mutex<OwnedWriteHalf>>;

pub async fn broadcast(msg: &str, connections: &mut Vec<ClientWriter>) {
    for client in connections.iter_mut() {
        let mut guard = client.lock().await;
        let _ = guard.write_all(msg.as_bytes()).await;
    }
}
