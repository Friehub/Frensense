// SAFE: Uses tokio::sync::RwLock for async-friendly shared state with concurrent reads
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
struct SharedState {
    data: Arc<RwLock<Vec<String>>>,
}

async fn add_item(state: &SharedState, item: String) {
    state.data.write().await.push(item);
}

async fn get_items(state: &SharedState) -> Vec<String> {
    state.data.read().await.clone()
}

async fn process(state: SharedState) {
    add_item(&state, "task1".into()).await;
    let items = get_items(&state).await;
    println!("Items: {:?}", items);
}
