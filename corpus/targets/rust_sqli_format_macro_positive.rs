use sqlx::PgPool;

async fn get_user(pool: &PgPool, user_id: &str) -> Result<User, sqlx::Error> {
    let query = format!("SELECT * FROM users WHERE id = '{}'", user_id);
    let user = sqlx::query_as::<_, User>(&query)
        .fetch_one(pool)
        .await?;
    Ok(user)
}
