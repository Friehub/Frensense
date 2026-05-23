fn test_sync() {
    std::thread::sleep(std::time::Duration::from_secs(1));
}

async fn test_async_safe() {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}
