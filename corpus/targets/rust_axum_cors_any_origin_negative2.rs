use axum::Router;
use axum::routing::get;
use tower_http::cors::{CorsLayer, AllowOrigin};

#[derive(Clone)]
struct AppState;

pub fn app() -> Router {
    // SAFE: Explicit origin list prevents wildcard bypass with credentials.
    let origins = ["https://app.example.com", "https://admin.example.com"]
        .map(|o| o.parse().unwrap());
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::list(origins))
        .allow_credentials(true);
    Router::new()
        .route("/api/data", get(|| async { "data" }))
        .layer(cors)
        .with_state(AppState)
}
