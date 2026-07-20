// [frensense]
// observation: `&State<UnsyncPool>` is used with a type that does not implement `Send + Sync`, causing undefined behavior or compile errors when Rocket tries to share state across threads.
// impact: Data races, UB, or compilation failure when Rocket's async runtime moves state between threads.
// improvement: Ensure state types implement `Send + Sync`, or use `Arc<Mutex<T>>` / `Arc<RwLock<T>>`.

use rocket::State;
use std::cell::RefCell;

pub struct UnsyncPool {
    pub connections: RefCell<Vec<u32>>,
}

#[get("/pool")]
pub fn pool_info(state: &State<UnsyncPool>) -> String {
    let count = state.connections.borrow().len();
    format!("pool size: {}", count)
}
