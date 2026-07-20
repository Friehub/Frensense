// [frensense]
// observation: Middleware is registered after routes using `.route().layer()` instead of before, so requests bypass the middleware on certain routes.
// impact: Authentication, logging, or rate-limiting middleware may not apply to all routes, leaving some endpoints unprotected.
// improvement: Register middleware before route definitions using `Router::new().layer(middleware).route(...)`, or use `RouteLayer` consistently.

use axum::{Router, routing::get, middleware, response::IntoResponse};

async fn sensitive_handler() -> &'static str {
    "sensitive data"
}

fn make_router() -> Router {
    Router::new()
        .route("/sensitive", get(sensitive_handler))
        .layer(middleware::from_fn(auth_middleware))
}

async fn auth_middleware(
    req: axum::extract::Request,
    next: middleware::Next,
) -> impl IntoResponse {
    next.run(req).await
}

async fn serve() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, make_router()).await.unwrap();
}
