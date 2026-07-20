use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

#[tokio::main]
async fn main() {
    // SAFE: `acquire_timeout` on the pool itself enforces a maximum wait per acquire.
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(3))
        .connect("postgres://localhost/test")
        .await
        .unwrap();

    let row: Option<(String,)> = sqlx::query_as("SELECT name FROM users WHERE id = $1")
        .bind(42i64)
        .fetch_optional(&pool)
        .await
        .unwrap();
    println!("{row:?}");
}
