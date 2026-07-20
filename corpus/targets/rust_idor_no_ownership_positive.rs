// [frensense]
// observation: An Axum or Actix-web handler fetches a resource using an ID from the request without verifying the current user owns that resource.
// impact: An attacker can access, modify, or delete resources belonging to other users by changing the resource ID parameter.
// improvement: Before returning or modifying the resource, verify that the authenticated user's ID matches the owner_id field.

use axum::{extract::{Path, State}, Json};
use std::sync::Arc;
use sqlx::PgPool;

async fn get_order(
    State(pool): State<Arc<PgPool>>,
    Path(order_id): Path<i32>,
) -> Result<Json<Order>, axum::http::StatusCode> {
    let order = sqlx::query_as::<_, Order>("SELECT * FROM orders WHERE id = $1")
        .bind(order_id)
        .fetch_optional(&*pool)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(axum::http::StatusCode::NOT_FOUND)?;
    Ok(Json(order))
}
