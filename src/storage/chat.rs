use crate::models::user::User;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone)]
pub struct ChatStorage {
    chats: HashMap<String, (User, User)>, // key: chat_id, value: (user1, user2)
    user_chats: HashMap<String, Vec<String>>, // key: user_token, value: list of chat_ids
}

impl ChatStorage {
    /// Create a new. empty chat storage.
    pub fn new() -> Self {
        Self {
            chats: HashMap::new(),
            user_chats: HashMap::new(),
        }
    }

    /// Create a chat between two users and registers it in both maps.
    pub async fn create_chat(&mut self, user1: &User, user2: &User) -> String {
        let chat_id = Uuid::new_v4().to_string();

        self.chats
            .insert(chat_id.clone(), (user1.clone(), user2.clone()));

        self.user_chats
            .entry(user1.token.clone())
            .or_default()
            .push(chat_id.clone());

        self.user_chats
            .entry(user2.token.clone())
            .or_default()
            .push(chat_id.clone());

        chat_id
    }

    /// Retrieves a chat by its ID, returning both users if found.
    pub async fn get_chat(&self, chat_id: &str) -> Option<(User, User)> {
        self.chats.get(chat_id).cloned()
    }

    /// Finds existing chat between two users
    pub async fn find_existing_chat(&self, user1_token: &str, user2_token: &str) -> Option<String> {
        self.chats
            .iter()
            .find(|(_, (u1, u2))| {
                (u1.token == user1_token && u2.token == user2_token)
                    || (u1.token == user2_token && u2.token == user1_token)
            })
            .map(|(id, _)| id.clone())
    }
}
