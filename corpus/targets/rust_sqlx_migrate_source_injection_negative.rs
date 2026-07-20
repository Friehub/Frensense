use sqlx::migrate::Migrator;
use std::path::Path;

fn is_safe_path(base: &Path, user_path: &str) -> bool {
    let full = base.join(user_path);
    // SAFE: Canonicalize both paths and verify the result is within the base.
    let canonical = std::fs::canonicalize(&full).ok();
    let base_canonical = std::fs::canonicalize(base).ok();
    match (canonical, base_canonical) {
        (Some(c), Some(b)) => c.starts_with(&b),
        _ => false,
    }
}

async fn run_migration(pool: &sqlx::PgPool, user_path: &str) -> Result<(), sqlx::Error> {
    let base = Path::new("./migrations");
    if !is_safe_path(base, user_path) {
        return Err(sqlx::Error::Protocol("invalid migration path".into()));
    }
    let full_path = base.join(user_path);
    let migrator = Migrator::new(full_path).await?;
    migrator.run(pool).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let pool = sqlx::PgPool::connect("postgres://localhost/test").await.unwrap();
    run_migration(&pool, "001_init").await.unwrap();
}
