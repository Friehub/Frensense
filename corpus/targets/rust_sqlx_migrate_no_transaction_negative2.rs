async fn run_migration(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    // SAFE: Single DDL statement — no partial migration risk since each statement is atomic.
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL
        )"
    )
    .execute(pool)
    .await?;
    Ok(())
}
