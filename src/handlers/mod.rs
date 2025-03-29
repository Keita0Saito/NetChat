mod command_handling;
mod connection_handling;
mod message_processing;
mod user_management;

pub use command_handling::handle_command;
pub use connection_handling::{add_connection, remove_connection};
pub use message_processing::{send_welcome, broadcast_message};
pub use user_management::{register_guest, remove_user};

use tokio::{net::tcp::OwnedReadHalf, io::AsyncReadExt};
use crate::{clients::AppState, network::ClientWriter};
use std::sync::Arc;

const READ_BUFFER_SIZE: usize = 1024;

pub async fn handle_client(
    mut reader: OwnedReadHalf,
    writer: ClientWriter,
    state: Arc<AppState>,
) {
    let user = register_guest(&state).await;
    add_connection(&state, &writer).await;
    send_welcome(&writer, &user).await;

    let mut buffer = [0; READ_BUFFER_SIZE];
    loop {
        let bytes_read = match reader.read(&mut buffer).await {
            Ok(0) => break,
            Ok(n) => n,
            Err(_) => break,
        };

        let msg = String::from_utf8_lossy(&buffer[..bytes_read]).trim().to_string();
        
        if let Some(cmd) = command_handling::parse_command(&msg) {
            handle_command(cmd, &state, &user).await;
        } else {
            broadcast_message(&state, &user, &msg, &writer).await;
        }
    }

    remove_user(&state, &user.token).await;
    remove_connection(&state, &writer).await;
}
