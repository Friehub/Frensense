use tokio::select;
use tokio::pin;

pub async fn race() {
    let fut = async {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        42
    };
    pin!(fut);
    select! {
        v = &mut fut => println!("{}", v),
    }
}
