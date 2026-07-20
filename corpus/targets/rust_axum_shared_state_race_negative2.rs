use axum::{Router, routing::get, extract::State};
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

type Db = Arc<RwLock<HashMap<String, String>>>;

async fn get_value(State(db): State<Db>, key: String) -> String {
    // SAFE: RwLock allows concurrent reads; only writes are serialized, improving throughput for read-heavy workloads.
    let map = db.read().unwrap();
    map.get(&key).cloned().unwrap_or_default()
}

async fn set_value(State(db): State<Db>, key: String, value: String) {
    let mut map = db.write().unwrap();
    map.insert(key, value);
}

#[tokio::main]
async fn main() {
    let db: Db = Arc::new(RwLock::new(HashMap::new()));
    let app = Router::new()
        .route("/get/{key}", get(get_value))
        .route("/set/{key}/{value}", get(set_value))
        .with_state(db);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
