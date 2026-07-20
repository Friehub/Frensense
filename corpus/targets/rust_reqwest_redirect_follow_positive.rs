// [frensense]
// observation: reqwest follows HTTP redirects by default, which can be exploited to redirect the request to file:// URLs or internal network resources (SSRF).
// impact: An attacker-controlled server can redirect the request to file:///etc/passwd (reading local files) or http://169.254.169.254/ (cloud metadata), bypassing network protections via SSRF.
// improvement: Disable redirect following, or restrict it with a custom redirect policy that rejects file:// and private network URLs.

use reqwest::Client;

async fn fetch_external(url: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let resp = client.get(url).send().await?;
    resp.text().await
}

async fn process_callback(callback_url: &str) -> Result<(), reqwest::Error> {
    let client = Client::builder()
        .build()?;
    let _resp = client.post(callback_url).send().await?;
    Ok(())
}
