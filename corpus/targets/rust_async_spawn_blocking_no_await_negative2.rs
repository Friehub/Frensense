// SAFE: JoinHandle is awaited to capture panics and ensure completion.
use tokio::task::JoinHandle;

pub async fn process() -> Result<(), tokio::task::JoinError> {
    let handle: JoinHandle<()> = tokio::task::spawn_blocking(|| {
        std::thread::sleep(std::time::Duration::from_secs(5));
    });
    handle.await
}
