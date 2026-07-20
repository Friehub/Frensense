// [frensense]
// observation: A WebSocket message handler processes incoming messages from a connection without any per-connection rate limiting. A client can send messages as fast as the network allows, flooding the server.
// impact: Resource exhaustion — the server spends all CPU/memory processing messages from one aggressive client, starving other connections. This can lead to degraded service or complete denial of service.
// improvement: Implement per-connection rate limiting using a token bucket or sliding window that limits the number of messages processed per second for each WebSocket peer.

use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:9001").await?;
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            let (mut write, mut read) = accept_async(stream).await.unwrap().split();
            while let Some(Ok(msg)) = read.next().await {
                let _ = write.send(Message::Text(format!("echo: {}", msg))).await;
            }
        });
    }
    Ok(())
}
