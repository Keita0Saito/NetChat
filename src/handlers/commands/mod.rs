mod create_chat;
mod find_user;
mod help;
mod nick;
mod parse;
mod private_msg;

pub use create_chat::handle_create_chat;
pub use find_user::handle_find_user;
pub use help::handle_help;
pub use nick::handle_nick;
pub use parse::parse_command;
pub use private_msg::handle_private_message;

use crate::{models::user::User, network::ClientConnection, storage::AppState};
use std::sync::Arc;

#[derive(Debug)]
pub enum Command {
    Help,
    Nick(String),
    FindUser(String),
    CreateChatWith(String),
    SendPrivateMessage { chat_id: String, message: String },
}

pub async fn handle_command(
    command: Command,
    state: &Arc<AppState>,
    user: &mut User,
    writer: &ClientConnection,
) -> Result<(), String> {
    match command {
        Command::Help => handle_help(writer).await,
        Command::Nick(new_nick) => handle_nick(new_nick, state, user).await,
        Command::FindUser(nick) => handle_find_user(nick, state, writer).await,
        Command::CreateChatWith(nick) => handle_create_chat(nick, state, user, writer).await,
        Command::SendPrivateMessage { chat_id, message } => {
            handle_private_message(chat_id, message, state, user).await
        }
    }
}
