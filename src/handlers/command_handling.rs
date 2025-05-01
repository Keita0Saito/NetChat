use crate::{clients::{User, AppState}, network::broadcast};
use std::sync::Arc;

pub enum Command {
    Nick(String),
    FindUser(String),
}

pub fn parse_command(msg: &str) -> Option<Command> {
    msg.split_once(' ')
        .and_then(|(cmd, arg)| match cmd {
            "/nick" => Some(Command::Nick(arg.trim().to_string())),
            "/find_user" => Some(Command::FindUser(arg.trim().to_string())),
            _ => None
        })
}

pub async fn handle_command(
    command: Command,
    state: &Arc<AppState>,
    user: &mut User,
) {
    match command {
        Command::Nick(new_nick) => {
            let updated = {
                let mut users = state.users.lock().await;
                users.change_nickname(&user.token, &new_nick).await
            };

            if let Some(updated_user) = updated {
                let msg = format!("{} changed name to {}\n", user.nickname, updated_user.nickname);
                let mut connections = state.connections.lock().await;
                broadcast(&msg, &mut *connections).await;
                *user = updated_user;
            }
        }

        Command::FindUser(user_nick) => {
            let user_token = {
                let users = state.users.lock().await;
                users.find_by_nickname(&user_nick).await.map(|u| u.token.clone())
            };

            let mut connections = state.connections.lock().await;
            match user_token {
                Some(token) => {
                    let msg = format!("User {} found with token: {}\n", user_nick, token);
                    broadcast(&msg, &mut *connections).await;
                },
                None => {
                    let msg = format!("User {} not found\n", user_nick);
                    broadcast(&msg, &mut *connections).await;
                }
            }
        }
    }
}
