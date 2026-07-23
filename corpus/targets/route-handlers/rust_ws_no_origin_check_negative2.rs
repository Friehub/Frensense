use tokio::net::TcpListener;
use tokio_tungstenite::accept_hdr_async;
use tokio_tungstenite::tungstenite::handshake::server::{Request, Response, ErrorResponse};
use futures_util::StreamExt;
use std::collections::HashSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let allowed_origins: HashSet<String> = ["https://app.example.com", "https://admin.example.com"]
        .iter().map(|s| s.to_string()).collect();
    let listener = TcpListener::bind("127.0.0.1:9000").await?;
    while let Ok((stream, peer)) = listener.accept().await {
        let origins = allowed_origins.clone();
        tokio::spawn(async move {
            let callback = move |req: &Request, resp: Response| -> Result<Response, ErrorResponse> {
                // SAFE: Only origins in the allowlist can establish a WebSocket connection.
                match req.headers().get("Origin").and_then(|v| v.to_str().ok()) {
                    Some(origin) if origins.contains(origin) => Ok(resp),
                    _ => Err(ErrorResponse::new(Some("origin not allowed".into()))),
                }
            };
            let ws_stream = accept_hdr_async(stream, callback).await.unwrap();
            let (_, mut read) = ws_stream.split();
            while let Some(msg) = read.next().await {
                eprintln!("msg from {}: {:?}", peer, msg);
            }
        });
    }
    Ok(())
}
