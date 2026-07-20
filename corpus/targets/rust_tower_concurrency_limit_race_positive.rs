// [frensense]
// observation: A concurrency limit is implemented with shared state using `Arc<Mutex<usize>>` where the increment/decrement and the actual request processing are not atomic. Two concurrent requests can both pass the capacity check before either increments the counter.
// execution: The shared counter is checked, then the request proceeds — but another thread can also check between the check and the increment, allowing more requests than the limit.
// improvement: Use `tower::limit::ConcurrencyLimit` which correctly uses a semaphore internally, or use `tokio::sync::Semaphore` for cooperative limiting.

use std::sync::{Arc, Mutex};
use tower::{Service, service_fn};
use std::task::{Poll, Context};
use std::future::Future;
use std::pin::Pin;

#[derive(Clone)]
struct Limiter {
    max: usize,
    current: Arc<Mutex<usize>>,
}

impl<S, Req> tower::Layer<S> for Limiter {
    type Service = LimitService<S>;

    fn layer(&self, service: S) -> Self::Service {
        LimitService {
            inner: service,
            max: self.max,
            current: self.current.clone(),
        }
    }
}

#[derive(Clone)]
struct LimitService<S> {
    inner: S,
    max: usize,
    current: Arc<Mutex<usize>>,
}

impl<S, Req> Service<Req> for LimitService<S>
where
    S: Service<Req>,
    S::Future: Send + 'static,
    Req: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let count = *self.current.lock().unwrap();
        if count < self.max {
            Poll::Ready(Ok(()))
        } else {
            Poll::Pending
        }
    }

    fn call(&mut self, req: Req) -> Self::Future {
        *self.current.lock().unwrap() += 1;
        let current = self.current.clone();
        let fut = self.inner.call(req);
        Box::pin(async move {
            let result = fut.await;
            *current.lock().unwrap() -= 1;
            result
        })
    }
}

#[tokio::main]
async fn main() {
    let svc = service_fn(|_: String| async { Ok::<_, String>("done".into()) });
    let limiter = Limiter { max: 5, current: Arc::new(Mutex::new(0)) };
    let mut limited = limiter.layer(svc);
    for _ in 0..10 {
        limited.call("req".into()).await.unwrap();
    }
}
