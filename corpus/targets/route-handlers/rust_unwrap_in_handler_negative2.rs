// SAFE alternative: pattern match for explicit error handling
async fn get_user_handler(user_id: &str, pool: &sqlx::PgPool) -> impl axum::response::IntoResponse {
    match sqlx::query!("SELECT * FROM users WHERE id = $1", user_id)
        .fetch_optional(pool)
        .await
    {
        Ok(Some(user)) => (axum::http::StatusCode::OK, axum::Json(user)).into_response(),
        Ok(None) => (axum::http::StatusCode::NOT_FOUND, "not found").into_response(),
        Err(e) => {
            tracing::error!("db error: {e}");
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "internal error").into_response()
        }
    }
}
