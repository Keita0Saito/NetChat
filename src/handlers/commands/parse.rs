use super::Command;

pub fn parse_command(msg: &str) -> Option<Command> {
    let parts: Vec<&str> = msg.splitn(3, ' ').collect();
    match parts.as_slice() {
        ["/help"] => Some(Command::Help),
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
