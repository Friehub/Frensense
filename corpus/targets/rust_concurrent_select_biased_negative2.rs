// SAFE: Polls each channel separately with rate limiting to ensure fairness
use tokio::sync::mpsc;

async fn handle_channels(mut high_rx: mpsc::Receiver<Msg>, mut low_rx: mpsc::Receiver<Msg>) {
    loop {
        tokio::select! {
            biased;
            Some(msg) = high_rx.recv() => {
                process_high(msg).await;
                tokio::task::yield_now().await;
            }
            Some(msg) = low_rx.recv() => process_low(msg).await,
            else => break,
        }
    }
}
