// [frensense]
// observation: A WebSocket broadcast system sends every incoming message to ALL connected clients, including those in different rooms or with different permissions. The broadcast function iterates over the global connection set and forwards the message unconditionally.
// impact: Sensitive data intended for one user or room is leaked to every connected client. This breaks confidentiality guarantees and can expose private messages, financial data, or internal system events.
// improvement: Scope broadcast to the appropriate room, group, or recipient list. Never send a message to clients who are not authorized to receive it.

use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::tungstenite::Message;
use tokio::sync::broadcast;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, _) = broadcast::channel::<String>(1024);
    let tx = Arc::new(tx);
    let listener = TcpListener::bind("127.0.0.1:9002").await?;
    while let Ok((stream, _)) = listener.accept().await {
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        tokio::spawn(async move {
            let (mut write, mut read) = accept_async(stream).await.unwrap().split();
            let tx2 = tx.clone();
            tokio::spawn(async move {
                while let Some(Ok(msg)) = read.next().await {
                    if let Message::Text(text) = msg {
                        let _ = tx2.send(text);
                    }
                }
            });
            while let Ok(msg) = rx.recv().await {
                let _ = write.send(Message::Text(msg)).await;
            }
        });
    }
    Ok(())
}
