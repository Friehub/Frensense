use std::sync::LazyLock;
use std::sync::Mutex;

static DB_POOL: LazyLock<Mutex<Option<Vec<u32>>>> = LazyLock::new(|| {
    // SAFE: Using Option to represent initialization failure instead of panicking.
    match std::env::var("DATABASE_URL") {
        Ok(_) => Mutex::new(Some(vec![1, 2, 3])),
        Err(_) => Mutex::new(None),
    }
});

fn get_connection() -> Option<u32> {
    let pool = DB_POOL.lock().unwrap();
    pool.as_ref().and_then(|v| v.first().copied())
}

fn main() {
    std::env::remove_var("DATABASE_URL");
    println!("{:?}", get_connection());
}
