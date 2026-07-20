// [frensense]
// observation: User input is concatenated directly into a sqlx::query string, enabling SQL injection via string interpolation.
// impact: An attacker can inject arbitrary SQL statements, leading to data exfiltration, modification, or deletion.
// improvement: Use parameterized queries with bind() instead of string concatenation.

use sqlx::PgPool;

async fn lookup_user(pool: &PgPool, name: &str) -> Result<Vec<User>, sqlx::Error> {
    let sql = "SELECT * FROM users WHERE name = '" + name + "'";
    sqlx::query_as::<_, User>(&sql).fetch_all(pool).await
}

async fn delete_order(pool: &PgPool, order_id: &str) -> Result<(), sqlx::Error> {
    let sql = format!("DELETE FROM orders WHERE id = '{}'", order_id);
    sqlx::query(&sql).execute(pool).await?;
    Ok(())
}
