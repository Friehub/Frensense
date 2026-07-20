// SAFE: Uses tokio::task::Builder with a span set, or creates an explicit child span for each spawned task.

use tokio::spawn;
use tracing::{info, info_span, Instrument};

async fn handle_request(req_id: u64) {
    let child_span = info_span!("request_worker", req_id);
    spawn(
        async {
            info!("background task for request");
        }
        .instrument(child_span),
    );
}
