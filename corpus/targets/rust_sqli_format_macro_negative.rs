use sqlx::PgPool;

async fn get_user(pool: &PgPool, user_id: &str) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(pool)
        .await?;
    Ok(user)
}
