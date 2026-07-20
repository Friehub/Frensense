// SAFE: The spawned task is instrumented with the current span using tracing::Instrument, preserving the parent context.

use tokio::spawn;
use tracing::{info, Instrument};

async fn handle_request(req_id: u64) {
    info!("processing request");
    spawn(
        async {
            info!("background task for request");
        }
        .instrument(tracing::Span::current()),
    );
}

async fn process_batch() {
    let span = tracing::info_span!("batch_processor");
    spawn(
        async {
            info!("processing batch item");
        }
        .instrument(span),
    );
}
