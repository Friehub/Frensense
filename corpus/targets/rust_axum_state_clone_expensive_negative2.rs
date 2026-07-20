use axum::{Router, routing::get, extract::FromRef};
use std::sync::Arc;

struct Config {
    db_url: String,
}

struct Metrics {
    counter: std::sync::atomic::AtomicU64,
}

#[derive(Clone)]
struct AppState {
    config: Arc<Config>,
    metrics: Arc<Metrics>,
}

impl FromRef<AppState> for Arc<Config> {
    fn from_ref(state: &AppState) -> Self {
        state.config.clone()
    }
}

async fn handler(State(config): State<Arc<Config>>) -> String {
    format!("db: {}", config.db_url)
}

#[tokio::main]
async fn main() {
    let state = AppState {
        config: Arc::new(Config { db_url: "postgres://localhost".into() }),
        metrics: Arc::new(Metrics { counter: std::sync::atomic::AtomicU64::new(0) }),
    };
    let app = Router::new()
        .route("/", get(handler))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
