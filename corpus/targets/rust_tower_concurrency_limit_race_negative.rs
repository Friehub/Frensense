use std::sync::Arc;
use tower::{service_fn, limit::ConcurrencyLimit};
use tokio::sync::Semaphore;

#[derive(Clone)]
struct SafeLimiter {
    semaphore: Arc<Semaphore>,
}

impl SafeLimiter {
    fn new(max: usize) -> Self {
        Self { semaphore: Arc::new(Semaphore::new(max)) }
    }
}

impl<S, Req> tower::Layer<S> for SafeLimiter
where
    S: tower::Service<Req>,
    S::Future: Send + 'static,
    Req: Send + 'static,
{
    type Service = tower::util::BoxService<Req, S::Response, S::Error>;

    fn layer(&self, service: S) -> Self::Service {
        let sem = self.semaphore.clone();
        let svc = service_fn(move |req: Req| {
            let sem = sem.clone();
            let inner = service.call(req);
            async move {
                let _permit = sem.acquire().await.unwrap();
                inner.await
            }
        });
        tower::util::BoxService::new(svc)
    }
}

#[tokio::main]
async fn main() {
    let svc = service_fn(|_: String| async { Ok::<_, String>("done".into()) });
    // SAFE: `Semaphore::acquire` is atomic; check and decrement happen together.
    let limited = SafeLimiter::new(5);
    let mut svc = limited.layer(svc);
    for _ in 0..10 {
        svc.call("req".into()).await.unwrap();
    }
}
