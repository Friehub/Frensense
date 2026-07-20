// [frensense]
// observation: A WebSocket server does not implement ping/pong timeouts. After the initial handshake, the server never sends pings or enforces a deadline for receiving pongs from the client. Idle connections remain open indefinitely.
// impact: Zombie connections accumulate, consuming file descriptors, memory, and task slots indefinitely. Over time, this exhausts server resources and prevents new legitimate clients from connecting, leading to denial of service.
// improvement: Configure a ping/pong interval (e.g., 10–30 seconds) with a timeout; close connections that do not respond in time.

use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:9004").await?;
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            let (mut write, mut read) = accept_async(stream).await.unwrap().split();
            while let Some(Ok(msg)) = read.next().await {
                if let Message::Text(t) = msg {
                    let _ = write.send(Message::Text(t)).await;
                }
            }
        });
    }
    Ok(())
}
