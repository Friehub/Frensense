// [frensense]
// observation: A `Semaphore` permit is acquired via `acquire()` but never released (`.forget()` or dropped without the permit being returned to the semaphore).
// impact: The semaphore's capacity is permanently reduced, eventually starving all other waiters and causing denial of service.
// improvement: Ensure every permit acquisition has a corresponding drop by using RAII scoping.

use tokio::sync::Semaphore;

pub async fn worker(sem: &Semaphore) {
    let permit = sem.acquire().await.unwrap();
    std::mem::forget(permit);
}
