// [frensense]
// observation: An HTTP request is made with reqwest without setting a timeout, allowing the connection to hang indefinitely if the server does not respond.
// impact: The application thread/task hangs forever on a slow or unresponsive server, leading to resource exhaustion (connection pool depletion, thread starvation) and denial of service.
// improvement: Set a connect timeout and request timeout on the reqwest::Client using .timeout() or .connect_timeout().

use reqwest::Client;

async fn fetch_data(url: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let resp = client.get(url).send().await?;
    resp.text().await
}

async fn post_event(endpoint: &str, body: String) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let _resp = client.post(endpoint).body(body).send().await?;
    Ok(())
}
