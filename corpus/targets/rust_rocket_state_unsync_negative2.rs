// SAFE: Wraps state in Arc<RwLock<T>> which is Send + Sync.
use rocket::State;
use std::sync::{Arc, RwLock};

pub struct SharedPool {
    pub connections: Arc<RwLock<Vec<u32>>>,
}

#[get("/pool")]
pub fn pool_info(state: &State<SharedPool>) -> String {
    let count = state.connections.read().unwrap().len();
    format!("pool size: {}", count)
}
