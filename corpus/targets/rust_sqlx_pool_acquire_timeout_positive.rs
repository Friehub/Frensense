// [frensense]
// observation: `PgPool::acquire().await` is called without a timeout. If all connections are busy and no new connections can be established (e.g., database is down, connection limit reached), the future hangs indefinitely.
// impact: The server thread/task hangs forever, gradually consuming all available tasks in the runtime. Under database outage, every request ends up stuck on `acquire()`, causing complete denial of service with no recovery.
// improvement: Use `tokio::time::timeout` on the acquire call, or use `PgPoolOptions::acquire_timeout` to set a maximum wait duration.

use sqlx::PgPool;
use std::time::Duration;

async fn query_user(pool: &PgPool, id: i64) -> Result<Option<String>, sqlx::Error> {
    let mut conn = pool.acquire().await?;
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
