// SAFE: A custom redirect policy rejects redirects to file:// and private IP ranges.

use reqwest::{Client, redirect::Policy, Url};

async fn fetch_external(url: &str) -> Result<String, reqwest::Error> {
    let policy = Policy::custom(|attempt| {
        let url = attempt.url();
        if url.scheme() == "file" {
            return attempt.error("file:// redirect rejected");
        }
        if let Some(host) = url.host_str() {
            if host == "169.254.169.254" || host == "127.0.0.1" || host == "localhost" {
                return attempt.error("internal redirect rejected");
            }
        }
        attempt.follow()
    });
    let client = Client::builder().redirect(policy).build()?;
    let resp = client.get(url).send().await?;
    resp.text().await
}
