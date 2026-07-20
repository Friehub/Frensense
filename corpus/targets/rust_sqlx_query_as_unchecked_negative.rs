use sqlx::FromRow;

#[derive(FromRow)]
struct User {
    id: i64,
    name: String,
    email: String,
}

async fn get_user(pool: &sqlx::PgPool, user_id: i64) -> Result<User, sqlx::Error> {
    // SAFE: Explicit column selection guarantees stable column order regardless of schema changes.
    sqlx::query_as::<_, User>("SELECT id, name, email FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(pool)
        .await
}
