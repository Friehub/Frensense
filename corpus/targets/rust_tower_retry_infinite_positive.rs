// [frensense]
// observation: A Tower retry policy is configured with an unlimited retry count (e.g., `RetryLayer::new(|| 0..)` or a policy that always returns `Attempt::Retry` without decrementing). When the downstream service fails, the retry loop runs forever.
// impact: Amplification attack — a single failing request can generate infinite retries, overwhelming the downstream service and burning CPU. In production, this can cascade into a self-inflicted DDoS.
// improvement: Set a maximum retry count (typically 2–5) with exponential backoff and jitter.

use tower::retry::{RetryLayer, Policy};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct InfiniteRetry;

impl<Req, Res, E> Policy<Req, Res, E> for InfiniteRetry {
    type Future = Pin<Box<dyn Future<Output = Result<(), E>> + Send>>;

    fn retry(&self, _req: &Req, _result: Result<&Res, &E>) -> Option<Self::Future> {
        Some(Box::pin(async { Ok(()) }))
    }

    fn clone_request(&self, req: &Req) -> Option<Req> where Req: Clone {
        None
    }
}
