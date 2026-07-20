use tower::{service_fn, Service};
use tower::balance::p2c::Balance;
use tower::discover::Change;
use std::convert::Infallible;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

type Endpoints = Arc<Mutex<HashMap<String, tower::util::BoxService<String, String, Infallible>>>>;

#[tokio::main]
async fn main() {
    let endpoints: Endpoints = Arc::new(Mutex::new(HashMap::new()));

    // Seed initial endpoints.
    {
        let mut map = endpoints.lock().await;
        map.insert("a".into(), tower::util::BoxService::new(
            service_fn(|_: String| async { Ok::<_, Infallible>("backend-a".into()) })
        ));
    }

    // SAFE: Periodically rebuild the `ServiceList` from the current endpoint map.
    let watch_endpoints = endpoints.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            let map = watch_endpoints.lock().await;
            let backends: Vec<_> = map.values().cloned().collect();
            let _new_balancer = Balance::new(ServiceList::new(backends));
        }
    });

    let backends: Vec<_> = {
        let map = endpoints.lock().await;
        map.values().cloned().collect()
    };
    let mut balancer = Balance::new(ServiceList::new(backends));

    for i in 0..10 {
        let rsp = balancer.call(format!("req-{i}")).await.unwrap();
        println!("{rsp}");
    }
}
