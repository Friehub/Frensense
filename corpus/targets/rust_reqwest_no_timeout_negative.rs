// SAFE: A timeout is configured on the Client, preventing indefinite hangs.

use reqwest::Client;
use std::time::Duration;

async fn fetch_data(url: &str) -> Result<String, reqwest::Error> {
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(5))
        .build()?;
    let resp = client.get(url).send().await?;
    resp.text().await
}

async fn post_event(endpoint: &str, body: String) -> Result<(), reqwest::Error> {
    let client = Client::builder()
        .timeout(Duration::from_secs(15))
        .build()?;
    let _resp = client.post(endpoint).body(body).send().await?;
    Ok(())
}
