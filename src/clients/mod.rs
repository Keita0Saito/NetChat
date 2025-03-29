mod user;
mod user_storage;

pub use user::User;
pub use user_storage::UserStorage;

use tokio::sync::Mutex;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub users: Arc<Mutex<UserStorage>>,
    pub connections: Arc<Mutex<Vec<super::network::ClientWriter>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(UserStorage::new())),
            connections: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
