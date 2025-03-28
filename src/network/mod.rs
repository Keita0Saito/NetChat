use tokio::io::AsyncWriteExt;
use super::clients::ClientWriter;

pub async fn broadcast(
    message_bytes: &[u8],
    connected_clients: &mut Vec<ClientWriter>,
) {
    let mut i = 0;
    while i < connected_clients.len() {
        let client = connected_clients[i].clone();
        let mut locked_writer = client.lock().await;
        
        match locked_writer.write_all(message_bytes).await {
            Ok(_) => i += 1,
            Err(_) => {
                connected_clients.remove(i);
                // Don't increment i since we removed an element
            }
        }
    }
}
