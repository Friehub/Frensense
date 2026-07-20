// SAFE: Uses std::sync::atomic::AtomicU32 for thread-safe counter; no unsafe code needed
use std::sync::atomic::{AtomicU32, Ordering};

static COUNTER: AtomicU32 = AtomicU32::new(0);

fn increment_counter() {
    COUNTER.fetch_add(1, Ordering::Relaxed);
}

fn get_counter() -> u32 {
    COUNTER.load(Ordering::Relaxed)
}

fn process_item(item_id: u32) {
    increment_counter();
    println!("Processed item {} (total: {})", item_id, get_counter());
}
