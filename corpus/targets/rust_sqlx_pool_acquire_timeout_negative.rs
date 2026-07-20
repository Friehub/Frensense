use sqlx::PgPool;
use tokio::time::{timeout, Duration};

async fn query_user(pool: &PgPool, id: i64) -> Result<Option<String>, sqlx::Error> {
    // SAFE: `timeout` prevents hanging forever if no connection is available.
    let mut conn = timeout(Duration::from_secs(5), pool.acquire())
        .await
        .map_err(|_| sqlx::Error::PoolTimedOut)?;
    let row: Option<(String,)> = sqlx::query_as("SELECT name FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(&mut *conn)
        .await?;
    Ok(row.map(|r| r.0))
}

#[tokio::main]
async fn main() {
    let pool = PgPool::connect("postgres://localhost/test").await.unwrap();
    let name = query_user(&pool, 42).await.unwrap_or_default();
    println!("{name:?}");
}
