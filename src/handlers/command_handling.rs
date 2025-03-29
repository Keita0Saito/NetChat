use crate::{clients::{User, AppState}, network::broadcast};
use std::sync::Arc;

pub enum Command {
    Nick(String),
}

pub fn parse_command(msg: &str) -> Option<Command> {
    msg.split_once(' ')
        .and_then(|(cmd, arg)| match cmd {
            "/nick" => Some(Command::Nick(arg.trim().to_string())),
            _ => None
        })
}

pub async fn handle_command(
    command: Command,
    state: &Arc<AppState>,
    user: &User,
) {
    match command {
        Command::Nick(new_nick) => {
            let mut users = state.users.lock().await;
            if let Some(updated) = users.change_nickname(&user.token, &new_nick).await {
                let msg = format!("{} changed name to {}\n", user.nickname, updated.nickname);
                let mut connections = state.connections.lock().await;
                broadcast(&msg, &mut *connections).await;
            }
        }
    }
}
