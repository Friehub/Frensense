// [frensense]
// observation: `tokio::time::timeout` is called with a duration of less than 1 millisecond, which is effectively immediate on most runtimes. This causes the wrapped future to always time out, even under normal conditions.
// impact: The operation always fails with a timeout error, making the system unavailable or rejecting valid requests.
// improvement: Use a reasonable timeout (at least 1 ms or more, depending on the operation's expected latency).

use tokio::time::{timeout, Duration};

pub async fn fetch_data() -> Result<String, ()> {
    timeout(Duration::from_nanos(100), async {
        tokio::time::sleep(Duration::from_millis(5)).await;
        Ok("data".to_string())
    }).await.map_err(|_| ())?
}
