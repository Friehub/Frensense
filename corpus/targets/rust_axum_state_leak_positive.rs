// [frensense]
// observation: Sensitive data like database credentials, API keys, or internal tokens stored in shared application state are exposed through a handler that serializes the entire state.
// impact: An attacker can read sensitive configuration or secrets by hitting the endpoint that leaks the state object.
// improvement: Create a response DTO that only includes non-sensitive fields, or avoid putting secrets in shared state.

use axum::{Router, routing::get, extract::State};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct AppState {
    pub db_url: String,
    pub api_key: String,
    pub jwt_secret: String,
    pub public_hostname: String,
}

async fn health_check(State(state): State<AppState>) -> String {
    format!("OK - {}", state.public_hostname)
}

async fn debug_state(State(state): State<AppState>) -> axum::Json<AppState> {
    axum::Json(state)
}

async fn serve() {
    let state = AppState {
        db_url: "postgres://admin:hunter2@db.internal:5432/prod".into(),
        api_key: "sk-live-abcdef123456".into(),
        jwt_secret: "supersecretkey".into(),
        public_hostname: "example.com".into(),
    };
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/debug", get(debug_state))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
