use sqlx::{PgPool, FromRow};

#[derive(FromRow)]
struct Revenue {
    total: Option<f64>,
}

async fn get_total_revenue(pool: &PgPool) -> Result<f64, sqlx::Error> {
    // SAFE: `query_as` with explicit struct avoids scalar type mismatch entirely.
    let row = sqlx::query_as::<_, Revenue>("SELECT SUM(amount)::float8 AS total FROM transactions")
        .fetch_one(pool)
        .await?;
    Ok(row.total.unwrap_or(0.0))
}

#[tokio::main]
async fn main() {
    let pool = PgPool::connect("postgres://localhost/test").await.unwrap();
    let revenue = get_total_revenue(&pool).await.unwrap();
    println!("revenue: {revenue}");
}
