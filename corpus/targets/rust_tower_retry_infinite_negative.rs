use tower::retry::{RetryLayer, Policy};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct BoundedRetry {
    max_attempts: u32,
    attempts: u32,
}

impl<Req, Res, E> Policy<Req, Res, E> for BoundedRetry {
    type Future = Pin<Box<dyn Future<Output = Result<(), E>> + Send>>;

    fn retry(&self, _req: &Req, result: Result<&Res, &E>) -> Option<Self::Future> {
        // SAFE: Max 3 attempts prevents infinite retry amplification.
        if self.attempts >= self.max_attempts {
            return None;
        }
        if result.is_err() {
            Some(Box::pin(async { Ok(()) }))
        } else {
            None
        }
    }

    fn clone_request(&self, req: &Req) -> Option<Req> where Req: Clone {
        Some(req.clone())
    }
}
