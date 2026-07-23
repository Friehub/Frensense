use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tower::Service;
use tokio::time::timeout;

pub struct TimeoutService<S> {
    inner: S,
    max_dur: Duration,
}

impl<S, Request> Service<Request> for TimeoutService<S>
where
    S: Service<Request>,
    S::Future: Send + 'static,
    S::Error: Send + Sync + 'static,
    S::Response: Send + 'static,
{
    type Response = Result<S::Response, Box<dyn std::error::Error + Send + Sync>>;
    type Error = Box<dyn std::error::Error + Send + Sync>;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(|e| Box::new(e) as _)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let fut = self.inner.call(req);
        let dur = self.max_dur;
        Box::pin(async move {
            // SAFE: Every call is bounded by a timeout to prevent hangs.
            match timeout(dur, fut).await {
                Ok(Ok(resp)) => Ok(Ok(resp)),
                Ok(Err(e)) => Err(Box::new(e) as _),
                Err(_) => Err("request timed out".into()),
            }
        })
    }
}
