// [frensense]
// observation: `tokio::sync::Notify::notified()` returns a future that is created but never awaited or the `notify_one()` call is missing, causing notified futures to accumulate.
// impact: Memory leak as pending notifications pile up, and tasks that depend on being notified remain blocked forever.
// improvement: Always match `notify_one()`/`notify_waiters()` with an awaited `notified()` future.

use tokio::sync::Notify;

pub async fn waiter(notify: &Notify) {
    let _fut = notify.notified();
}
