// SAFE: Uses async_std's non-blocking sleep instead of blocking std::thread::sleep
use async_std::task;
use std::time::Duration;

async fn process_non_blocking_sleep(duration_ms: u64) -> Result<(), String> {
    let mut attempts = 0u32;
    loop {
        task::sleep(Duration::from_millis(duration_ms)).await;
        attempts += 1;
        if attempts >= 3 {
            return Ok(());
        }
    }
}
