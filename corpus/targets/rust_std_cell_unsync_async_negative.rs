use std::sync::atomic::{AtomicU32, Ordering};

async fn inc(counter: &AtomicU32) -> u32 {
    // SAFE: `AtomicU32` provides atomic read-modify-write, safe across `.await`.
    let current = counter.fetch_add(1, Ordering::SeqCst);
    tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
    current
}

#[tokio::main]
async fn main() {
    let counter = AtomicU32::new(0);
    let (a, b) = tokio::join!(inc(&counter), inc(&counter));
    println!("{a} {b} final={}", counter.load(Ordering::SeqCst));
}
