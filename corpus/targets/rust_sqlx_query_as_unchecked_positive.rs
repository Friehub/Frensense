// [frensense]
// observation: `sqlx::query_as::<T>` is used with a wildcard select (`SELECT *`) and a struct `T` whose fields may not match the actual column order or types. If the table schema changes, the query silently maps columns by position rather than by name, producing incorrect or panic-inducing data.
// impact: A column reorder or addition in the database schema causes the query to silently return wrong values or panic at runtime due to type mismatch. Production incidents with data corruption are possible.
// improvement: Use explicit column selection in the query, or use `query_as::<T, _>` with `FromRow` and ensure the struct matches the query.

use sqlx::FromRow;

#[derive(FromRow)]
struct User {
    id: i64,
    name: String,
    email: String,
}

async fn get_user(pool: &sqlx::PgPool, user_id: i64) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(pool)
        .await
}
