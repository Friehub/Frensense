// SAFE: Uses Arc<Mutex<T>> for thread-safe shared mutable state; properly synchronized for concurrent access
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct SharedState {
    data: Arc<Mutex<Vec<String>>>,
}

fn add_item(state: &SharedState, item: String) {
    state.data.lock().unwrap().push(item);
}

fn get_items(state: &SharedState) -> Vec<String> {
    state.data.lock().unwrap().clone()
}

fn process(state: SharedState) {
    add_item(&state, "task1".into());
    let items = get_items(&state);
    println!("Items: {:?}", items);
}
