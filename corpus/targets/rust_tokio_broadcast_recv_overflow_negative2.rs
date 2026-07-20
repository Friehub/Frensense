use tokio::sync::watch;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = watch::channel(0u32);

    let sender = tokio::spawn(async move {
        for i in 0..10u32 {
            tx.send(i).ok();
            tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
        }
    });

    let receiver = tokio::spawn(async move {
        loop {
            let _ = rx.changed().await;
            let val = *rx.borrow();
            println!("latest {val}");
            if val >= 9 {
                break;
            }
        }
    });

    sender.await.ok();
    receiver.await.ok();
}
