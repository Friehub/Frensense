// SAFE: Uses block_on to run the future synchronously instead of using .await in a non-async context
use futures::executor;

fn handle() {
    let result = executor::block_on(fetch_data());
    println!("Got result: {:?}", result);
}

async fn fetch_data() -> Result<String, Error> {
    reqwest::get("https://example.com").await?.text().await
}
