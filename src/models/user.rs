use rand::Rng;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct User {
    pub token: String,
    pub nickname: String,
}

impl User {
    pub fn new_guest() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            token: Uuid::new_v4().to_string(),
            nickname: format!("Guest{:04}", rng.gen_range(0..10000)),
        }
    }
}
