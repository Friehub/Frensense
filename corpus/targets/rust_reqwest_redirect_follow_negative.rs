// SAFE: Redirect following is disabled entirely, preventing SSRF via redirect-based attacks.

use reqwest::{Client, redirect::Policy};

async fn fetch_external(url: &str) -> Result<String, reqwest::Error> {
    let client = Client::builder()
        .redirect(Policy::none())
        .build()?;
    let resp = client.get(url).send().await?;
    resp.text().await
}

async fn process_callback(callback_url: &str) -> Result<(), reqwest::Error> {
    let client = Client::builder()
        .redirect(Policy::none())
        .build()?;
    let _resp = client.post(callback_url).send().await?;
    Ok(())
}
