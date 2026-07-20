use tokio::sync::Notify;

pub async fn waiter(notify: &Notify) {
    notify.notified().await;
}
