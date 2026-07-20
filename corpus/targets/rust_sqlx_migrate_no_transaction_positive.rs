// [frensense]
// observation: A SQLx migration script performs DDL or DML changes outside an explicit transaction. If the migration fails partway through, the database is left in a partially migrated state.
// impact: Partial migration breaks schema consistency — some tables may be altered while others are not, leading to application crashes, data corruption, or stuck migration states that require manual recovery.
// improvement: Wrap migration SQL in an explicit `BEGIN` / `COMMIT` block, or use `sqlx::migrate!` which auto-wraps in a transaction.

async fn run_migration(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("CREATE TABLE IF NOT EXISTS users (id SERIAL PRIMARY KEY, name TEXT NOT NULL)")
        .execute(pool)
        .await?;
    sqlx::query("ALTER TABLE users ADD COLUMN email TEXT NOT NULL")
        .execute(pool)
        .await?;
    Ok(())
}
