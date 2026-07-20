// SAFE: Uses entry API for atomic read-modify-write
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

fn increment_counter(map: &Arc<Mutex<HashMap<String, i32>>>, key: &str) {
    let mut m = map.lock().unwrap();
    let counter = m.entry(key.to_string()).or_insert(0);
    *counter += 1;
}

fn update_stats(map: &Arc<Mutex<HashMap<String, Vec<u64>>>>, user: &str, latency: u64) {
    let mut m = map.lock().unwrap();
    m.entry(user.to_string()).or_default().push(latency);
}
