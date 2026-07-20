// SAFE: Uses a separate buffer and swaps after write lock is dropped.
use std::sync::RwLock;

pub fn process_data(data: &RwLock<Vec<u8>>) -> Vec<u8> {
    let mut new_buf = Vec::new();
    {
        let mut write = data.write().unwrap();
        write.push(42);
        new_buf = write.clone();
    }
    new_buf
}
