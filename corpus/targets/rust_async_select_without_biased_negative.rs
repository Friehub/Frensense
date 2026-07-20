// SAFE: Uses biased; to ensure shutdown signal takes priority over work processing
use tokio::sync::watch;

async fn worker(mut cancel_rx: watch::Receiver<bool>, mut work_rx: tokio::sync::mpsc::Receiver<Job>) {
    loop {
        tokio::select! {
            biased;
            _ = cancel_rx.changed() => break,
            Some(job) = work_rx.recv() => process_job(job).await,
        }
    }
}
