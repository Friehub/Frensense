// SAFE: Middleware is applied before routes using layer() at the top level
use axum::{Router, routing::get, middleware, response::IntoResponse};

async fn sensitive_handler() -> &'static str {
    "sensitive data"
}

fn make_router() -> Router {
    Router::new()
        .layer(middleware::from_fn(auth_middleware))
        .route("/sensitive", get(sensitive_handler))
}

async fn auth_middleware(
    req: axum::extract::Request,
    next: middleware::Next,
) -> impl IntoResponse {
    let auth_header = req.headers().get("Authorization");
    if auth_header.is_none() {
        return axum::response::Response::new(
            axum::body::Body::from("Unauthorized"),
        );
    }
    next.run(req).await
}

async fn serve() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, make_router()).await.unwrap();
}
