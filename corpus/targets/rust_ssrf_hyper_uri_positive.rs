// [frensense]
// observation: A user-controlled string is directly parsed as a hyper::Uri and used in a client request without validation, allowing SSRF to arbitrary hosts.
// impact: An attacker can provide a URI pointing to internal services or cloud metadata endpoints, and the application will make requests to those URIs.
// improvement: Validate the URI host and scheme against an allowlist before making the request.

use hyper::{Client, Uri};
use hyper::body::HttpBody as _;

async fn fetch_uri(uri_str: &str) -> Result<String, Box<dyn std::error::Error>> {
    let uri: Uri = uri_str.parse()?;
    let client = Client::new();
    let resp = client.get(uri).await?;
    let body = hyper::body::to_bytes(resp.into_body()).await?;
    Ok(String::from_utf8(body.to_vec())?)
}
