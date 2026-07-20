use std::sync::OnceLock;
use std::sync::Mutex;

static CONFIG: OnceLock<Mutex<String>> = OnceLock::new();

fn get_config() -> String {
    let lock = CONFIG.get_or_init(|| Mutex::new("default".into()));
    lock.lock().unwrap().clone()
}

fn update_config(val: String) {
    let lock = CONFIG.get_or_init(|| Mutex::new(val.clone()));
    let mut guard = lock.lock().unwrap();
    *guard = val;
}

fn main() {
    update_config("db://prod".into());
    println!("{}", get_config());
}
