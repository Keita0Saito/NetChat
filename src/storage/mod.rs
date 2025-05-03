pub mod chat;
pub mod message;
pub mod user;

use crate::network::ClientConnection;
pub use chat::ChatStorage;
pub use message::MessageStorage;
pub use user::UserStorage;

use std::sync::Arc;
use tokio::sync::Mutex;

/// Main application state that holds all essential data stuctures.
#[derive(Clone)]
pub struct AppState {
    pub users: Arc<Mutex<UserStorage>>,
    pub chats: Arc<Mutex<ChatStorage>>,
    pub messages: Arc<Mutex<MessageStorage>>,
    pub connections: Arc<Mutex<Vec<ClientConnection>>>,
}

impl AppState {
    /// Initializes a new AppState instance with fresh storages for users, chats, messages, and connections.
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(UserStorage::new())),
            chats: Arc::new(Mutex::new(ChatStorage::new())),
            messages: Arc::new(Mutex::new(MessageStorage::new())),
            connections: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
