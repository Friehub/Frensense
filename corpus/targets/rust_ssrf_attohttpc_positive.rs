// [frensense]
// observation: User-controlled URL is passed directly to attohttpc::get without validation, enabling SSRF.
// impact: An attacker can make the server issue GET requests to internal services, cloud metadata, or localhost.
// improvement: Validate the host against an allowlist or reject private IP ranges.

use attohttpc;

fn fetch_url(url: &str) -> Result<String, attohttpc::Error> {
    let resp = attohttpc::get(url).send()?;
    Ok(resp.text()?)
}

fn fetch_resource(user_url: String) -> Result<Vec<u8>, attohttpc::Error> {
    let resp = attohttpc::get(&user_url).send()?;
    Ok(resp.bytes()?)
}
