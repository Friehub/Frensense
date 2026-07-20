use std::sync::OnceLock;

static CONFIG: OnceLock<String> = OnceLock::new();

fn get_config() -> &'static str {
    CONFIG.get().map(String::as_str).unwrap_or("default")
}

fn load_config() -> Option<String> {
    Some("db://prod".into())
}

fn reset_config() {
    // SAFE: Use `get_or_init` instead of `take` to ensure the value persists.
    let _ = CONFIG.get_or_init(|| load_config().unwrap());
}

fn main() {
    CONFIG.set("db://staging".into()).ok();
    reset_config();
    println!("{}", get_config());
}
