// [frensense]
// observation: A `tokio::sync::broadcast::Receiver` does not call `recv` frequently enough, causing the channel's internal buffer to overflow. When the lag exceeds capacity, `recv` returns `RecvError::Lagged(n)`, silently dropping `n` messages.
// impact: The receiver silently misses messages without the sender being notified. For event-driven systems (order notifications, price feeds, log streams), this causes data loss and incorrect application state.
// improvement: Process messages fast enough to keep up, increase channel capacity, or use a different channel type (`watch` for latest-value semantics, or `mpsc` with backpressure).

use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = broadcast::channel(2);

    let sender = tokio::spawn(async move {
        for i in 0..10u32 {
            tx.send(i).ok();
            tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
        }
    });

    let receiver = tokio::spawn(async move {
        loop {
            match rx.recv().await {
                Ok(msg) => println!("got {msg}"),
                Err(broadcast::error::RecvError::Lagged(n)) => {
                    println!("missed {n} messages");
                }
                Err(broadcast::error::RecvError::Closed) => break,
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }
    });

    sender.await.ok();
    receiver.await.ok();
}
