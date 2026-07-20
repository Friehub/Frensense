use tokio::sync::watch;

pub async fn producer(tx: watch::Sender<i32>) {
    for i in 0..10 {
        tx.send(i).ok();
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    }
}
