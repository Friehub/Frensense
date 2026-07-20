// [frensense]
// observation: `tower::steer::Steer` distributes requests across backends using a round-robin or `usize`-returning closure. When backends have different capacities, a naive uniform distribution (e.g., `i % n`) sends equal traffic to all backends regardless of their ability to handle it.
// impact: Faster or less-loaded backends get the same traffic as slower ones, causing latency spikes on overcapacity backends while others sit idle. This defeats horizontal scaling — the system is only as fast as the slowest backend.
// improvement: Use weighted distribution (e.g., `pending_requests` count per backend) or a proper load balancer like `tower::balance::p2c::Balance`.

use tower::steer::Steer;
use tower::{service_fn, Service};
use std::convert::Infallible;

async fn backend_task(id: u32) -> String {
    format!("backend-{id}")
}

#[tokio::main]
async fn main() {
    let backends: Vec<_> = (0..3)
        .map(|id| {
            service_fn(move |_: String| {
                let id = id;
                async move { Ok::<_, Infallible>(backend_task(id).await) }
            })
        })
        .collect();

    let mut steer = Steer::new(backends, |_req: &String| -> usize {
        0 // Always sends to first backend, uneven load
    });

    for i in 0..100 {
        let rsp = steer.call(format!("req-{i}")).await.unwrap();
        println!("{rsp}");
    }
}
