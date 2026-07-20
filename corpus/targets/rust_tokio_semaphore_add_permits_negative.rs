use tokio::sync::Semaphore;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let sem = Arc::new(Semaphore::new(10));
    let max_permits = 100usize;
    let used = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let mut handles = Vec::new();

    for _ in 0..100 {
        let sem = sem.clone();
        let used = used.clone();
        handles.push(tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            // SAFE: Track total permits used; never exceed max.
            let prev = used.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            if prev + 1 <= max_permits {
                sem.add_permits(1);
            } else {
                used.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
            }
        }));
    }

    for h in handles {
        h.await.ok();
    }
    println!("available: {}", sem.available_permits());
}
