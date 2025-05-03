use crate::{models::user::User, storage::AppState};
use std::sync::Arc;

/// Registers a new guest user and adds them to the shared user state.
pub async fn register_guest(state: &Arc<AppState>) -> User {
    let mut users = state.users.lock().await;
    users.register_guest().await
}

/// Removes a user from the shared state by their token.
pub async fn remove_user(state: &Arc<AppState>, token: &str) {
    let mut users = state.users.lock().await;
    users.remove_user(token).await;
}
