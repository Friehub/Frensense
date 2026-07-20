// SAFE: Uses a custom AuthGuard extractor that validates the session token
use axum::{Router, routing::get, extract::FromRequestParts, Json};

async fn admin_panel(user: AuthUser) -> Json<serde_json::Value> {
    Json(serde_json::json!({"users": [], "config": {}}))
}

async fn user_profile(user: AuthUser, Path(user_id): Path<String>) -> Json<User> {
    if user.id != user_id {
        return Json(serde_json::json!({"error": "forbidden"}));
    }
    Json(User { id: user_id, email: "private@example.com".into() })
}

fn make_router() -> Router {
    Router::new()
        .route("/admin", get(admin_panel))
        .route("/profile/:id", get(user_profile))
}
