// SAFE: Future is Box::pinned before use in select!.
use tokio::select;

pub async fn race() {
    let fut = Box::pin(async {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        42
    });
    select! {
        v = fut => println!("{}", v),
    }
}
