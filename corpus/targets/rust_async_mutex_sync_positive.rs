// [frensense]
// observation: tokio::sync::Mutex is used inside a synchronous context (not .await) or held across a function boundary where the lock guard is not held for the right scope, effectively blocking the entire runtime thread.
// impact: The tokio::sync::Mutex::blocking_lock is called from an async context, which can cause deadlock by blocking the tokio runtime thread.
// improvement: Use std::sync::Mutex for short critical sections, or ensure tokio::sync::Mutex is only used with .await.

use tokio::sync::Mutex;
use std::sync::Arc;

async fn update_counter(counter: Arc<Mutex<i32>>) -> i32 {
    counter.blocking_lock().clone()
}

fn sync_update(counter: Arc<Mutex<Vec<u8>>>) {
    let mut guard = counter.blocking_lock();
    guard.push(42);
}

async fn async_fn_uses_blocking(counter: Arc<Mutex<i32>>) {
    let val = counter.blocking_lock();
    println!("{}", *val);
}
