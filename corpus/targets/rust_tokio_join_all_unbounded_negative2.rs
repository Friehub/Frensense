use futures::future::join_all;
use tokio::time::{sleep, Duration};

const BATCH_SIZE: usize = 100;

async fn fetch_user(id: u32) -> u32 {
    sleep(Duration::from_millis(10)).await;
    id
}

async fn fetch_all(ids: Vec<u32>) -> Vec<u32> {
    // SAFE: Processing in bounded batches prevents unbounded memory growth.
    let mut results = Vec::with_capacity(ids.len());
    for chunk in ids.chunks(BATCH_SIZE) {
        let batch: Vec<_> = chunk.iter().copied().map(fetch_user).collect();
        results.extend(join_all(batch).await);
    }
    results
}

#[tokio::main]
async fn main() {
    let ids: Vec<u32> = (0..100_000).collect();
    let results = fetch_all(ids).await;
    println!("{}", results.len());
}
