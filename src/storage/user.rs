use crate::models::user::User;
use std::collections::HashMap;

#[derive(Clone)]
pub struct UserStorage {
    users: HashMap<String, User>,
}

impl UserStorage {
    /// Initializes a new, empty user storage.
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }

    /// Registers a new guest user and stores them.
    pub async fn register_guest(&mut self) -> User {
        let user = User::new_guest();
        self.users.insert(user.token.clone(), user.clone());
        user
    }

    /// Updates a users nickname if the token matches.
    /// Returns the updated user if found.
    pub async fn change_nickname(&mut self, token: &str, new_nick: &str) -> Option<User> {
        if let Some(user) = self.users.get_mut(token) {
            user.nickname = new_nick.to_string();
            Some(user.clone())
        } else {
            None
        }
    }

    /// Removes a user by their token.
    pub async fn remove_user(&mut self, token: &str) {
        self.users.remove(token);
    }

    /// Finds a user by their nickname(case-sensitive) or their token.
    pub async fn find_by_nickname_or_id(&self, identifier: &str) -> Option<User> {
        self.users
            .values()
            .find(|u| u.nickname == identifier || u.token == identifier)
            .cloned()
    }
}
