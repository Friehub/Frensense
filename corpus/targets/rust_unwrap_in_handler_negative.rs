// SAFE: use ? operator to propagate errors
async fn get_user_handler(user_id: &str, pool: &sqlx::PgPool) -> Result<axum::Json<User>, axum::http::StatusCode> {
    let user = sqlx::query!("SELECT * FROM users WHERE id = $1", user_id)
        .fetch_optional(pool)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(axum::http::StatusCode::NOT_FOUND)?;
    Ok(axum::Json(user))
}

async fn delete_item_handler(item_id: i32, db: &sqlx::PgPool) -> Result<(), (axum::http::StatusCode, &'static str)> {
    sqlx::query!("DELETE FROM items WHERE id = $1", item_id)
        .execute(db)
        .await
        .map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "db error"))?;
    Ok(())
}
