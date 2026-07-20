// [frensense]
// observation: A `RwLock` write lock is held while attempting to acquire a read lock in the same scope, causing a deadlock on single-threaded runtimes (e.g., tokio current-thread) or contention.
// impact: The application hangs forever when the code path is exercised, leading to denial of service.
// improvement: Restructure to release the write lock before acquiring the read lock, or use a different synchronization primitive.

use std::sync::RwLock;

pub fn process_data(data: &RwLock<Vec<u8>>) -> Vec<u8> {
    let mut write = data.write().unwrap();
    write.push(42);
    let read = data.read().unwrap();
    read.clone()
}
