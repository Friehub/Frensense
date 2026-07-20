use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let data: Arc<RwLock<Vec<u64>>> = Arc::new(RwLock::new(Vec::new()));
    let data_clone = data.clone();

    let writer = tokio::spawn(async move {
        for i in 0..1000u64 {
            let mut guard = data_clone.write().await;
            guard.push(i);
        }
    });

    let reader = tokio::spawn(async move {
        for _ in 0..100 {
            // SAFE: RwLock allows concurrent reads; write lock blocks readers.
            let guard = data.read().await;
            let snapshot: Vec<_> = guard.iter().copied().collect();
            for val in &snapshot {
                let _ = val + 1;
            }
        }
    });

    let _ = tokio::join!(writer, reader);
}
