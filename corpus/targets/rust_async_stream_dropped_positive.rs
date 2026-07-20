// [frensense]
// observation: An item yielded by a `tokio_stream::Stream` is dropped without being processed or awaited, so the side-effect in the stream's production is lost.
// impact: Lost data or unprocessed events — the stream item may represent a connection, a message, or a file handle that is silently discarded.
// improvement: Process every item from the stream in the loop body; do not discard items.

use tokio_stream::StreamExt;

pub async fn consume(stream: impl tokio_stream::Stream<Item = String> + Unpin) {
    let _ = stream.next().await;
}
