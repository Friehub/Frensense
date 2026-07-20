// [frensense]
// observation: A Tower service stack is created without a timeout middleware. If the inner service hangs (e.g., waiting on a slow database or external API), the caller blocks indefinitely, consuming a worker thread.
// impact: Connection exhaustion, resource starvation, and denial-of-service. A single hung backend can tie up all server threads, making the service unresponsive.
// improvement: Wrap the inner service with `tower::timeout::TimeoutLayer` or `tokio::time::timeout` to bound the maximum duration per request.

use tower::Service;
use std::task::{Context, Poll};
use std::future::Future;
use std::pin::Pin;

pub struct NoTimeoutService<S> {
    inner: S,
}

impl<S, Request> Service<Request> for NoTimeoutService<S>
where
    S: Service<Request>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        self.inner.call(req)
    }
}
