use crate::models::{message::Message, user::User};
use std::collections::HashMap;

#[derive(Clone)]
pub struct MessageStorage {
    messages: HashMap<String, Vec<Message>>, // chat_id: [messages]
}

impl MessageStorage {
    /// Initializes a new, empty message storage.
    pub fn new() -> Self {
        Self {
            messages: HashMap::new(),
        }
    }

    /// Adds a new message to the storage for a given chat.
    pub async fn add_message(&mut self, chat_id: &str, sender: &User, content: &str) {
        // Create a new message from the sender, content and the current time.
        let message = Message::new(&sender.token, content);

        self.messages
            .entry(chat_id.to_string())
            .or_default()
            .push(message);
    }
}
