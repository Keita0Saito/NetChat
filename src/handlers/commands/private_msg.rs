use crate::{models::user::User, storage::AppState};
use std::sync::Arc;

/// Handles sending a private message within an existing chat.
pub async fn handle_private_message(
    chat_id: String,
    message: String,
    state: &Arc<AppState>,
    user: &mut User,
) -> Result<(), String> {
    let chat_info = {
        let chats = state.chats.lock().await;
        chats.get_chat(&chat_id).await
    };

    match chat_info {
        Some((user1, user2)) => {
            if user.token != user1.token && user.token != user2.token {
                return Err("Not a chat participant".to_string());
            }

            let recipient = if user.token == user1.token {
                &user2
            } else {
                &user1
            };

            let mut messages = state.messages.lock().await;
            messages.add_message(&chat_id, user, &message).await;

            let msg = format!(
                "[Private from {} in chat {}]: {}\n",
                user.nickname, chat_id, message
            );

            let connections = state.connections.lock().await;

            if let Some(conn) = connections.iter().find(|c| c.user_token == recipient.token) {
                match conn.send(&msg).await {
                    Ok(_) => println!("[/pm] Message sent successfully."),
                    Err(e) => println!("[/pm] Failed to send message: {}", e),
                }
            }

            Ok(())
        }
        None => Err("Chat not found".to_string()),
    }
}
