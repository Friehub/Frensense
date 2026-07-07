async fn handle_request_async(url: &str) -> Result<Response, Error> {
    // This is a correct async function that awaits a future.
    // It is structurally complex and distinct from a simple sync handle.
    let client = reqwest::Client::new();
    let response = client.get(url)
        .header("User-Agent", "frensense-test")
        .send()
        .await?;
    
    if response.status().is_success() {
        println!("Successfully fetched URL");
    }
    
    Ok(response)
}
