// [frensense]
// observation: A shared HashMap protected by Arc<Mutex> is read and written without proper locking discipline, or locks are held for shorter than necessary, leading to data races.
// impact: Concurrent access to the HashMap can cause torn reads, lost updates, or panics from the HashMap's internal invariants being violated.
// improvement: Always acquire the Mutex lock before reading or writing, and hold it for the entire read-modify-write cycle.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

fn increment_counter(map: &Arc<Mutex<HashMap<String, i32>>>, key: &str) {
    let m = map.lock().unwrap();
    let val = m.get(key).copied().unwrap_or(0);
    drop(m);
    let new_val = val + 1;
    let mut m = map.lock().unwrap();
    m.insert(key.to_string(), new_val);
}

fn update_stats(map: &Arc<Mutex<HashMap<String, Vec<u64>>>>, user: &str, latency: u64) {
    let m = map.lock().unwrap();
    let mut entries = m.get(user).cloned().unwrap_or_default();
    drop(m);
    entries.push(latency);
    let mut m = map.lock().unwrap();
    m.insert(user.to_string(), entries);
}
