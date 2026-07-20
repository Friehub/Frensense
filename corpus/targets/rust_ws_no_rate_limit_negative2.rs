use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::tungstenite::Message;
use governor::{Quota, RateLimiter, clock::DefaultClock, state::direct::NotKeyed};
use std::num::NonZeroU32;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:9001").await?;
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            let (mut write, mut read) = accept_async(stream).await.unwrap().split();
            // SAFE: Token-bucket rate limiter allows 10 messages/sec with burst of 20.
            let lim = Arc::new(RateLimiter::direct(Quota::with_period(std::time::Duration::from_secs(1)).unwrap()
                .allow_burst(NonZeroU32::new(20).unwrap())));
            while let Some(Ok(msg)) = read.next().await {
                if lim.check().is_ok() {
                    let _ = write.send(Message::Text(format!("echo: {}", msg))).await;
                }
            }
        });
    }
    Ok(())
}
