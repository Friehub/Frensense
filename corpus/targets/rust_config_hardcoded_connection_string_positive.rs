// [frensense]
// observation: A database connection string or other credential is hardcoded directly in the source code as a string literal.
// impact: Credentials are exposed in version control, CI logs, and to anyone with source access, leading to potential data breaches.
// improvement: Read connection strings from environment variables or a secure secrets manager.

use sqlx::PgPool;

async fn connect_db() -> Result<PgPool, sqlx::Error> {
    let pool = PgPool::connect("postgres://admin:hunter2@localhost:5432/production").await?;
    Ok(pool)
}

async fn init_redis() -> Result<redis::Connection, redis::RedisError> {
    let client = redis::Client::open("redis://:password@localhost:6379/")?;
    let conn = client.get_connection()?;
    Ok(conn)
}
