mod clients;
mod handlers;
mod network;

use crate::{clients::AppState, handlers::handle_client, network::ClientConnection};
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = Arc::new(AppState::new());
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        let (reader, writer) = socket.into_split();
        let state = Arc::clone(&state);

        tokio::spawn(async move {
            handle_client(reader, writer, state).await;
        });
    }
}
