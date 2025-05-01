use super::user::User;
use std::collections::HashMap;

#[derive(Clone)]
pub struct UserStorage {
    users: HashMap<String, User>,
}

impl UserStorage {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }

    pub async fn register_guest(&mut self) -> User {
        let user = User::new_guest();
        self.users.insert(user.token.clone(), user.clone());
        user
    }

    pub async fn change_nickname(&mut self, token: &str, new_nick: &str) -> Option<User> {
        if let Some(user) = self.users.get_mut(token) {
            user.nickname = new_nick.to_string();
            Some(user.clone())
        } else {
            None
        }
    }

    pub async fn remove_user(&mut self, token: &str) {
        self.users.remove(token);
    }

    pub async fn find_by_nickname(&self, nickname: &str) -> Option<User> {
        self.users
            .values()
            .find(|u| u.nickname == nickname)
            .cloned()
    }
}
