// SAFE: Uses a pre-configured HttpClient with redirect restrictions and IP filter
use isahc::{HttpClient, config::RedirectPolicy};
use std::net::IpAddr;

fn build_safe_client() -> Result<HttpClient, isahc::Error> {
    HttpClient::builder()
        .redirect_policy(RedirectPolicy::Limit(0))
        .build()
}

fn fetch_allowed(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let parsed = url::Url::parse(url)?;
    let ip: IpAddr = dns_lookup(parsed.host_str().unwrap())?;
    if ip.is_loopback() || ip.is_private() {
        return Err("internal IP not allowed".into());
    }
    let client = build_safe_client()?;
    let mut resp = client.get(url)?;
    Ok(resp.text()?)
}
