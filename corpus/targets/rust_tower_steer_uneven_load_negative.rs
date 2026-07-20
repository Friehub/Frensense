use tower::steer::Steer;
use tower::{service_fn, Service};
use std::convert::Infallible;
use std::sync::atomic::{AtomicUsize, Ordering};

#[tokio::main]
async fn main() {
    let backends: Vec<_> = (0..3)
        .map(|id| {
            service_fn(move |_: String| {
                let id = id;
                async move { Ok::<_, Infallible>(format!("backend-{id}")) }
            })
        })
        .collect();

    let counter = AtomicUsize::new(0);
    let mut steer = Steer::new(backends, |_req: &String| -> usize {
        // SAFE: Round-robin distribution spreads load across all backends.
        counter.fetch_add(1, Ordering::Relaxed) % 3
    });

    for i in 0..100 {
        let rsp = steer.call(format!("req-{i}")).await.unwrap();
        println!("{rsp}");
    }
}
