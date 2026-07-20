// [frensense]
// observation: `std::cell::OnceCell::set` or `OnceLock::set` is called a second time without checking the return value. The second call panics because the cell is already initialized.
// impact: A double-init panic crashes the program. In async or concurrent contexts, this is a denial-of-service vector if an attacker can trigger re-initialization.
// improvement: Check the `Result` returned by `set`, or use `get_or_init` which handles the already-initialized case gracefully.

use std::sync::OnceLock;

static CONFIG: OnceLock<String> = OnceLock::new();

fn init_config(value: String) {
    CONFIG.set(value).unwrap();
}

fn reload_config(value: String) {
    CONFIG.set(value).unwrap();
}

fn main() {
    init_config("first".into());
    reload_config("second".into());
    println!("{}", CONFIG.get().unwrap());
}
