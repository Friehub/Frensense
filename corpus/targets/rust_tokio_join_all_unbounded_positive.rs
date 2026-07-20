// [frensense]
// observation: `futures::future::join_all` is used on a potentially unbounded `Vec<impl Future>`. All futures are polled concurrently and held in memory until the slowest completes.
// impact: If the vector grows with user-controlled input, memory scales linearly with the number of futures. An attacker can exhaust memory by sending many requests that each spawn a future — OOM denial of service.
// improvement: Use `FuturesUnordered` with concurrency limiting, or process futures in bounded batches.

use futures::future::join_all;
use tokio::time::{sleep, Duration};

async fn fetch_user(id: u32) -> u32 {
    sleep(Duration::from_millis(10)).await;
    id
}

async fn fetch_all(ids: Vec<u32>) -> Vec<u32> {
    let futs: Vec<_> = ids.into_iter().map(fetch_user).collect();
    join_all(futs).await
}

#[tokio::main]
async fn main() {
    let ids: Vec<u32> = (0..100_000).collect();
    let results = fetch_all(ids).await;
    println!("{}", results.len());
}
