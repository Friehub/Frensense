use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::tungstenite::Message;
use tokio::time::{interval, Duration, timeout};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:9004").await?;
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            let (mut write, mut read) = accept_async(stream).await.unwrap().split();
            let mut ping_interval = interval(Duration::from_secs(15));
            loop {
                tokio::select! {
                    msg = read.next() => {
                        match msg {
                            Some(Ok(Message::Pong(_))) => continue,
                            Some(Ok(Message::Text(t))) => {
                                let _ = write.send(Message::Text(t)).await;
                            }
                            _ => break,
                        }
                    }
                    _ = ping_interval.tick() => {
                        // SAFE: Ping/pong keeps alive and detects zombie connections.
                        let _ = write.send(Message::Ping(vec![])).await;
                    }
                }
            }
        });
    }
    Ok(())
}
