use sqlx::FromRow;

#[derive(FromRow)]
struct User {
    id: i64,
    name: String,
    email: String,
}

async fn get_user(pool: &sqlx::PgPool, user_id: i64) -> Result<User, sqlx::Error> {
    // SAFE: Using query_as with the sqlx::Type check at compile time ensures column-type compatibility.
    let row: (i64, String, String) = sqlx::query_as(
        "SELECT id, name, email FROM users WHERE id = $1"
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(User { id: row.0, name: row.1, email: row.2 })
}
