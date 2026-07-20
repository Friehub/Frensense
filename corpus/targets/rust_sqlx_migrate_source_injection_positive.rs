// [frensense]
// observation: A migration path is constructed by concatenating user-controlled input (e.g., from a query parameter or request body) with a base directory. An attacker can use `../` path traversal to load arbitrary SQL files from the filesystem.
// impact: Arbitrary SQL files on the server can be executed as migrations, including files outside the intended migration directory. This can lead to privilege escalation, data destruction, or reading sensitive data via crafted SQL.
// improvement: Validate the migration path against an allowlist, or use embedded migrations (`sqlx::migrate!()`) which compile the SQL into the binary.

use sqlx::migrate::Migrator;
use std::path::Path;

async fn run_migration(pool: &sqlx::PgPool, user_path: &str) -> Result<(), sqlx::Error> {
    let base = std::path::Path::new("./migrations");
    let full_path = base.join(user_path);
    let migrator = Migrator::new(full_path).await?;
    migrator.run(pool).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let pool = sqlx::PgPool::connect("postgres://localhost/test").await.unwrap();
    run_migration(&pool, "../../etc/passwd").await.unwrap();
}
