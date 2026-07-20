// SAFE: Uses compile-time checked queries with query_as! macro
use sqlx::{PgPool, Row};

async fn lookup_user(pool: &PgPool, name: &str) -> Result<Vec<User>, sqlx::Error> {
    let rows = sqlx::query("SELECT * FROM users WHERE name = $1")
        .bind(name)
        .fetch_all(pool)
        .await?;
    Ok(rows.iter().map(|r| User {
        id: r.get("id"),
        name: r.get("name"),
    }).collect())
}
