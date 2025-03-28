use tokio::net::{TcpListener};
use tokio::sync::Mutex;
use std::sync::Arc;
use tokio::io::{AsyncWriteExt, AsyncReadExt};

#[tokio::main]
async fn main() {
    // Bind TCP listener to address
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    // Shared list of connected clients (wrapped in Arc<Mutex> for thread safety)
    let clients = Arc::new(Mutex::new(Vec::new()));

    loop {
        // Accept incoming connections
        let (stream, _) = listener.accept().await.unwrap();
        let clients = clients.clone();

        // Spawn async task to handle client
        tokio::spawn(async move {
            // Split stream into read/write halves
            let (mut reader, writer) = stream.into_split();
            let writer = Arc::new(Mutex::new(writer));

            // Add client's writer to shared list
            clients.lock().await.push(writer.clone());

            // Buffer for reading messages
            let mut buf = [0; 1024];
            loop {
                // Read data from client
                let n = match reader.read(&mut buf).await {
                    Ok(n) if n == 0 => break,  // Client disconnected
                    Ok(n) => n,
                    Err(_) => break,  // Read error
                };

                // Broadcast message to all clients except the sender
                let msg = &buf[..n];
                let mut clients = clients.lock().await;

                // Collect clients that are alive (filter out dead ones)
                let mut alive_clients = Vec::new();
                let mut i = 0;
                while i < clients.len() {
                    let client = &clients[i];

                    // Skip the sender client by comparing Arc directly
                    if Arc::ptr_eq(client, &writer) {
                        i += 1;
                        continue;
                    }

                    // Lock the client and try to send the message
                    let mut client = client.lock().await;
                    match client.write_all(msg).await {
                        Ok(_) => {
                            alive_clients.push(clients[i].clone());
                            i += 1;  // Success: move to next client
                        }
                        Err(_) => {
                            // Failed to send, skip this client (will be removed)
                            i += 1;  // Still move to the next
                        }
                    }
                }

                // Update the client list to include only the alive clients
                *clients = alive_clients;
            }

            // Cleanup: remove client from list (this happens automatically after broadcast loop)
        });
    }
}
