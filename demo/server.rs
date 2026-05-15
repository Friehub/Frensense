use std::sync::Mutex;
use tokio::time::{sleep, Duration};

lazy_static::lazy_static! {
    static ref DB_POOL: Mutex<i32> = Mutex::new(0);
}

pub async fn process_transaction(id: u64) {
    println!("Processing transaction {}...", id);
    
    // VULNERABILITY: Acquiring a synchronous std::sync::Mutex guard...
    let _guard = DB_POOL.lock().unwrap();
    
    // ...and then yielding the thread via .await.
    // This blocks the entire Tokio executor thread, causing system-wide stalls.
    sleep(Duration::from_millis(100)).await; 
    
    println!("Transaction {} complete.", id);
}
