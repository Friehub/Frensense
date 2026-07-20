// SAFE: Holds the lock for the entire read-modify-write cycle
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

fn increment_counter(map: &Arc<Mutex<HashMap<String, i32>>>, key: &str) {
    let mut m = map.lock().unwrap();
    let val = m.get(key).copied().unwrap_or(0);
    m.insert(key.to_string(), val + 1);
}
