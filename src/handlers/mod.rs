mod command_handling;
mod connection_handling;
mod message_processing;
mod user_management;

pub use command_handling::handle_command;
pub use connection_handling::{add_connection, remove_connection};
pub use message_processing::{broadcast_message, send_welcome};
pub use user_management::{register_guest, remove_user};

use crate::{clients::AppState, network::ClientConnection};
use std::sync::Arc;
use tokio::{io::AsyncReadExt, net::tcp::OwnedReadHalf};
use tokio::net::tcp::OwnedWriteHalf;

const READ_BUFFER_SIZE: usize = 1024;


pub async fn handle_client(
    mut reader: OwnedReadHalf,
    writer_half: OwnedWriteHalf,
    state: Arc<AppState>,
) {
    let mut user = register_guest(&state).await;

    // Создаём соединение с правильным user_token
    let connection = ClientConnection {
        writer: Arc::new(tokio::sync::Mutex::new(writer_half)),
        user_token: user.token.clone(),
    };

    add_connection(&state, connection.clone()).await;
    send_welcome(&connection, &user).await;

    let mut buffer = [0; READ_BUFFER_SIZE];
    loop {
        let bytes_read = match reader.read(&mut buffer).await {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) => {
                eprintln!("Error reading from client: {}", e);
                break;
            }
        };

        let msg = String::from_utf8_lossy(&buffer[..bytes_read])
            .trim()
            .to_string();

        if let Some(cmd) = command_handling::parse_command(&msg) {
            if let Err(e) = handle_command(cmd, &state, &mut user, &connection).await {
                let _ = connection.send(&format!("Error: {}\n", e)).await;
            }
        } else {
            broadcast_message(&state, &user, &msg, &connection).await;
        }
    }

    remove_user(&state, &user.token).await;
    remove_connection(&state, &connection).await;
}

