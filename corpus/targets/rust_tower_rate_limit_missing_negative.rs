use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tower::Service;
use tokio::sync::Semaphore;

pub struct RateLimitedService<S> {
    inner: S,
    semaphore: Semaphore,
}

impl<S, Request> Service<Request> for RateLimitedService<S>
where
    S: Service<Request>,
{
    type Response = S::Response;
    type Error = Box<dyn std::error::Error + Send + Sync>;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(|e| Box::new(e) as _)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let permit = self.semaphore.clone().try_acquire_owned();
        // SAFE: Rate limit enforces a maximum number of concurrent requests.
        if permit.is_err() {
            return Box::pin(async { Err("rate limit exceeded".into()) });
        }
        Box::pin(async move {
            let _permit = permit.unwrap();
            self.inner.call(req).await.map_err(|e| Box::new(e) as _)
        })
    }
}
