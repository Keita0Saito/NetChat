use crate::{clients::AppState, network::ClientConnection};
use std::sync::Arc;

pub async fn add_connection(state: &AppState, conn: ClientConnection) {
    let mut connections = state.connections.lock().await;
    connections.push(conn);
}

pub async fn remove_connection(state: &Arc<AppState>, writer: &ClientConnection) {
    let mut connections = state.connections.lock().await;
    connections.retain(|c| c.user_token != writer.user_token);
}
