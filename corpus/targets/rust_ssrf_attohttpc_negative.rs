// SAFE: URL is validated against host allowlist
use attohttpc;

const ALLOWED_DOMAINS: &[&str] = &["api.example.com"];

fn fetch_url(url: &str) -> Result<String, attohttpc::Error> {
    let parsed = url::Url::parse(url).map_err(|_| attohttpc::Error::new("bad url"))?;
    let host = parsed.host_str().ok_or_else(|| attohttpc::Error::new("no host"))?;
    if !ALLOWED_DOMAINS.contains(&host) {
        return Err(attohttpc::Error::new("host not allowed"));
    }
    let resp = attohttpc::get(url).send()?;
    Ok(resp.text()?)
}
