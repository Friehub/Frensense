// [frensense]
// observation: Axum state type implements `Clone` with an expensive operation (e.g., deep-cloning a large configuration, re-establishing a connection pool clone). Axum clones state for each request handler by default.
// impact: Every HTTP request pays the cost of cloning the entire state. Under load, the CPU and memory overhead can cause severe latency spikes and throughput degradation. An attacker can amplify this by issuing many concurrent requests.
// improvement: Wrap state in `Arc` to share it cheaply via reference-counted clone, or use `FromRef` with a layered state pattern.

use axum::{Router, routing::get, extract::State};

#[derive(Clone)]
struct AppState {
    data: Vec<u64>,
}

async fn handler(State(state): State<AppState>) -> String {
    format!("size: {}", state.data.len())
}

#[tokio::main]
async fn main() {
    let state = AppState {
        data: vec![0u64; 1_000_000],
    };
    let app = Router::new()
        .route("/", get(handler))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
