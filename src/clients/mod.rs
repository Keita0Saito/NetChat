mod user;
pub use user::{UserManager};

use std::sync::Arc;
use tokio::{
    net::tcp::OwnedWriteHalf, 
    sync::Mutex
};

pub type ClientWriter = Arc<Mutex<OwnedWriteHalf>>;
pub type ClientList = Arc<Mutex<Vec<ClientWriter>>>;

pub struct AppState {
    pub connections: ClientList,
    pub users: UserManager,
}
