use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tower::{Layer, Service};
use governor::{Quota, RateLimiter, clock::DefaultClock, state::keyed::DefaultKeyedStateStore, middleware::NoOpMiddleware};
use std::net::SocketAddr;
use std::num::NonZeroU32;

#[derive(Clone)]
pub struct RateLimitLayer {
    limiter: RateLimiter<SocketAddr, DefaultKeyedStateStore<SocketAddr>, DefaultClock>,
}

impl RateLimitLayer {
    pub fn new(max_burst: u32, period: Duration) -> Self {
        let quota = Quota::with_period(period).unwrap().allow_burst(NonZeroU32::new(max_burst).unwrap());
        Self {
            limiter: RateLimiter::keyed(quota),
        }
    }
}

impl<S> Layer<S> for RateLimitLayer {
    type Service = RateLimitedService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RateLimitedService {
            inner,
            limiter: self.limiter.clone(),
        }
    }
}

pub struct RateLimitedService<S> {
    inner: S,
    limiter: RateLimiter<SocketAddr, DefaultKeyedStateStore<SocketAddr>, DefaultClock>,
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
        // SAFE: governor rate limiter enforces per-client-IP quotas.
        let allowed = self.limiter.check();
        if allowed.is_err() {
            return Box::pin(async { Err("rate limit exceeded".into()) });
        }
        Box::pin(self.inner.call(req).map_err(|e| Box::new(e) as _))
    }
}
