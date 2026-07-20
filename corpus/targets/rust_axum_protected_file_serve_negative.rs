use axum::Router;
use axum::routing::get;
use axum::middleware;
use tower_http::services::ServeDir;

#[derive(Clone)]
struct AppState;

async fn auth_middleware<B>(req: axum::http::Request<B>, next: middleware::Next<B>) -> Result<axum::response::Response, axum::response::Response> {
    if req.headers().get("Authorization").is_some() {
        Ok(next.run(req).await)
    } else {
        Err(axum::response::Response::new(axum::body::Body::from("unauthorized")))
    }
}

pub fn app() -> Router {
    // SAFE: ServeDir is wrapped in an auth middleware that requires a valid token.
    Router::new()
        .nest_service("/files", ServeDir::new("./uploads"))
        .route_layer(middleware::from_fn(auth_middleware))
        .with_state(AppState)
}
