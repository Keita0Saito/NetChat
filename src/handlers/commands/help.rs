use crate::network::ClientConnection;

pub async fn handle_help(writer: &ClientConnection) -> Result<(), String> {
    let msg = r#"Available commands:
/help — show this help message
/nick <new_nick> — change your nickname
/find <nick> — find a user by nickname
/create <nick> — create a private chat with a user
/msg <chat_id> <message> — send a message to a private chat"#;

    // Send notification to current user
    writer.send(&msg).await.map_err(|e| e.to_string())?;
    Ok(())
}
