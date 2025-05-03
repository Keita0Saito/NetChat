use crate::{models::user::User, network::ClientConnection, storage::AppState};
use std::sync::Arc;

/// Handles chat creation between current user and target user
pub async fn handle_create_chat(
    target_identifier: String, // Accepts both nickname and user ID
    state: &Arc<AppState>,
    user: &mut User,
    writer: &ClientConnection,
) -> Result<(), String> {
    //Find target user by either nickname or ID
    let target_user = {
        let users = state.users.lock().await;
        users
            .find_by_nickname_or_id(&target_identifier)
            .await
            .ok_or_else(|| format!("User '{}' not found", target_identifier))?
    };

    //Check if chat already exists
    let existing_chat = {
        let chats = state.chats.lock().await;
        chats
            .find_existing_chat(&user.token, &target_user.token)
            .await
    };

    if let Some(chat_id) = existing_chat {
        let msg = format!(
            "Chat already exists with {} (ID: {})\n",
            target_user.nickname, chat_id
        );
        return writer.send(&msg).await.map_err(|e| e.to_string());
    }

    //Create new chat
    let chat_id = {
        let mut chats = state.chats.lock().await;
        chats.create_chat(user, &target_user).await
    };

    //Notify both participants
    let success_msg = format!(
        "Private chat created with {}! Chat ID: {}\n",
        target_user.nickname, chat_id
    );

    // Send notification to current user
    writer.send(&success_msg).await.map_err(|e| e.to_string())?;

    // Send notification to target user (if online)
    let connections = state.connections.lock().await;
    if let Some(target_conn) = connections
        .iter()
        .find(|c| c.user_token == target_user.token)
    {
        let _ = target_conn
            .send(&format!(
                "{} started a chat with you! Chat ID: {}\n",
                user.nickname, chat_id
            ))
            .await;
    }

    Ok(())
}
