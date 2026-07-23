// [frensense]
// observation: .unwrap() or .expect() called on Result or Option inside a request handler function.
// impact: If the Result is Err or Option is None, unwrap panics, which crashes the handler thread. In a web server this causes a 500 error or connection reset, leading to denial of service.
// improvement: Use pattern matching, ? operator, or .ok_or() to handle errors gracefully and return appropriate HTTP error codes.

async fn get_user_handler(user_id: &str, pool: &sqlx::PgPool) -> impl axum::response::IntoResponse {
    // VULNERABLE: panics if user not found or DB error
    let user = sqlx::query!("SELECT * FROM users WHERE id = $1", user_id)
        .fetch_one(pool)
        .await
        .unwrap();
    axum::Json(user)
}

async fn delete_item_handler(item_id: i32, db: &sqlx::PgPool) {
    // VULNERABLE: panics on DB constraint violation
    sqlx::query!("DELETE FROM items WHERE id = $1", item_id)
        .execute(db)
        .await
        .expect("failed to delete item");
}
