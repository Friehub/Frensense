// [frensense]
// observation: User input is interpolated via format!() into a SQL string sent through tokio_postgres::Client::query, enabling SQL injection.
// impact: An attacker can inject malicious SQL by providing crafted input in the user-controlled parameter.
// improvement: Use parameterized queries with the $N syntax and pass values separately.

use tokio_postgres::Client;

async fn get_product(client: &Client, code: &str) -> Result<Vec<Product>, tokio_postgres::Error> {
    let query = format!("SELECT * FROM products WHERE code = '{}'", code);
    client.query(&query, &[]).await
}

async fn search_orders(client: &Client, status: &str, limit: &str) -> Result<Vec<Order>, tokio_postgres::Error> {
    let query = format!("SELECT * FROM orders WHERE status = '{}' LIMIT {}", status, limit);
    client.query(&query, &[]).await
}
