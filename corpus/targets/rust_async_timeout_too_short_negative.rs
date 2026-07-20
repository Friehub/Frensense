use tokio::time::{timeout, Duration};

pub async fn fetch_data() -> Result<String, ()> {
    timeout(Duration::from_secs(5), async {
        tokio::time::sleep(Duration::from_millis(5)).await;
        Ok("data".to_string())
    }).await.map_err(|_| ())?
}
