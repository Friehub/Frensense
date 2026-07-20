use tokio::sync::Semaphore;

pub async fn worker(sem: &Semaphore) {
    let _permit = sem.acquire().await.unwrap();
}
