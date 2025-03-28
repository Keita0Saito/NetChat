use tokio::{net::tcp::OwnedReadHalf, io::AsyncReadExt};
use std::sync::Arc;
use super::{clients::*, network::broadcast};

const READ_BUFFER_SIZE: usize = 1024;

pub async fn handle_client(
    mut reader: OwnedReadHalf,
    writer: ClientWriter,
    clients: ClientList,
) {
    // Add client to list
    {
        let mut connected_clients = clients.lock().await;
        connected_clients.push(writer.clone());
    }

    // Read loop
    let mut read_buffer = [0; READ_BUFFER_SIZE];
    loop {
        let bytes_read = match reader.read(&mut read_buffer).await {
            Ok(0) => break,  // Clean disconnect
            Ok(n) => n,
            Err(_) => break, // Error case
        };

        let message_bytes = &read_buffer[..bytes_read];
        let mut connected_clients = clients.lock().await;
        broadcast(message_bytes, &mut connected_clients).await;
    }

    // Remove client on disconnect
    let mut connected_clients = clients.lock().await;
    connected_clients.retain(|client| !Arc::ptr_eq(client, &writer));
}
