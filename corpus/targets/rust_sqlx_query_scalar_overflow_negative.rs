use sqlx::PgPool;

async fn get_total_revenue(pool: &PgPool) -> Result<i64, sqlx::Error> {
    // SAFE: `i64` matches the `BIGINT` / `NUMERIC` range, no truncation.
    sqlx::query_scalar::<_, i64>("SELECT SUM(amount) FROM transactions")
        .fetch_one(pool)
        .await
}

#[tokio::main]
async fn main() {
    let pool = PgPool::connect("postgres://localhost/test").await.unwrap();
    let revenue = get_total_revenue(&pool).await.unwrap();
    println!("revenue: {revenue}");
}
