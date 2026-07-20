// SAFE: Per-request timeout is set using tokio::time::timeout, wrapping the entire request even if Client has no default timeout.

use reqwest::Client;
use std::time::Duration;
use tokio::time::timeout;

async fn fetch_data(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let resp = timeout(Duration::from_secs(10), client.get(url).send()).await??;
    let body = timeout(Duration::from_secs(10), resp.text()).await??;
    Ok(body)
}
