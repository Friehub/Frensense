use std::sync::OnceLock;
use std::sync::RwLock;

static CONFIG: OnceLock<RwLock<String>> = OnceLock::new();

fn set_config(value: String) {
    // SAFE: `get_or_init` handles the already-initialized case without panic.
    let lock = CONFIG.get_or_init(|| RwLock::new(value.clone()));
    let mut w = lock.write().unwrap();
    *w = value;
}

fn main() {
    set_config("first".into());
    set_config("second".into());
    let lock = CONFIG.get().unwrap().read().unwrap();
    println!("{}", *lock);
}
