// [frensense]
// observation: A tokio::sync::mpsc channel is used after being closed, or send() is called without checking whether the receiver has been dropped.
// impact: Sending on a closed channel returns an error that may be silently ignored, causing message loss and stalled processing.
// improvement: Always check the Result from send() or use the closed channel to trigger shutdown logic.

use tokio::sync::mpsc;

async fn broadcast(mut tx: mpsc::Sender<String>, msg: String) {
    tx.send(msg).await;
}

async fn fan_out(tx: mpsc::Sender<u64>, values: Vec<u64>) {
    for v in values {
        if tx.send(v).await.is_err() {
            break;
        }
    }
}
