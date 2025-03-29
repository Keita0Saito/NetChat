mod clients;
mod handlers;
mod network;

use tokio::net::TcpListener;
use crate::clients::AppState;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = Arc::new(AppState::new());
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        let state = Arc::clone(&state);
        
        tokio::spawn(async move {
            let (reader, writer) = socket.into_split();
            let writer = Arc::new(Mutex::new(writer));
            crate::handlers::handle_client(reader, writer, state).await;
        });
    }
}
