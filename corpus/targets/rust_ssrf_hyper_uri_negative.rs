// SAFE: URI is parsed and validated before the request; only HTTPS to allowed hosts is permitted
use hyper::{Client, Uri};
use hyper::body::HttpBody as _;

const ALLOWED_HOSTS: &[&str] = &["api.trusted.com", "data.trusted.com"];

fn is_safe_uri(uri: &Uri) -> bool {
    if uri.scheme_str() != Some("https") {
        return false;
    }
    let host = uri.host().unwrap_or("");
    ALLOWED_HOSTS.contains(&host)
}

async fn fetch_uri(uri_str: &str) -> Result<String, String> {
    let uri: Uri = uri_str.parse().map_err(|e| format!("Invalid URI: {}", e))?;
    if !is_safe_uri(&uri) {
        return Err("URI not allowed".into());
    }
    let client = Client::new();
    let resp = client.get(uri).await.map_err(|e| e.to_string())?;
    let body = hyper::body::to_bytes(resp.into_body())
        .await
        .map_err(|e| e.to_string())?;
    Ok(String::from_utf8(body.to_vec()).map_err(|e| e.to_string())?)
}
