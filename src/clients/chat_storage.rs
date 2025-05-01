use super::user::User;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone)]
pub struct ChatStorage {
    chats: HashMap<String, (User, User)>, // key: chat_id, value: (user1, user2)
    user_chats: HashMap<String, Vec<String>>, // key: user_token, value: list of chat_ids
}

impl ChatStorage {
    pub fn new() -> Self {
        Self {
            chats: HashMap::new(),
            user_chats: HashMap::new(),
        }
    }

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

    pub async fn get_chat(&self, chat_id: &str) -> Option<(User, User)> {
        self.chats.get(chat_id).cloned()
    }

    pub async fn get_user_chats(&self, user_token: &str) -> Vec<String> {
        self.user_chats.get(user_token).cloned().unwrap_or_default()
    }
}
