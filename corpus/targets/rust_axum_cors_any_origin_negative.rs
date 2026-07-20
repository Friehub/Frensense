use axum::Router;
use axum::routing::get;
use tower_http::cors::{CorsLayer, AllowOrigin, Any};

#[derive(Clone)]
struct AppState;

pub fn app() -> Router {
    // SAFE: CORS restricted to single trusted origin; credentials only sent to that origin.
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::exact("https://app.example.com".parse().unwrap()))
        .allow_credentials(true);
    Router::new()
        .route("/api/data", get(|| async { "data" }))
        .layer(cors)
        .with_state(AppState)
}
