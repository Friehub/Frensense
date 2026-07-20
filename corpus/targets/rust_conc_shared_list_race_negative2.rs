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
            // SAFE: Snapshot is taken under the lock; iteration is over the clone.
            let snapshot = {
                let guard = data.lock().await;
                guard.clone()
            };
            for val in &snapshot {
                let _ = val + 1;
            }
        }
    });

    let _ = tokio::join!(writer, reader);
}
