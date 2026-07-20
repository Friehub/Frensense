// SAFE alternative: structured concurrency with JoinSet
use tokio::task::JoinSet;

async fn process_batch(items: Vec<Item>) {
    let mut set = JoinSet::new();
    for item in items {
        set.spawn(async move { process_item(item).await });
    }
    while let Some(result) = set.join_next().await {
        match result {
            Ok(Ok(())) => {},
            Ok(Err(e)) => tracing::error!("task returned error: {e}"),
            Err(e) => tracing::error!("task panicked: {e}"),
        }
    }
}
