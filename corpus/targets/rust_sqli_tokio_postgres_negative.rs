use tokio_postgres::Client;

async fn get_product(client: &Client, code: &str) -> Result<Vec<Product>, tokio_postgres::Error> {
    client.query("SELECT * FROM products WHERE code = $1", &[&code]).await
}

async fn search_orders(client: &Client, status: &str, limit: i64) -> Result<Vec<Order>, tokio_postgres::Error> {
    client.query("SELECT * FROM orders WHERE status = $1 LIMIT $2", &[&status, &limit]).await
}
