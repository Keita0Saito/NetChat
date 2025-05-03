use crate::{network::ClientConnection, storage::AppState};
use std::sync::Arc;

/// Handles user lookup by nickname or token and sends the result back to the client.
pub async fn handle_find_user(
    user_nick: String,
    state: &Arc<AppState>,
    writer: &ClientConnection,
) -> Result<(), String> {
    println!("Finding user: {}", user_nick);
    let users = state.users.lock().await;
    match users.find_by_nickname_or_id(&user_nick).await {
        Some(found_user) => {
            let msg = format!(
                "User {} found with token: {}\n",
                user_nick, found_user.token
            );
            writer.send(&msg).await.map_err(|e| e.to_string())
        }
        None => {
            let msg = format!("User {} not found\n", user_nick);
            writer.send(&msg).await.map_err(|e| e.to_string())
        }
    }
}
