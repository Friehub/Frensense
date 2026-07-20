use tokio::sync::Semaphore;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let sem = Arc::new(Semaphore::new(10));
    let mut handles = Vec::new();

    // SAFE: Permits are only returned, never added beyond the initial capacity.
    for _ in 0..10 {
        let sem = sem.clone();
        handles.push(tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }));
    }

    for h in handles {
        h.await.ok();
    }
    println!("available: {}", sem.available_permits());
}
