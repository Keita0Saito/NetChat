use tokio::{net::tcp::OwnedWriteHalf, sync::Mutex};
use std::sync::Arc;

pub type ClientWriter = Arc<Mutex<OwnedWriteHalf>>;
pub type ClientList = Arc<Mutex<Vec<ClientWriter>>>;

pub fn new() -> ClientList {
    Arc::new(Mutex::new(Vec::new()))
}
