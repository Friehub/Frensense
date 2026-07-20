// [frensense]
// observation: `tokio::task::spawn_blocking` is called but the returned `JoinHandle` is discarded without being awaited.
// impact: The blocking task runs in the background, but errors are silently lost. If the task panics, the panic is swallowed. The application has no way to know if the task completed successfully.
// improvement: Store the `JoinHandle` and await it, or use `spawn_blocking` with structured concurrency.

pub async fn process() {
    tokio::task::spawn_blocking(|| {
        std::thread::sleep(std::time::Duration::from_secs(10));
    });
}
