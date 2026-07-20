use tokio::sync::Mutex;
use std::sync::Arc;
use tonic::transport::Server;
use tonic::{Request, Streaming, Status, Response};
use hello::counter_server::{CounterServer, Counter};
use hello::{CounterRequest, CounterResponse};
use tokio_stream::wrappers::ReceiverStream;
use tokio::sync::mpsc;

pub mod hello {
    tonic::include_proto!("hello");
}

#[derive(Default)]
pub struct CounterService {
    counts: Arc<tokio::sync::RwLock<Vec<u64>>>,
}

#[tonic::async_trait]
impl Counter for CounterService {
    type stream_countsStream = ReceiverStream<Result<CounterResponse, Status>>;

    async fn stream_counts(&self, _req: Request<Streaming<CounterRequest>>) -> Result<Response<Self::stream_countsStream>, Status> {
        unimplemented!()
    }
}

impl CounterService {
    pub async fn handle_stream(&self, mut stream: Streaming<CounterRequest>) -> Result<Response<ReceiverStream<Result<CounterResponse, Status>>>, Status> {
        let counts = self.counts.clone();
        let (tx, rx) = mpsc::channel(1024);
        tokio::spawn(async move {
            while let Some(req) = stream.message().await.unwrap_or(None) {
                // SAFE: RwLock allows concurrent reads with exclusive writes.
                let mut guard = counts.write().await;
                guard.push(req.value);
                let total: u64 = guard.iter().sum();
                let _ = tx.send(Ok(CounterResponse { total })).await;
            }
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
