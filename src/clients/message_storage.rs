use super::user::User;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone)]
pub struct Message {
    pub sender: String, // user token
    pub content: String,
    pub timestamp: i64, // unix timestamp
}

#[derive(Clone)]
pub struct MessageStorage {
    messages: std::collections::HashMap<String, Vec<Message>>, // chat_id: [messages]
}

impl MessageStorage {
    pub fn new() -> Self {
        Self {
            messages: HashMap::new(),
        }
    }

    pub async fn add_message(&mut self, chat_id: &str, sender: &User, content: &str) {
        let message = Message {
            sender: sender.token.clone(),
            content: content.to_string(),
            timestamp: chrono::Utc::now().timestamp(),
        };

        self.messages
            .entry(chat_id.to_string())
            .or_default()
            .push(message);
    }

    pub async fn get_chat_messages(&self, chat_id: &str) -> Vec<Message> {
        self.messages.get(chat_id).cloned().unwrap_or_default()
    }
}
