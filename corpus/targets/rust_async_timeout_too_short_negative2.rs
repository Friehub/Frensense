// SAFE: Timeout is set to a reasonable 30 seconds for network operations.
use tokio::time::{timeout, Duration};

pub async fn fetch_data() -> Result<String, ()> {
    timeout(Duration::from_secs(30), async {
        tokio::time::sleep(Duration::from_millis(100)).await;
        Ok("data".to_string())
    }).await.map_err(|_| ())?
}
