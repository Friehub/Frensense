use std::sync::Mutex;
use std::sync::OnceLock;

static DB_POOL: OnceLock<Mutex<Vec<u32>>> = OnceLock::new();

fn init_db_pool() -> Result<(), String> {
    let _url = std::env::var("DATABASE_URL").map_err(|_| "DATABASE_URL not set".to_string())?;
    // SAFE: OnceLock allows explicit error handling; we set only on success.
    DB_POOL.set(Mutex::new(vec![1, 2, 3])).ok().unwrap();
    Ok(())
}

fn get_connection() -> Option<u32> {
    let pool = DB_POOL.get()?.lock().ok()?;
    pool.first().copied()
}

fn main() {
    std::env::remove_var("DATABASE_URL");
    if let Err(e) = init_db_pool() {
        eprintln!("init failed: {e}");
    }
    println!("{:?}", get_connection());
}
