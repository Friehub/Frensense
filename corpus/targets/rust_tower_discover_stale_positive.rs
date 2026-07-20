// [frensense]
// observation: A `tower::discover::Change` stream is consumed once at startup and never refreshed. The discovered service list becomes stale when backends are added or removed — the load balancer continues routing to dead endpoints while ignoring healthy new ones.
// impact: Traffic is routed to endpoints that no longer exist (connection errors) or are unhealthy, while new or healthy endpoints remain idle. This causes partial or complete service degradation during rolling deployments or auto-scaling events.
// improvement: Periodically re-discover backends, use a service mesh with health-check-based discovery, or wrap each endpoint with a circuit breaker.

use tower::discover::{Change, ServiceList};
use tower::service_fn;
use std::convert::Infallible;

#[tokio::main]
async fn main() {
    let backends = vec![
        service_fn(|_: String| async { Ok::<_, Infallible>("backend-a".into()) }),
        service_fn(|_: String| async { Ok::<_, Infallible>("backend-b".into()) }),
    ];

    let discovered = vec![
        Change::Insert("a", backends[0].clone()),
        Change::Insert("b", backends[1].clone()),
    ];

    let mut balancer = tower::balance::p2c::Balance::new(
        ServiceList::from(discovered.into_iter())
    );

    // Even if "a" goes down, the service list is never updated
    for i in 0..10 {
        let rsp = balancer.call(format!("req-{i}")).await.unwrap();
        println!("{rsp}");
    }
}
