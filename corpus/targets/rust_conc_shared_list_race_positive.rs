// [frensense]
// observation: An `Arc<Mutex<Vec<T>>>` is shared across concurrent tasks where one task holds the lock and iterates over the vector while another task holds the lock and pushes new elements. The iteration expects a stable snapshot, but concurrent push can invalidate the iteration or cause the iterator to miss/duplicate items.
// impact: Non-deterministic behavior — the iteration may see partially-modified state, skip elements, or panic if the underlying allocation shifts. In production, this can cause inconsistent aggregation results, missed events, or application crashes.
// improvement: Use `tokio::sync::RwLock` for concurrent reads with periodic writes, or use a channel-based architecture where writes are serialized.

use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let data: Arc<Mutex<Vec<u64>>> = Arc::new(Mutex::new(Vec::new()));
    let data_clone = data.clone();

    let writer = tokio::spawn(async move {
        for i in 0..1000u64 {
            let mut guard = data_clone.lock().await;
            guard.push(i);
        }
    });

    let reader = tokio::spawn(async move {
        for _ in 0..100 {
            let guard = data.lock().await;
            for val in guard.iter() {
                let _ = val + 1;
            }
        }
    });

    let _ = tokio::join!(writer, reader);
}
