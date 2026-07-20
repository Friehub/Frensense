// [frensense]
// observation: `tokio::task::spawn_blocking` is called in a loop or on every request without any limit. When the blocking thread pool is saturated, tasks queue up unboundedly, consuming memory.
// impact: Under load, the spawn_blocking queue grows without bound. An attacker can trigger many blocking operations (e.g., CPU-bound or I/O-bound tasks) to exhaust server memory (OOM).
// improvement: Use a semaphore to limit the number of in-flight blocking tasks, or offload to a dedicated thread pool with a bounded work queue.

use tokio::task::spawn_blocking;

async fn handle_request(id: u32) {
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
