// [frensense]
// observation: A `Barrier::wait()` call is wrapped with a timeout. If the timeout fires, some threads proceed while others remain blocked, breaking barrier synchronization guarantees.
// impact: Partial synchronization — threads that timed out may access shared state before others are ready, causing data races or logic errors.
// improvement: Do not use timeouts with barriers, or use a fallback that resets all participants.

use std::sync::{Barrier, Arc};
use std::time::Duration;

pub fn sync_workers(barrier: Arc<Barrier>) {
    let _ = barrier.wait_timeout(Duration::from_millis(100));
}
