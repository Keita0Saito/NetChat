mod clients;
mod handlers;
mod network;

use std::sync::Arc;
use anyhow::Result;
use tokio::net::TcpListener;
use clients::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let state = Arc::new(AppState {
        connections: clients::ClientList::default(),
        users: clients::UserManager::new(),
    });

    loop {
        let (stream, _) = listener.accept().await?;
        let state = state.clone();

        tokio::spawn(async move {
            let (reader, writer) = stream.into_split();
            let writer = Arc::new(tokio::sync::Mutex::new(writer));
            handlers::handle_client(reader, writer, state).await;
        });
    }
}
