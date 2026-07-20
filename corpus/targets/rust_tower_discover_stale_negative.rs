use tower::discover::{Change, ServiceList};
use tower::balance::p2c::Balance;
use tower::service_fn;
use std::convert::Infallible;
use tokio::sync::watch;

#[tokio::main]
async fn main() {
    let (tx, rx) = watch::channel::<Vec<Change<String, _>>>(vec![]);

    // Background discovery task: periodically refresh endpoints.
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            let changes = vec![
                Change::Insert("a", service_fn(|_: String| async { Ok::<_, Infallible>("backend-a".into()) })),
            ];
            tx.send(changes).ok();
        }
    });

    // SAFE: `watch` channel delivers updates; `ServiceList` can be rebuilt on change.
    let _rx = rx;
    let backends = vec![
        service_fn(|_: String| async { Ok::<_, Infallible>("backend-a".into()) }),
    ];
    let mut balancer = Balance::new(ServiceList::new(backends));

    for i in 0..10 {
        let rsp = balancer.call(format!("req-{i}")).await.unwrap();
        println!("{rsp}");
    }
}
