// [frensense]
// observation: `tokio::sync::Semaphore::add_permits` is called in a loop without any bound check, adding permits far beyond the intended capacity. The semaphore's permit count grows without limit.
// impact: The semaphore no longer limits concurrency — an unbounded number of tasks can acquire permits simultaneously. This defeats the purpose of rate-limiting, potentially causing resource exhaustion (connection pool overflow, memory pressure).
// improvement: Avoid calling `add_permits` in uncontrolled loops. Use a fixed set of permits initialized at construction, or add permits only when explicitly releasing known resources.

use tokio::sync::Semaphore;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let sem = Arc::new(Semaphore::new(10));
    let mut handles = Vec::new();

    for _ in 0..100 {
        let sem = sem.clone();
        handles.push(tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            // Reset: add permits without limit
            sem.add_permits(10);
        }));
    }

    for h in handles {
        h.await.ok();
    }
    println!("available: {}", sem.available_permits());
}
