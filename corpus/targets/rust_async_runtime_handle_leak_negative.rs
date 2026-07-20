// SAFE: Uses .await directly instead of nesting block_on inside async
async fn fetch_data() -> Result<String, reqwest::Error> {
    reqwest::get("https://example.com").await?.text().await
}
