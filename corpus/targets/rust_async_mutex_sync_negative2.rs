// SAFE: Uses std::sync::Mutex for synchronous operations, tokio::sync::Mutex for async
use std::sync::{Arc, Mutex};

fn sync_update(counter: Arc<Mutex<Vec<u8>>>) {
    let mut guard = counter.lock().unwrap();
    guard.push(42);
}
