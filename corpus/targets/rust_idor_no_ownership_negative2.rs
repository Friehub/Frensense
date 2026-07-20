// SAFE: Fetches the resource first, then checks ownership after retrieval
use axum::{extract::{Path, State}, Json};
use std::sync::Arc;
use sqlx::PgPool;

async fn get_order(
    State(pool): State<Arc<PgPool>>,
    Path(order_id): Path<i32>,
    auth_user: AuthUser,
) -> Result<Json<Order>, axum::http::StatusCode> {
    let order = sqlx::query_as::<_, Order>("SELECT * FROM orders WHERE id = $1")
        .bind(order_id)
        .fetch_optional(&*pool)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(axum::http::StatusCode::NOT_FOUND)?;
    if order.user_id != auth_user.id {
        return Err(axum::http::StatusCode::FORBIDDEN);
    }
    Ok(Json(order))
}
