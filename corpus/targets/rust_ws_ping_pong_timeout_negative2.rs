use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::tungstenite::Message;
use tokio::time::{sleep, Duration, timeout};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:9004").await?;
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            let (mut write, mut read) = accept_async(stream).await.unwrap().split();
            loop {
                // SAFE: Timeout on read ensures zombie connections are cleaned up.
                match timeout(Duration::from_secs(30), read.next()).await {
                    Ok(Some(Ok(Message::Text(t)))) => {
                        let _ = write.send(Message::Text(t)).await;
                    }
                    Ok(Some(Ok(Message::Pong(_)))) => continue,
                    _ => break,
                }
            }
        });
    }
    Ok(())
}
