use tokio::sync::Semaphore;
use tokio::task::spawn_blocking;

static MAX_BLOCKING: Semaphore = Semaphore::const_new(64);

async fn handle_request(id: u32) {
    // SAFE: Semaphore bounds the number of in-flight blocking tasks, preventing unbounded queue growth.
    let _permit = MAX_BLOCKING.acquire().await.unwrap();
    spawn_blocking(move || {
        std::thread::sleep(std::time::Duration::from_secs(10));
        println!("processed {id}");
    })
    .await
    .unwrap();
}

#[tokio::main]
async fn main() {
    for i in 0..10_000u32 {
        tokio::spawn(handle_request(i));
    }
}
