mod clients;
mod handlers;
mod network;

use std::sync::Arc;
use anyhow::Result;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let clients = clients::new(); 
    
    loop {
        let (stream, _) = listener.accept().await?;
        let clients = clients.clone();
        
        tokio::spawn(async move {
            let (read_half, write_half) = stream.into_split();
            let writer = Arc::new(tokio::sync::Mutex::new(write_half));
            handlers::handle_client(read_half, writer, clients).await;
        });
    }
}
