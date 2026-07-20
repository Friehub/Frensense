// SAFE: Uses sqlx::query_as with bind parameters instead of string concatenation
use sqlx::PgPool;

async fn search_items(pool: &PgPool, name: &str) -> Result<Vec<Item>, sqlx::Error> {
    let items = sqlx::query_as::<_, Item>("SELECT * FROM items WHERE name = $1")
        .bind(name)
        .fetch_all(pool)
        .await?;
    Ok(items)
}
