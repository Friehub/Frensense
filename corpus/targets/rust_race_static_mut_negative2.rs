// SAFE: Uses tokio::sync::Mutex for async-safe synchronized access; no static mut
use tokio::sync::Mutex;
use std::sync::OnceLock;

static COUNTER: OnceLock<Mutex<u32>> = OnceLock::new();

fn get_counter() -> &'static Mutex<u32> {
    COUNTER.get_or_init(|| Mutex::new(0))
}

async fn increment_counter() {
    let mut c = get_counter().lock().await;
    *c += 1;
}

async fn get_count() -> u32 {
    *get_counter().lock().await
}

async fn process_item(item_id: u32) {
    increment_counter().await;
    println!("Processed item {} (total: {})", item_id, get_count().await);
}
