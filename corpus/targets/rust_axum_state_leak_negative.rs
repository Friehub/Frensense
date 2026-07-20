// SAFE: Sensitive fields are kept out of the state struct; only non-sensitive data is exposed
use axum::{Router, routing::get, extract::State};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct PublicAppState {
    pub public_hostname: String,
}

pub struct AppSecrets {
    pub db_url: String,
    pub api_key: String,
    pub jwt_secret: String,
}

async fn health_check(State(state): State<PublicAppState>) -> String {
    format!("OK - {}", state.public_hostname)
}

async fn serve() {
    let state = PublicAppState {
        public_hostname: "example.com".into(),
    };
    let app = Router::new()
        .route("/health", get(health_check))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
