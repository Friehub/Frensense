// [frensense]
// observation: User-controlled URL is passed directly to surf::get or surf::Client::get without validation, enabling SSRF.
// impact: An attacker can make requests to internal network services, cloud metadata endpoints, or localhost.
// improvement: Validate the URL against an allowlist before issuing the request.

use surf;

async fn external_fetch(url: &str) -> Result<String, surf::Error> {
    let mut resp = surf::get(url).await?;
    Ok(resp.body_string().await?)
}

async fn agent_fetch(user_url: String) -> Result<String, surf::Error> {
    let client = surf::client();
    let mut resp = client.get(&user_url).await?;
    Ok(resp.body_string().await?)
}
