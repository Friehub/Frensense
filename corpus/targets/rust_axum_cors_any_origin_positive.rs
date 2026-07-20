// [frensense]
// observation: Axum's CORS layer is configured with `.allow_origin(AllowOrigin::any())` combined with `.allow_credentials(true)`. The `any()` origin produces a wildcard `Access-Control-Allow-Origin: *` header, but when `allow_credentials` is true, browsers require a specific origin — yet Axum serves the request's own origin reflected back.
// impact: Any website can make authenticated cross-origin requests to this API. Since credentials (cookies, auth headers) are allowed and the origin is dynamically reflected, this effectively bypasses CORS protections, enabling CSRF and data theft.
// improvement: Use `.allow_origin(AllowOrigin::exact("https://app.example.com"))` with specific origins; never use `any()` when credentials are enabled.

use axum::Router;
use axum::routing::get;
use tower_http::cors::{CorsLayer, AllowOrigin};

#[derive(Clone)]
struct AppState;

pub fn app() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::any())
        .allow_credentials(true);
    Router::new()
        .route("/api/data", get(|| async { "data" }))
        .layer(cors)
        .with_state(AppState)
}
