// [frensense]
// observation: A `tower::buffer::Buffer` is created without a bound on its capacity (e.g., `Buffer::new(inner, usize::MAX)` or using the default that grows unboundedly). When requests arrive faster than the inner service can process them, the buffer grows without limit.
// impact: Out-of-memory crash. An attacker can trivially exhaust server memory by sending a flood of requests, causing the process to be killed by the OOM killer.
// improvement: Always set a finite, reasonable bound on the buffer capacity. Monitor back-pressure with `poll_ready`.

use tower::buffer::Buffer;
use tower::Service;

pub fn create_pipeline<S>(svc: S) -> Buffer<S, S::Request>
where
    S: Service<S::Request> + Send + 'static,
    S::Future: Send,
{
    Buffer::new(svc, usize::MAX)
}

pub async fn run_unbounded() {
    let svc = tower::service_fn(|req: String| async move { Ok::<_, String>(req) });
    let mut buf = create_pipeline(svc);
    for i in 0..1_000_000 {
        let _ = buf.call(format!("req-{}", i)).await;
    }
}
