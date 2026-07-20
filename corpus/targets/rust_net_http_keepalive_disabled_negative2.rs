// SAFE: Uses hyper with connection pooling via keep-alive, sharing the same client across requests.

use hyper_util::client::legacy::Client;

async fn fetch_data(host: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder(
        hyper_util::rt::TokioExecutor::new()
    ).build_http();
    let uri: hyper::Uri = format!("http://{}:{}/api/data", host, port).parse()?;
    let resp = client.get(uri).await?;
    println!("status: {}", resp.status());
    Ok(())
}
