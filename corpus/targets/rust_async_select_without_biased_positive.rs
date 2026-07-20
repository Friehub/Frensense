// [frensense]
// observation: tokio::select! is used without the biased; keyword in scenarios where deterministic priority is expected, causing non-deterministic branch selection.
// impact: A cancellation or shutdown signal may not be processed promptly because select! randomly picks another ready branch, delaying shutdown and potentially causing data corruption.
// improvement: Use biased; when a branch must take priority (e.g., shutdown signals), or handle signals with dedicated priority channels.

use tokio::sync::watch;
use tokio::time::{sleep, Duration};

async fn worker(mut cancel_rx: watch::Receiver<bool>, mut work_rx: tokio::sync::mpsc::Receiver<Job>) {
    loop {
        tokio::select! {
            _ = cancel_rx.changed() => {
                break;
            }
            Some(job) = work_rx.recv() => {
                process_job(job).await;
            }
        }
    }
}

async fn stream_handler(mut shutdown: tokio::sync::oneshot::Receiver<()>) {
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    loop {
        tokio::select! {
            _ = interval.tick() => periodic_work().await,
            _ = &mut shutdown => break,
        }
    }
}
