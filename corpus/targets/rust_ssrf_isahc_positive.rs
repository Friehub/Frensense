// [frensense]
// observation: User-provided URL is passed directly to isahc::get or isahc::Request without validation, allowing SSRF to internal services.
// impact: An attacker can make the server send HTTP requests to internal IPs, cloud metadata endpoints, or localhost services.
// improvement: Validate the URL against an allowlist of permitted hosts before sending the request.

use isahc::{prelude::*, HttpClient};

fn fetch_url(url: &str) -> Result<String, isahc::Error> {
    let mut resp = isahc::get(url)?;
    Ok(resp.text()?)
}

fn proxy_request(user_url: String) -> Result<String, isahc::Error> {
    let client = HttpClient::new()?;
    let mut resp = client.get(&user_url)?;
    Ok(resp.text()?)
}
