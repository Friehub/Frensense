use sqlx::PgPool;

async fn lookup_user(pool: &PgPool, name: &str) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE name = $1")
        .bind(name)
        .fetch_all(pool)
        .await
}

async fn delete_order(pool: &PgPool, order_id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM orders WHERE id = $1")
        .bind(order_id)
        .execute(pool)
        .await?;
    Ok(())
}
