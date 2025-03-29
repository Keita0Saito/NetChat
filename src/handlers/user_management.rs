use crate::clients::{AppState, User};
use std::sync::Arc;

pub async fn register_guest(state: &Arc<AppState>) -> User {
    let mut users = state.users.lock().await;
    users.register_guest().await
}

pub async fn remove_user(state: &Arc<AppState>, token: &str) {
    let mut users = state.users.lock().await;
    users.remove_user(token).await;
}

//pub async fn change_nickname(
//    state: &Arc<AppState>,
//    token: &str,
//    new_nickname: &str,
//) -> Option<User> {
//    state.users.change_nickname(token, new_nickname).await
//}
