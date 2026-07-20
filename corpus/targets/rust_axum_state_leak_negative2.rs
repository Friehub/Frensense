// SAFE: The debug handler explicitly maps to a response DTO and omits secrets
use axum::{Router, routing::get, extract::State, Json};
use serde::Serialize;

#[derive(Clone)]
pub struct AppState {
    pub db_url: String,
    pub api_key: String,
    pub jwt_secret: String,
    pub public_hostname: String,
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    hostname: String,
}

async fn health_check(State(state): State<AppState>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".into(),
        hostname: state.public_hostname,
    })
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
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
