use axum::{Router, routing::get, extract::State};
use std::sync::Arc;

struct InnerState {
    data: Vec<u64>,
}

type AppState = Arc<InnerState>;

async fn handler(State(state): State<AppState>) -> String {
    format!("size: {}", state.data.len())
}

#[tokio::main]
async fn main() {
    // SAFE: `Arc` clone is O(1), avoiding expensive deep-clone per request.
    let state = Arc::new(InnerState {
        data: vec![0u64; 1_000_000],
    });
    let app = Router::new()
        .route("/", get(handler))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
