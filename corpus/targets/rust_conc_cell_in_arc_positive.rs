// [frensense]
// observation: A `std::cell::Cell<T>` is wrapped in `Arc` and shared across threads in an async context. `Cell` implements `Send` but not `Sync`, yet `Arc<T>` provides `Sync` only when `T: Sync`. Since `Cell` is !Sync, `Arc<Cell<T>>` is !Sync, preventing compile-time detection of cross-thread use. However, `Cell` is not thread-safe — concurrent `set`/`get` from different tasks causes data races.
// impact: Undefined behavior: concurrent reads and writes to the same `Cell` from different threads produce torn values, non-atomic updates, and memory corruption. This can manifest as silent data corruption that is extremely hard to debug.
// improvement: Use `Arc<Mutex<T>>`, `Arc<RwLock<T>>`, or `Arc<AtomicU64>` instead. Never share `Cell` across thread boundaries.

use std::cell::Cell;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let counter = Arc::new(Cell::new(0u64));
    let c2 = counter.clone();

    let t1 = tokio::spawn(async move {
        for _ in 0..10000 {
            c2.set(c2.get() + 1);
        }
    });

    let t2 = tokio::spawn(async move {
        for _ in 0..10000 {
            counter.set(counter.get() + 1);
        }
    });

    let _ = tokio::join!(t1, t2);
}
