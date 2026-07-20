// SAFE: notified() future is immediately consumed with a timeout guard.
use tokio::sync::Notify;
use tokio::time::{timeout, Duration};

pub async fn waiter(notify: &Notify) {
    timeout(Duration::from_secs(5), notify.notified()).await.ok();
}
