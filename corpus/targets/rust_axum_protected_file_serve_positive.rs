// [frensense]
// observation: Axum's `ServeDir` is configured to serve static files from a directory without an authentication middleware layer. The route for protected files uses `.nest("/files", ServeDir::new("./uploads"))` without wrapping it in an auth guard.
// impact: Anyone can access protected files (user uploads, documents, configs) by navigating to `/files/<filename>`. Sensitive data is exposed without any access control.
// improvement: Wrap the `ServeDir` route with an authentication middleware layer that checks user permissions before serving files.

use axum::Router;
use axum::routing::get;
use tower_http::services::ServeDir;

#[derive(Clone)]
struct AppState;

pub fn app() -> Router {
    Router::new()
        .nest_service("/files", ServeDir::new("./uploads"))
        .with_state(AppState)
}
