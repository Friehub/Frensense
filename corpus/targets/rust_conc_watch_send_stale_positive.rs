// [frensense]
// observation: A `tokio::sync::watch::Sender` sends values without coordinating with the receiver — rapid sends may overwrite before the receiver processes, causing stale intermediate values.
// impact: The receiver misses critical state transitions (e.g., a "stopping" state sent and overwritten by "stopped"), leading to incorrect application behavior.
// improvement: Use `oneshot` for synchronization or ensure sender respects receiver processing before sending the next value.

use tokio::sync::watch;

pub async fn producer(tx: watch::Sender<i32>) {
    for i in 0..10 {
        tx.send(i).ok();
    }
}
