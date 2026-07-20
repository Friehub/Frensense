use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0u64));
    let c2 = counter.clone();

    let t1 = tokio::spawn(async move {
        for _ in 0..10000 {
            // SAFE: Mutex provides exclusive access; no concurrent reads/writes.
            let mut val = c2.lock().await;
            *val += 1;
        }
    });

    let t2 = tokio::spawn(async move {
        for _ in 0..10000 {
            let mut val = counter.lock().await;
            *val += 1;
        }
    });

    let _ = tokio::join!(t1, t2);
}
