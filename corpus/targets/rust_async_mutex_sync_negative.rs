// SAFE: Uses tokio::sync::Mutex properly with .await in async context
use tokio::sync::Mutex;
use std::sync::Arc;

async fn update_counter(counter: Arc<Mutex<i32>>) -> i32 {
    *counter.lock().await
}
