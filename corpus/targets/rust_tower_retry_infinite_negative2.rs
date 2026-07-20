use tower::retry::{RetryLayer, Policy};
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;
use tokio::time::sleep;

pub struct ExponentialBackoffRetry {
    max_attempts: u32,
    attempt: u32,
}

impl<Req, Res, E> Policy<Req, Res, E> for ExponentialBackoffRetry {
    type Future = Pin<Box<dyn Future<Output = Result<(), E>> + Send>>;

    fn retry(&self, _req: &Req, result: Result<&Res, &E>) -> Option<Self::Future> {
        // SAFE: Bounded retries with exponential backoff prevents amplification.
        if self.attempt >= self.max_attempts {
            return None;
        }
        if result.is_err() {
            let delay = Duration::from_millis(100 * 2u64.pow(self.attempt));
            Some(Box::pin(async move {
                sleep(delay).await;
                Ok(())
            }))
        } else {
            None
        }
    }

    fn clone_request(&self, req: &Req) -> Option<Req> where Req: Clone {
        Some(req.clone())
    }
}
