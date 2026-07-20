// [frensense]
// observation: A route handler is defined without any authentication middleware or guard, allowing unauthenticated access.
// impact: An unauthenticated attacker can call sensitive endpoints that should require login, such as profile retrieval or data modification.
// improvement: Apply an authentication middleware or extract an AuthUser extractor to the handler.

use axum::{Router, routing::get, Json};

async fn admin_panel() -> Json<serde_json::Value> {
    Json(serde_json::json!({"users": [], "config": {}}))
}

async fn user_profile(user_id: String) -> Json<User> {
    Json(User { id: user_id, email: "leaked@example.com".into() })
}

fn make_router() -> Router {
    Router::new()
        .route("/admin", get(admin_panel))
        .route("/profile/:id", get(user_profile))
}
