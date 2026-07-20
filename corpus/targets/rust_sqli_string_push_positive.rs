use sqlx::PgPool;

async fn search_items(pool: &PgPool, name: &str) -> Result<Vec<Item>, sqlx::Error> {
    let mut query = String::from("SELECT * FROM items WHERE ");
    query.push_str(&format!("name = '{}'", name));
    let items = sqlx::query_as::<_, Item>(&query)
        .fetch_all(pool)
        .await?;
    Ok(items)
}
