// [frensense]
// observation: A Tonic gRPC streaming response handler shares mutable state (e.g., `Arc<Mutex<Vec>>`) between concurrent streaming tasks without synchronization that accounts for the non-deterministic interleaving of stream messages. Multiple clients' streams are handled concurrently, and the shared state is accessed without a per-stream or per-message lock discipline.
// impact: Data races on the shared state — concurrent reads and writes can produce corrupted data, panic (due to mutex poison), or silently drop messages. Under load, this can cause incorrect billing, missed events, or inconsistent state.
// improvement: Use per-stream state, or protect shared state with proper synchronization (e.g., `tokio::sync::RwLock`), or use actors/channels to serialize access.

use tokio::sync::Mutex;
use std::sync::Arc;
use tonic::transport::Server;
use tonic::{Request, Streaming, Status, Response};
use hello::counter_server::{CounterServer, Counter};
use hello::{CounterRequest, CounterResponse};

pub mod hello {
    tonic::include_proto!("hello");
}

#[derive(Default)]
pub struct CounterService {
    counts: Arc<Mutex<Vec<u64>>>,
}

#[tonic::async_trait]
impl Counter for CounterService {
    async fn stream_counts(&self, _req: Request<Streaming<CounterRequest>>) -> Result<Response<Self::stream_countsStream>, Status> {
        unimplemented!()
    }

    type stream_countsStream = CounterResponse;
}

use tokio_stream::wrappers::ReceiverStream;
use tokio::sync::mpsc;

impl CounterService {
    pub async fn handle_stream(&self, mut stream: Streaming<CounterRequest>) -> Result<Response<ReceiverStream<Result<CounterResponse, Status>>>, Status> {
        let counts = self.counts.clone();
        let (tx, rx) = mpsc::channel(1024);
        tokio::spawn(async move {
            while let Some(req) = stream.message().await.unwrap_or(None) {
                let mut guard = counts.lock().await;
                guard.push(req.value);
                let total: u64 = guard.iter().sum();
                let _ = tx.send(Ok(CounterResponse { total })).await;
            }
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
