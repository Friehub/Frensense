use tokio::net::TcpListener;
use tokio_tungstenite::accept_hdr_async;
use tokio_tungstenite::tungstenite::handshake::server::{Request, Response, ErrorResponse};
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:9000").await?;
    while let Ok((stream, peer)) = listener.accept().await {
        tokio::spawn(async move {
            let callback = |req: &Request, resp: Response| -> Result<Response, ErrorResponse> {
                // SAFE: Origin header must match the trusted application domain.
                match req.headers().get("Origin") {
                    Some(origin) if origin == "https://app.example.com" => Ok(resp),
                    _ => Err(ErrorResponse::new(Some("forbidden origin".into()))),
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
