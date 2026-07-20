use sqlx::migrate::Migrator;
use std::path::Path;

#[tokio::main]
async fn main() {
    let pool = sqlx::PgPool::connect("postgres://localhost/test").await.unwrap();
    // SAFE: Embedded migrations compile SQL into the binary; no runtime path traversal.
    let migrator = Migrator::new(Path::new("./migrations")).await.unwrap();
    migrator.run(&pool).await.unwrap();
}
