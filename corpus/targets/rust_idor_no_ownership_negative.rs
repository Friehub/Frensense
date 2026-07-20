// SAFE: Verifies ownership by adding user_id to the WHERE clause
use axum::{extract::{Path, State}, Json};
use std::sync::Arc;
use sqlx::PgPool;

async fn get_order(
    State(pool): State<Arc<PgPool>>,
    Path(order_id): Path<i32>,
    user_id: i32,
) -> Result<Json<Order>, axum::http::StatusCode> {
    let order = sqlx::query_as::<_, Order>("SELECT * FROM orders WHERE id = $1 AND user_id = $2")
        .bind(order_id)
        .bind(user_id)
        .fetch_optional(&*pool)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(axum::http::StatusCode::NOT_FOUND)?;
    Ok(Json(order))
}
