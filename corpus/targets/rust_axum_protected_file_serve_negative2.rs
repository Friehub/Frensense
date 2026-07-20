use axum::Router;
use axum::routing::get;
use tower_http::services::ServeDir;
use tower_http::validate_request::ValidateRequestHeaderLayer;

#[derive(Clone)]
struct AppState;

pub fn app() -> Router {
    // SAFE: `ValidateRequestHeaderLayer` rejects requests without a bearer token before files are served.
    let auth = ValidateRequestHeaderLayer::bearer("secret-token");
    Router::new()
        .nest_service("/files", ServeDir::new("./uploads"))
        .route_layer(auth)
        .with_state(AppState)
}
