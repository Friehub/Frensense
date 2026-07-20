use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = broadcast::channel(64);

    let sender = tokio::spawn(async move {
        for i in 0..10u32 {
            tx.send(i).ok();
            tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
        }
    });

    let receiver = tokio::spawn(async move {
        loop {
            // SAFE: Capacity is large enough for the expected message rate.
            match rx.recv().await {
                Ok(msg) => println!("got {msg}"),
                Err(broadcast::error::RecvError::Lagged(n)) => {
                    println!("missed {n} messages");
                }
                Err(broadcast::error::RecvError::Closed) => break,
            }
        }
    });

    sender.await.ok();
    receiver.await.ok();
}
