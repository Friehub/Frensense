async fn process_non_blocking_sleep(duration_ms: u64) -> Result<(), String> {
    // This is a correct async function that uses non-blocking sleep.
    // It is structurally distinct from the positive example because it contains
    // loops, let bindings, and custom control flow.
    let mut retry_count = 0;
    while retry_count < 3 {
        match tokio::time::timeout(
            std::time::Duration::from_millis(duration_ms),
            tokio::time::sleep(std::time::Duration::from_millis(duration_ms))
        ).await {
            Ok(_) => {
                println!("Successfully completed sleep");
                return Ok(());
            }
            Err(_) => {
                retry_count += 1;
            }
        }
    }
    Err("Failed after max retries".to_string())
}
