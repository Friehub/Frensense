// SAFE: Uses query prepared with type-specific parameter binding and LIMIT as integer parameter
use tokio_postgres::{Client, types::ToSql};
use std::sync::Arc;

async fn get_product(client: &Client, code: &str) -> Result<Vec<Product>, tokio_postgres::Error> {
    let stmt = client
        .prepare("SELECT * FROM products WHERE code = $1")
        .await?;
    client.query(&stmt, &[&code]).await
}

async fn search_orders(client: &Client, status: &str, limit: i64) -> Result<Vec<Order>, tokio_postgres::Error> {
    let stmt = client
        .prepare("SELECT * FROM orders WHERE status = $1 LIMIT $2")
        .await?;
    client.query(&stmt, &[&status, &limit]).await
}
