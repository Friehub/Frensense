// SAFE: URL is validated against an allowlist before making the request
use isahc::HttpClient;
use std::collections::HashSet;

const ALLOWED_HOSTS: &[&str] = &["api.example.com", "data.example.com"];

fn fetch_url(url: &str) -> Result<String, isahc::Error> {
    let parsed = url::Url::parse(url).map_err(|_| isahc::Error::new("invalid URL"))?;
    if !ALLOWED_HOSTS.iter().any(|h| parsed.host_str() == Some(h)) {
        return Err(isahc::Error::new("host not allowed"));
    }
    let mut resp = isahc::get(url)?;
    Ok(resp.text()?)
}
