// [frensense]
// observation: An `Arc<Mutex<HashMap>>` is used as shared state with the entire map locked on every request, even for fine-grained operations. This serializes all concurrent access and creates a performance bottleneck that can amplify contention under load.
// impact: Under high concurrency, the single mutex becomes a contention point, drastically reducing throughput. An attacker can exploit this for a slowloris-style degradation attack by issuing many concurrent requests that hold the lock for extended periods.
// improvement: Use a sharded approach (`dashmap::DashMap`), `RwLock` for read-heavy workloads, or fine-grained per-key locking.

use axum::{Router, routing::get, extract::State};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

type Db = Arc<Mutex<HashMap<String, String>>>;

async fn get_value(State(db): State<Db>, key: String) -> String {
    let map = db.lock().unwrap();
    map.get(&key).cloned().unwrap_or_default()
}

async fn set_value(State(db): State<Db>, key: String, value: String) {
    let mut map = db.lock().unwrap();
    map.insert(key, value);
}

#[tokio::main]
async fn main() {
    let db: Db = Arc::new(Mutex::new(HashMap::new()));
    let app = Router::new()
        .route("/get/{key}", get(get_value))
        .route("/set/{key}/{value}", get(set_value))
        .with_state(db);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
