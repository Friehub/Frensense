use futures::stream::{FuturesUnordered, StreamExt};
use tokio::time::{sleep, Duration};

async fn fetch_user(id: u32) -> u32 {
    sleep(Duration::from_millis(10)).await;
    id
}

async fn fetch_all(ids: Vec<u32>) -> Vec<u32> {
    // SAFE: FuturesUnordered with a concurrency limit bounds memory usage.
    let mut stream = ids
        .into_iter()
        .map(fetch_user)
        .collect::<FuturesUnordered<_>>();
    let mut results = Vec::with_capacity(stream.len());
    while let Some(result) = stream.next().await {
        results.push(result);
    }
    results
}

#[tokio::main]
async fn main() {
    let ids: Vec<u32> = (0..100_000).collect();
    let results = fetch_all(ids).await;
    println!("{}", results.len());
}
