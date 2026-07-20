use axum::{Router, routing::get, extract::State};
use dashmap::DashMap;
use std::sync::Arc;

type Db = Arc<DashMap<String, String>>;

async fn get_value(State(db): State<Db>, key: String) -> String {
    // SAFE: DashMap provides per-shard locking, eliminating the single-point contention bottleneck.
    db.get(&key).map(|v| v.clone()).unwrap_or_default()
}

async fn set_value(State(db): State<Db>, key: String, value: String) {
    db.insert(key, value);
}

#[tokio::main]
async fn main() {
    let db: Db = Arc::new(DashMap::new());
    let app = Router::new()
        .route("/get/{key}", get(get_value))
        .route("/set/{key}/{value}", get(set_value))
        .with_state(db);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
