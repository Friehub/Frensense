// SAFE: use tokio::sync::Mutex for async contexts
use tokio::sync::Mutex;

async fn process_item(state: &Mutex<Vec<String>>, item: String) {
    let mut data = state.lock().await;
    data.push(item);
    some_async_operation().await;
    data.push("processed".to_string());
}

async fn some_async_operation() {
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
}
