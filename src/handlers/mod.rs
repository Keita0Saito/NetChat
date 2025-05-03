mod commands;
mod connection;
mod messages;
mod user_management;

pub use commands::handle_command;
pub use connection::{add_connection, remove_connection};
pub use messages::{broadcast_message, send_welcome};
pub use user_management::{register_guest, remove_user};

use crate::{network::ClientConnection, storage::AppState};
use std::sync::Arc;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::{io::AsyncReadExt, net::tcp::OwnedReadHalf};

const READ_BUFFER_SIZE: usize = 1024; // Buffer size for incoming data.

/// Handles a single client connection lifecycle.
pub async fn handle_client(
    mut reader: OwnedReadHalf,
    writer_half: OwnedWriteHalf,
    state: Arc<AppState>,
) {
    // Register the new client as a guest user.
    let mut user = register_guest(&state).await;

    // Wrap the writer in the mutex for concurrent write access.
    let connection = ClientConnection {
        writer: Arc::new(tokio::sync::Mutex::new(writer_half)),
        user_token: user.token.clone(),
    };

    // Add the connection to global state.
    add_connection(&state, connection.clone()).await;
    // Send welcome to the newly conneced client.
    send_welcome(&connection, &user).await;

    // Create buffer for reading incomming message.
    let mut buffer = [0; READ_BUFFER_SIZE];

    loop {
        // Read data from client into buffer.
        let bytes_read = match reader.read(&mut buffer).await {
            Ok(0) => break, // Client disconnected.
            Ok(n) => n,     // Received n bytes of data.
            Err(e) => {
                // Log and break on read error.
                eprintln!("Error reading from client: {}", e);
                break;
            }
        };

        // Convert raw bytes into UTF-8 string.
        let msg = String::from_utf8_lossy(&buffer[..bytes_read])
            .trim()
            .to_string();

        // Try to parse input as a command.
        if let Some(cmd) = commands::parse_command(&msg) {
            // Execute the command and handle potencial errors.
            if let Err(e) = handle_command(cmd, &state, &mut user, &connection).await {
                let _ = connection.send(&format!("Error: {}\n", e)).await;
            }
        } else {
            // If not a command, treat as public message and broadcast.
            broadcast_message(&state, &user, &msg, &connection).await;
        }
    }

    // Cleanup: remove user and connection from shared state.
    remove_user(&state, &user.token).await;
    remove_connection(&state, &connection).await;
}
