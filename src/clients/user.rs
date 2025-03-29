use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc
    }
};
use tokio::sync::RwLock;
use uuid::Uuid;

// User data structure
#[derive(Debug, Clone)]
pub struct User {
    pub nickname: String,  // Current display name
    pub token: String,     // Unique session identifier
}

// Manages user authentication and sessions
pub struct UserManager {
    pub users: Arc<RwLock<HashMap<String, User>>>,  // token -> User mapping
    pub guest_counter: AtomicUsize,                 // Auto-increment for guest names
}

impl UserManager {
    // Create new UserManager with empty state
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
            guest_counter: AtomicUsize::new(1),
        }
    }

    // Register new guest user with auto-generated name
    pub async fn register_guest(&self) -> User {
        let num = self.guest_counter.fetch_add(1, Ordering::SeqCst);
        let token = Uuid::new_v4().to_string();
        
        let user = User {
            nickname: format!("Guest-{}", num),
            token: token.clone(),
        };
        
        self.users.write().await.insert(token, user.clone());
        user
    }

    // Update nickname for existing user
    pub async fn change_nickname(&self, token: &str, new_nick: &str) -> Option<User> {
        let mut users = self.users.write().await;
        users.get_mut(token).map(|user| {
            user.nickname = new_nick.to_string();
            user.clone()
        })
    }

    // Remove a user
    pub async fn remove_user(&self, token: &str) {
        self.users.write().await.remove(token);
    }
}
