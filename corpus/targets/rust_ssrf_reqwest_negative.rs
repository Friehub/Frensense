// SAFE: URL is validated against a list of allowed hosts before the request is made
use reqwest;
use url::Url;

const ALLOWED_HOSTS: &[&str] = &["api.trusted.com", "data.trusted.com"];

fn is_safe_url(url_str: &str) -> bool {
    match Url::parse(url_str) {
        Ok(parsed) => {
            if parsed.scheme() != "https" {
                return false;
            }
            ALLOWED_HOSTS.contains(&parsed.host_str().unwrap_or(""))
        }
        Err(_) => false,
    }
}

async fn fetch_external(url: &str) -> Result<String, String> {
    if !is_safe_url(url) {
        return Err("URL not allowed".into());
    }
    let resp = reqwest::get(url)
        .await
        .map_err(|e| e.to_string())?;
    let body = resp.text().await.map_err(|e| e.to_string())?;
    Ok(body)
}
