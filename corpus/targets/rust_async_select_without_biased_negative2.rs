// SAFE: Checks cancellation before processing each item instead of relying on select! fairness
use tokio::sync::watch;

async fn worker(mut cancel_rx: watch::Receiver<bool>, mut work_rx: tokio::sync::mpsc::Receiver<Job>) {
    loop {
        if cancel_rx.has_changed().unwrap_or(true) {
            break;
        }
        match work_rx.recv().await {
            Some(job) => process_job(job).await,
            None => break,
        }
    }
}
