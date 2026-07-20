use std::sync::Mutex;

async fn inc(counter: &Mutex<u32>) -> u32 {
    let mut guard = counter.lock().unwrap();
    let current = *guard;
    *guard += 1;
    drop(guard);
    // SAFE: Mutex is released before `.await`, no lock held across yield point.
    tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
    current
}

#[tokio::main]
async fn main() {
    let counter = Mutex::new(0u32);
    let (a, b) = tokio::join!(inc(&counter), inc(&counter));
    println!("{a} {b} final={}", counter.lock().unwrap());
}
