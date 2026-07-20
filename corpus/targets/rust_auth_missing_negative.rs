// SAFE: Applies authentication middleware to all routes
use axum::{Router, routing::get, middleware, Json};

async fn admin_panel(user: AuthUser) -> Json<serde_json::Value> {
    if user.role != "admin" {
        return Json(serde_json::json!({"error": "forbidden"}));
    }
    Json(serde_json::json!({"users": [], "config": {}}))
}

async fn user_profile(Path(user_id): Path<String>, user: AuthUser) -> Json<User> {
    if user.id != user_id && user.role != "admin" {
        return Json(serde_json::json!({"error": "forbidden"}));
    }
    Json(User { id: user_id, email: "hidden@example.com".into() })
}

fn make_router() -> Router {
    Router::new()
        .route("/admin", get(admin_panel))
        .route("/profile/:id", get(user_profile))
        .layer(middleware::from_fn(auth_middleware))
}
