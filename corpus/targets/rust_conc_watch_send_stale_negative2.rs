// SAFE: Uses a backchannel (oneshot) to confirm the receiver processed each value.
use tokio::sync::{watch, oneshot};

pub async fn producer(tx: watch::Sender<i32>, mut ack: oneshot::Receiver<()>) {
    for i in 0..10 {
        tx.send(i).ok();
        ack.await.ok();
        (ack, _) = oneshot::channel();
    }
}
