// SAFE: Every stream item is collected and processed via for_each.
use futures::StreamExt;

pub async fn consume(stream: impl futures::Stream<Item = String> + Unpin) {
    stream.for_each(|item| async move {
        println!("processing: {}", item);
    }).await;
}
