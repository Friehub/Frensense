// SAFE: Receiver is closed explicitly to unblock sender before drop.
use tokio::sync::oneshot;

pub async fn worker() {
    let (tx, rx) = oneshot::channel::<String>();
    let _tx = tx;
    rx.close();
    drop(rx);
}
