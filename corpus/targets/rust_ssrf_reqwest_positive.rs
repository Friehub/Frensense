// [frensense]
// observation: User-controlled URL is passed directly to reqwest::get or reqwest::Client::get without validation, allowing SSRF to internal services.
// impact: An attacker can make the server issue GET requests to internal IPs, cloud metadata endpoints, or otherwise unreachable services.
// improvement: Validate the URL against an allowlist of permitted hosts before making the request.

use reqwest;

async fn fetch_external(url: &str) -> Result<String, reqwest::Error> {
    let resp = reqwest::get(url).await?;
    let body = resp.text().await?;
    Ok(body)
}

async fn proxy_handler(url: String) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let resp = client.get(&url).send().await?;
    Ok(resp.text().await?)
}
