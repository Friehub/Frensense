// SAFE: Connection string is read from an environment variable instead of being hardcoded
use sqlx::PgPool;
use std::env;

async fn connect_db() -> Result<PgPool, String> {
    let database_url = env::var("DATABASE_URL").map_err(|_| "DATABASE_URL not set".to_string())?;
    let pool = PgPool::connect(&database_url).await.map_err(|e| e.to_string())?;
    Ok(pool)
}

async fn init_redis() -> Result<redis::Connection, String> {
    let redis_url = env::var("REDIS_URL").map_err(|_| "REDIS_URL not set".to_string())?;
    let client = redis::Client::open(redis_url.as_str()).map_err(|e| e.to_string())?;
    let conn = client.get_connection().map_err(|e| e.to_string())?;
    Ok(conn)
}
