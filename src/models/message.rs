#[allow(dead_code)]
#[derive(Clone)]
pub struct Message {
    sender: String, // user token
    content: String,
    timestamp: i64, // unix timestamp
}

impl Message {
    pub fn new(sender: &str, content: &str) -> Self {
        Self {
            sender: sender.to_string(),
            content: content.to_string(),
            timestamp: chrono::Utc::now().timestamp(),
        }
    }
}
