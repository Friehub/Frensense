use std::sync::OnceLock;

static CONFIG: OnceLock<String> = OnceLock::new();

fn set_config(value: String) -> Result<(), String> {
    // SAFE: Checking the Result avoids panic on double-init.
    CONFIG.set(value).map_err(|_| "config already initialized".to_string())
}

fn main() {
    set_config("first".into()).ok();
    match set_config("second".into()) {
        Ok(_) => println!("set"),
        Err(e) => eprintln!("{e}"),
    }
    println!("{}", CONFIG.get().unwrap());
}
