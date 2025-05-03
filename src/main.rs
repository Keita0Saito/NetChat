mod handlers;
mod models;
mod network;
mod storage;

use crate::{handlers::handle_client, storage::AppState};
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize shared application state.
    let state = Arc::new(AppState::new());

    // Bind TCP listener.
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    // Main loop: handle incoming client connections.
    loop {
        // Accept a new TCP connection.
        let (socket, _) = listener.accept().await?;

        // Split the socket into separete reader and writer halves.
        let (reader, writer) = socket.into_split();

        // Clone shared state to pass into the spawed task.
        let state = Arc::clone(&state);

        // Spawn a new asynchronous task to handle the client independetly.
        tokio::spawn(async move {
            handle_client(reader, writer, state).await;
        });
    }
}
