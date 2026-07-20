use tower::{service_fn, Service};
use tower::balance::p2c::Balance;
use tower::discover::ServiceList;
use std::convert::Infallible;

#[tokio::main]
async fn main() {
    let backends: Vec<_> = (0..3)
        .map(|id| {
            service_fn(move |_: String| {
                async move { Ok::<_, Infallible>(format!("backend-{id}")) }
            })
        })
        .collect();

    // SAFE: Power-of-two-choices load balancer distributes requests based on
    // each backend's `poll_ready` state, avoiding the uneven distribution of Steer.
    let mut balancer = Balance::new(ServiceList::new(backends));

    for i in 0..100 {
        let rsp = balancer.call(format!("req-{i}")).await.unwrap();
        println!("{rsp}");
    }
}
