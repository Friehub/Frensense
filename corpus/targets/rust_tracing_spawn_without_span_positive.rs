// [frensense]
// observation: A tokio::spawn call inside a traced context does not propagate the current tracing span to the spawned task, losing the parent span context in logs.
// impact: Logs from spawned tasks cannot be correlated with the parent request or operation, making debugging and observability significantly harder in async applications.
// improvement: Use tracing::Instrument to attach the current span (or a new child span) to spawned tasks.

use tokio::spawn;
use tracing::info;

async fn handle_request(req_id: u64) {
    info!("processing request");
    spawn(async {
        info!("background task for request");
    });
}

async fn process_batch() {
    let _span = tracing::info_span!("batch_processor").entered();
    spawn(async {
        info!("processing batch item");
    });
}
