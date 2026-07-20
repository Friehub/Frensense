async fn run_migration(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    // SAFE: Explicit transaction ensures atomicity — all changes roll back on failure.
    let mut tx = pool.begin().await?;
    sqlx::query("CREATE TABLE IF NOT EXISTS users (id SERIAL PRIMARY KEY, name TEXT NOT NULL)")
        .execute(&mut *tx)
        .await?;
    sqlx::query("ALTER TABLE users ADD COLUMN email TEXT NOT NULL")
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(())
}
