// SAFE: URL is validated against an allowlist before sending the request
use surf;

const ALLOWED_HOSTS: &[&str] = &["api.example.com", "cdn.example.com"];

async fn external_fetch(url: &str) -> Result<String, surf::Error> {
    let parsed = url::Url::parse(url).map_err(|_| surf::Error::new("invalid URL"))?;
    if !ALLOWED_HOSTS.iter().any(|h| parsed.host_str() == Some(h)) {
        return Err(surf::Error::new("host not allowed"));
    }
    let mut resp = surf::get(url).await?;
    Ok(resp.body_string().await?)
}
