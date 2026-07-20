// [frensense]
// observation: tokio::select! is used without the biased; keyword, causing unfair branch selection that can starve certain branches.
// impact: When multiple branches are ready, select! picks one pseudo-randomly; without biased, the first branch can be starved by the second if both are always ready.
// improvement: Use biased; when you need deterministic priority, or restructure the select to ensure fairness.

use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

async fn handle_channels(mut high_rx: mpsc::Receiver<Msg>, mut low_rx: mpsc::Receiver<Msg>) {
    loop {
        tokio::select! {
            Some(msg) = high_rx.recv() => process_high(msg).await,
            Some(msg) = low_rx.recv() => process_low(msg).await,
            else => break,
        }
    }
}

async fn race_timeout(fut: impl std::future::Future<Output = ()>) {
    tokio::select! {
        _ = fut => {},
        _ = sleep(Duration::from_secs(5)) => {},
    }
}
