use crate::{clients::AppState, network::ClientWriter};
use std::sync::Arc;

pub async fn add_connection(state: &Arc<AppState>, writer: &ClientWriter) {
    state.connections.lock().await.push(writer.clone());
}

pub async fn remove_connection(state: &Arc<AppState>, writer: &ClientWriter) {
    let mut connections = state.connections.lock().await;
    connections.retain(|w| !Arc::ptr_eq(w, writer));
}
