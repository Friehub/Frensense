use tower::{service_fn, limit::ConcurrencyLimit};
use std::convert::Infallible;

#[tokio::main]
async fn main() {
    let svc = service_fn(|_: String| async { Ok::<_, Infallible>("done".into()) });
    // SAFE: `tower::limit::ConcurrencyLimit` uses an internal semaphore with atomic
    // acquire/release, preventing the TOCTOU race in the positive case.
    let mut limited = ConcurrencyLimit::new(svc, 5);
    for _ in 0..10 {
        limited.call("req".into()).await.unwrap();
    }
}
