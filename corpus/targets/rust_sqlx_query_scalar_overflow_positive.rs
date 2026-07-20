// [frensense]
// observation: `sqlx::query_scalar` is used with a Rust type that cannot represent the full range of the database column's type. For example, `i32` for a DB `BIGINT` column silently truncates values beyond `i32::MAX`.
// impact: Values larger than the target type's maximum are silently truncated to a smaller value (e.g., 2^31 becomes -2^31 for `i32`). This causes silent data corruption in financial calculations, IDs, or counters.
// improvement: Match the Rust scalar type to the database column's precision (e.g., `i64` for `BIGINT`), or use `query_as` with a strongly-typed struct.

use sqlx::PgPool;

async fn get_total_revenue(pool: &PgPool) -> Result<i32, sqlx::Error> {
    sqlx::query_scalar::<_, i32>("SELECT SUM(amount) FROM transactions")
        .fetch_one(pool)
        .await
}

#[tokio::main]
async fn main() {
    let pool = PgPool::connect("postgres://localhost/test").await.unwrap();
    let revenue = get_total_revenue(&pool).await.unwrap();
    println!("revenue: {revenue}");
}
