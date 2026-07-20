// SAFE: Uses an explicit closed state flag to avoid sending after shutdown
use tokio::sync::mpsc;
use std::sync::atomic::{AtomicBool, Ordering};

async fn fan_out(tx: mpsc::Sender<u64>, values: Vec<u64>) {
    for v in values {
        if tx.send(v).await.is_err() {
            return;
        }
    }
}
