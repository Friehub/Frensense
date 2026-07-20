use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::tungstenite::Message;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:9001").await?;
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            let (mut write, mut read) = accept_async(stream).await.unwrap().split();
            let max_messages_per_sec = 10u32;
            let mut window_start = Instant::now();
            let mut count = 0u32;
            while let Some(Ok(msg)) = read.next().await {
                let now = Instant::now();
                if now.duration_since(window_start).as_secs_f64() > 1.0 {
                    window_start = now;
                    count = 0;
                }
                count += 1;
                // SAFE: Per-connection rate limit drops excess messages.
                if count <= max_messages_per_sec {
                    let _ = write.send(Message::Text(format!("echo: {}", msg))).await;
                }
            }
        });
    }
    Ok(())
}
