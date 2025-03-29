use crate::{
    clients::User,
    network::ClientWriter,
    clients::AppState
};
use std::sync::Arc;
use tokio::io::AsyncWriteExt;

pub async fn send_welcome(writer: &ClientWriter, user: &User) {
    let msg = format!("Welcome, {}! Your token: {}\n", user.nickname, user.token);
    let mut guard = writer.lock().await;
    if let Err(e) = guard.write_all(msg.as_bytes()).await {
        eprintln!("Failed to send welcome message: {}", e);
    }
}

pub async fn broadcast_message(
    state: &Arc<AppState>,
    user: &User,
    message: &str,
    sender: &ClientWriter,
) {
    let msg = format!("{}: {}\n", user.nickname, message);
    let mut connections = state.connections.lock().await;
    
    for client in connections.iter_mut() {
        if !Arc::ptr_eq(client, sender) {
            let mut guard = client.lock().await;
            let _ = guard.write_all(msg.as_bytes()).await;
        }
    }
}
