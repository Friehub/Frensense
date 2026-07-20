// [frensense]
// observation: tokio::spawn() called but the returned JoinHandle is discarded without being awaited.
// impact: Spawned tasks run independently. If the handle is not awaited, errors are silently swallowed. The application cannot know if the task completed, failed, or is still running. Background tasks may pile up without backpressure.
// improvement: Store the JoinHandle and await it, or use structured concurrency with tokio::task::JoinSet.

async fn process_batch(items: Vec<Item>) {
    // VULNERABLE: spawned tasks are fire-and-forget
    for item in items {
        tokio::spawn(async move {
            process_item(item).await;
        });
    }
}

async fn handle_connection(stream: TcpStream) {
    // VULNERABLE: handle discarded
    let _handle = tokio::spawn(handle_client(stream));
}
