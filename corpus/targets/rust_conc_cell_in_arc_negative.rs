use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

#[tokio::main]
async fn main() {
    let counter = Arc::new(AtomicU64::new(0));
    let c2 = counter.clone();

    let t1 = tokio::spawn(async move {
        for _ in 0..10000 {
            // SAFE: AtomicU64 is thread-safe; fetch_add is atomic across threads.
            c2.fetch_add(1, Ordering::Relaxed);
        }
    });

    let t2 = tokio::spawn(async move {
        for _ in 0..10000 {
            counter.fetch_add(1, Ordering::Relaxed);
        }
    });

    let _ = tokio::join!(t1, t2);
}
