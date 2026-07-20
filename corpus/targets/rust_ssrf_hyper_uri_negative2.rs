// SAFE: Uses a hyper client with a connector that blocks private IP ranges; all requests are filtered at the transport layer
use hyper::{Client, Uri, client::HttpConnector};
use hyper::body::HttpBody as _;
use std::net::SocketAddr;
use hyper::service::service_fn;
use hyper::client::connect::HttpConnector;

struct SafeConnector;

impl tower::Service<Uri> for SafeConnector {
    type Response = hyper::client::connect::HttpConnection;
    type Error = std::io::Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, uri: Uri) -> Self::Future {
        let host = uri.host().unwrap_or("").to_string();
        Box::pin(async move {
            let addr: SocketAddr = tokio::net::lookup_host((host.as_str(), 443))
                .await?
                .find(|a| !(a.ip().is_loopback() || a.ip().is_private() || a.ip().is_unspecified()))
                .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::PermissionDenied, "blocked"))?;
            let stream = tokio::net::TcpStream::connect(addr).await?;
            Ok(hyper::client::connect::HttpConnection::new(stream))
        })
    }
}

async fn fetch_uri(uri_str: &str) -> Result<String, Box<dyn std::error::Error>> {
    let uri: Uri = uri_str.parse()?;
    let connector = SafeConnector;
    let client = Client::builder().build::<_, hyper::Body>(connector);
    let resp = client.get(uri).await?;
    let body = hyper::body::to_bytes(resp.into_body()).await?;
    Ok(String::from_utf8(body.to_vec())?)
}
