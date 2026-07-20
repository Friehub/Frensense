use tokio_stream::StreamExt;

pub async fn consume(stream: impl tokio_stream::Stream<Item = String> + Unpin) {
    while let Some(item) = stream.next().await {
        println!("processing: {}", item);
    }
}
