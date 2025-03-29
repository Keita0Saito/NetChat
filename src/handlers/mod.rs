use super::{
    clients::{AppState},
    network::broadcast
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::tcp::OwnedReadHalf
};
use std::sync::Arc;

const READ_BUFFER_SIZE: usize = 1024;

// Handle client connection lifecycle
pub async fn handle_client(
    mut reader: OwnedReadHalf,
    writer: super::clients::ClientWriter,
    state: Arc<AppState>,
) {
    // Register new user
    let mut user = state.users.register_guest().await;
    
    // Add to active connections
    {
        let mut connections = state.connections.lock().await;
        connections.push(writer.clone());
    }

    // Send welcome message
    let _ = writer.lock().await.write_all(
        format!("Welcome, {}! Your token: {}\n", user.nickname, user.token).as_bytes()
    ).await;

    let mut buffer = [0; READ_BUFFER_SIZE];
    loop {
        // Read incoming data
        let bytes_read = match reader.read(&mut buffer).await {
            Ok(0) => break,  // Graceful disconnect
            Ok(n) => n,
            Err(_) => break,  // Network error
        };

        let msg = String::from_utf8_lossy(&buffer[..bytes_read]);
        let msg = msg.trim();
        
        // Process /nick command
        if msg.starts_with("/nick ") {
            let new_nick = msg[6..].trim();
            if let Some(updated) = state.users.change_nickname(&user.token, new_nick).await {
                let notification = format!("{} changed name to {}\n", user.nickname, updated.nickname);
                let mut connections = state.connections.lock().await;
                broadcast(notification.as_bytes(), &mut *connections).await;
                user = updated;
            }
            continue;
        }

        // Broadcast regular message
        let formatted = format!("{}: {}\n", user.nickname, msg);
        let mut connections = state.connections.lock().await;
        broadcast(formatted.as_bytes(), &mut *connections).await;
    }

    // Cleanup on disconnect
state.users.remove_user(&user.token).await;
}
