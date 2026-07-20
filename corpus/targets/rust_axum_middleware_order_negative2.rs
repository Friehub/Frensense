// SAFE: Uses a routed-scoped middleware with `.route_layer()` for precise route coverage
use axum::{Router, routing::get, middleware, response::IntoResponse};

async fn sensitive_handler() -> &'static str {
    "sensitive data"
}

async fn public_handler() -> &'static str {
    "public info"
}

async fn auth_middleware(
    req: axum::extract::Request,
    next: middleware::Next,
) -> impl IntoResponse {
    if req.headers().get("Authorization").is_none() {
        return axum::response::Response::new(
            axum::body::Body::from("Unauthorized"),
        );
    }
    next.run(req).await
}

fn make_router() -> Router {
    Router::new()
        .route("/public", get(public_handler))
        .route("/sensitive", get(sensitive_handler))
        .route_layer(middleware::from_fn(auth_middleware))
}

async fn serve() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, make_router()).await.unwrap();
}
