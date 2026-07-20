// SAFE: store handles and await them or use JoinSet
async fn process_batch(items: Vec<Item>) {
    let mut handles = Vec::new();
    for item in items {
        handles.push(tokio::spawn(async move {
            process_item(item).await;
        }));
    }
    for handle in handles {
        if let Err(e) = handle.await {
            tracing::error!("task failed: {e}");
        }
    }
}

async fn handle_connection(stream: TcpStream) {
    let handle = tokio::spawn(handle_client(stream));
    tokio::spawn(async move {
        if let Err(e) = handle.await {
            tracing::error!("client handler failed: {e}");
        }
    });
}
