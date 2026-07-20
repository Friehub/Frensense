// SAFE: Checks the Result from send() and handles closed channel
use tokio::sync::mpsc;

async fn broadcast(tx: mpsc::Sender<String>, msg: String) {
    if tx.send(msg).await.is_err() {
        eprintln!("receiver dropped, stopping");
    }
}

async fn fan_out(tx: mpsc::Sender<u64>, values: Vec<u64>) {
    for v in values {
        if tx.send(v).await.is_err() {
            break;
        }
    }
}
