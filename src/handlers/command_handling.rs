use crate::{
    clients::{AppState, User},
    network::ClientConnection,
};
use std::sync::Arc;

#[derive(Debug)]
pub enum Command {
    Nick(String),
    FindUser(String),
    CreateChatWith(String),
    SendPrivateMessage { chat_id: String, message: String },
}

pub fn parse_command(msg: &str) -> Option<Command> {
    let parts: Vec<&str> = msg.splitn(3, ' ').collect();
    match parts.as_slice() {
        ["/nick", arg] => Some(Command::Nick(arg.trim().to_string())),
        ["/find_user", arg] => Some(Command::FindUser(arg.trim().to_string())),
        ["/create_chat_with", arg] => Some(Command::CreateChatWith(arg.trim().to_string())),
        ["/pm", chat_id, message] => Some(Command::SendPrivateMessage {
            chat_id: chat_id.to_string(),
            message: message.to_string(),
        }),
        _ => None,
    }
}

pub async fn handle_command(
    command: Command,
    state: &Arc<AppState>,
    user: &mut User,
    writer: &ClientConnection,
) -> Result<(), String> {
    println!("Parsed command: {:?}", command);
    match command {
        Command::Nick(new_nick) => {
            let mut users = state.users.lock().await;
            if let Some(updated) = users.change_nickname(&user.token, &new_nick).await {
                let msg = format!("{} changed name to {}\n", user.nickname, updated.nickname);
                *user = updated;
                let mut connections = state.connections.lock().await;
                for client in connections.iter() {
                    let _ = client.send(&msg).await;
                }
            }
            Ok(())
        }

        Command::FindUser(user_nick) => {
            let users = state.users.lock().await;
            match users.find_by_nickname(&user_nick).await {
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

        Command::CreateChatWith(target_nick) => {
            let target_user = {
                let users = state.users.lock().await;
                users.find_by_nickname(&target_nick).await
            };

            match target_user {
                Some(target_user) => {
                    let mut chats = state.chats.lock().await;
                    let chat_id = chats.create_chat(user, &target_user).await;
                    let msg = format!(
                        "Private chat created with {}! Chat ID: {}\n",
                        target_nick, chat_id
                    );
                    writer.send(&msg).await.map_err(|e| e.to_string())
                }
                None => {
                    let msg = format!("User {} not found\n", target_nick);
                    writer.send(&msg).await.map_err(|e| e.to_string())
                }
            }
        }

        Command::SendPrivateMessage { chat_id, message } => {
            let chat_info = {
                let chats = state.chats.lock().await;
                println!("[/pm] Looking up chat with id: {}", chat_id);
                chats.get_chat(&chat_id).await
            };

            println!("[/pm] Chat info for {}: {:?}", chat_id, chat_info);

            match chat_info {
                Some((user1, user2)) => {
                    println!("[/pm] Participants: {}, {}", user1.token, user2.token);
                    println!("[/pm] Current user: {}", user.token);

                    if user.token != user1.token && user.token != user2.token {
                        println!("[/pm] User is not a participant.");
                        return Err("Not a chat participant".to_string());
                    }

                    let recipient = if user.token == user1.token {
                        &user2
                    } else {
                        &user1
                    };

                    println!("[/pm] Recipient token: {}", recipient.token);

                    let mut messages = state.messages.lock().await;
                    messages.add_message(&chat_id, user, &message).await;

                    let msg = format!(
                        "[Private from {} in chat {}]: {}\n",
                        user.nickname, chat_id, message
                    );

                    let mut connections = state.connections.lock().await;
                    println!(
                        "[/pm] Searching for recipient in {} connections...",
                        connections.len()
                    );

                    if let Some(conn) = connections.iter().find(|c| c.user_token == recipient.token)
                    {
                        println!("[/pm] Found recipient connection. Sending message...");
                        match conn.send(&msg).await {
                            Ok(_) => println!("[/pm] Message sent successfully."),
                            Err(e) => println!("[/pm] Failed to send message: {}", e),
                        }
                    } else {
                        println!("[/pm] Recipient connection not found.");
                    }

                    Ok(())
                }
                None => {
                    println!("[/pm] Chat not found for id: {}", chat_id);
                    Err("Chat not found".to_string())
                }
            }
        }
    }
}
