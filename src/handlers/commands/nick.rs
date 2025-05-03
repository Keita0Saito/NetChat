use crate::{models::user::User, storage::AppState};
use std::sync::Arc;

/// Handles nickname change command and broadcasts the change to all connected clients.
pub async fn handle_nick(
    new_nick: String,
    state: &Arc<AppState>,
    user: &mut User,
) -> Result<(), String> {
    let mut users = state.users.lock().await;
    if let Some(updated) = users.change_nickname(&user.token, &new_nick).await {
        let msg = format!("{} changed name to {}\n", user.nickname, updated.nickname);
        *user = updated;
        let connections = state.connections.lock().await;
        for client in connections.iter() {
            let _ = client.send(&msg).await;
        }
    }
    Ok(())
}
