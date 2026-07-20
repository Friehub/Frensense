pub async fn process() -> Result<(), Box<dyn std::error::Error>> {
    let handle = tokio::task::spawn_blocking(|| {
        std::thread::sleep(std::time::Duration::from_secs(10));
    });
    handle.await?;
    Ok(())
}
