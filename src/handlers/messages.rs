use crate::{models::user::User, network::ClientConnection, storage::AppState};

/// Sends a welcome message to the newly connected user.
pub async fn send_welcome(writer: &ClientConnection, user: &User) {
    let _ = writer.send(&format!("Welcome, {}!\n", user.nickname)).await;
}

/// Broadcasts a message from the sender to all connected users(expect the sender).
pub async fn broadcast_message(
    state: &AppState,
    sender: &User,
    msg: &str,
    _writer: &ClientConnection,
) {
    let connections = state.connections.lock().await;

    for conn in connections.iter() {
        // Skip the sender.
        if conn.user_token != sender.token {
            match conn.send(&format!("{}: {}\n", sender.nickname, msg)).await {
                Ok(_) => println!("Message sent to {}", conn.user_token),
                Err(e) => eprintln!("Failed to send message: {}", e),
            }
        }
    }
}
