// [frensense]
// observation: std::sync::Mutex guard held across an .await point in async code.
// impact: std::sync::Mutex is not designed for async. Holding the guard across .await keeps the thread locked for the entire await duration, potentially blocking the entire async runtime and causing deadlocks if the same mutex is needed by other tasks.
// improvement: Use tokio::sync::Mutex for cross-await locking, or restructure to avoid holding locks across await points.

use std::sync::Mutex;

async fn process_item(state: &Mutex<Vec<String>>, item: String) {
    // VULNERABLE: MutexGuard held across .await
    let mut data = state.lock().unwrap();
    data.push(item);
    some_async_operation().await;
    data.push("processed".to_string());
}

async fn some_async_operation() {
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
}
