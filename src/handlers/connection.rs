use crate::{network::ClientConnection, storage::AppState};
use std::sync::Arc;

/// Adds a new connection to the shared application state.
pub async fn add_connection(state: &AppState, conn: ClientConnection) {
    let mut connections = state.connections.lock().await;
    connections.push(conn);
}
/// Removes a client connection from the shared application state by matching user token.
pub async fn remove_connection(state: &Arc<AppState>, writer: &ClientConnection) {
    let mut connections = state.connections.lock().await;
    connections.retain(|c| c.user_token != writer.user_token);
}
