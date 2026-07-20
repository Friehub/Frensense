// [frensense]
// observation: `std::cell::Cell` (which is `!Sync`) is shared across an `.await` boundary via `&` reference. The `Cell`'s non-atomic reads and writes race when the same cell is accessed from multiple tasks.
// impact: Lost updates, torn reads, and logical data races. Two concurrent tasks can interleave their `get`/`set` operations, causing one task's increment to overwrite another's.
// improvement: Replace `Cell` with `AtomicU32`/`AtomicU64` for concurrent access, or use `Mutex`/`RwLock` if the value is complex.

use std::cell::Cell;

async fn inc(counter: &Cell<u32>) -> u32 {
    let current = counter.get();
    tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
    counter.set(current + 1);
    current
}

#[tokio::main]
async fn main() {
    let counter = Cell::new(0u32);
    let (a, b) = tokio::join!(inc(&counter), inc(&counter));
    println!("{a} {b} final={}", counter.get());
}
