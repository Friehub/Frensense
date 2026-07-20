use rocket::State;
use std::sync::Mutex;

pub struct SyncPool {
    pub connections: Mutex<Vec<u32>>,
}

#[get("/pool")]
pub fn pool_info(state: &State<SyncPool>) -> String {
    let count = state.connections.lock().unwrap().len();
    format!("pool size: {}", count)
}
