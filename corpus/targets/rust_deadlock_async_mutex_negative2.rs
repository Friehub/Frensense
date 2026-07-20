// SAFE alternative: restructure to avoid holding lock across .await
use std::sync::Mutex;

async fn process_item(state: &Mutex<Vec<String>>, item: String) {
    // SAFe: drop lock before await
    {
        let mut data = state.lock().unwrap();
        data.push(item);
    }
    some_async_operation().await;
    {
        let mut data = state.lock().unwrap();
        data.push("processed".to_string());
    }
}

async fn some_async_operation() {
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
}
