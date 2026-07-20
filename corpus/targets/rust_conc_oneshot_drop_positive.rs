// [frensense]
// observation: A `tokio::sync::oneshot::Receiver` is dropped without ever calling `.await` or `.try_recv()` to receive the value.
// impact: The sender blocks indefinitely or gets an error, causing a resource leak or task hang. The intended one-shot message is lost.
// improvement: Always await or explicitly close the receiver when the value is no longer needed.

use tokio::sync::oneshot;

pub async fn worker() {
    let (tx, rx) = oneshot::channel::<String>();
    tokio::spawn(async move {
        tx.send("result".into()).ok();
    });
    drop(rx);
}
