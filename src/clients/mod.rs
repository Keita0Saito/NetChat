mod chat_storage;
mod message_storage;
mod user;
mod user_storage;

pub use chat_storage::ChatStorage;
pub use message_storage::MessageStorage;
pub use user::User;
pub use user_storage::UserStorage;

use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub users: Arc<Mutex<UserStorage>>,
    pub chats: Arc<Mutex<ChatStorage>>,
    pub messages: Arc<Mutex<MessageStorage>>,
    pub connections: Arc<Mutex<Vec<crate::network::ClientConnection>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(UserStorage::new())),
            chats: Arc::new(Mutex::new(ChatStorage::new())),
            messages: Arc::new(Mutex::new(MessageStorage::new())),
            connections: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
