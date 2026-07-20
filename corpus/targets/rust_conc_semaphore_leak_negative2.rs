// SAFE: Permit is explicitly dropped via owned scope to ensure release.
use tokio::sync::Semaphore;

pub async fn do_work(sem: &Semaphore) {
    let permit = sem.acquire().await.unwrap();
    perform_io().await;
    drop(permit);
}

async fn perform_io() {}
