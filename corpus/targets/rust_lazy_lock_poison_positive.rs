// [frensense]
// observation: `std::sync::LazyLock` initialization panics, poisoning the lazy for all threads. Every subsequent access will also panic, since `LazyLock` does not retry initialization.
// impact: A transient failure during initialization permanently denies service to all consumers. Unlike `OnceLock`, there is no way to reset or recover — the program must restart.
// improvement: Ensure the init closure is infallible, or use `OnceLock` with explicit error handling and retry logic.

use std::sync::LazyLock;
use std::sync::Mutex;

static DB_POOL: LazyLock<Mutex<Vec<u32>>> = LazyLock::new(|| {
    if std::env::var("DATABASE_URL").is_err() {
        panic!("DATABASE_URL not set");
    }
    Mutex::new(vec![1, 2, 3])
});

fn get_connection() -> u32 {
    let pool = DB_POOL.lock().unwrap();
    pool.first().copied().unwrap()
}

fn main() {
    std::env::remove_var("DATABASE_URL");
    println!("{}", get_connection());
}
