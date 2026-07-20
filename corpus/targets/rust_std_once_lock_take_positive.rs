// [frensense]
// observation: `OnceLock::take()` removes the value from the cell, leaving it empty for all subsequent callers. Code calls `take()` multiple times assuming the value persists.
// impact: The second and later callers receive `None`, causing panics from unwrap or silent fallback to broken behavior. In concurrent contexts, a racing consumer can permanently deprive other consumers of the initialized value.
// improvement: Use `OnceLock::get()` for read-only access, or reinitialize the cell after `take()` if single-consumer semantics are intended.

use std::sync::OnceLock;

static CONFIG: OnceLock<String> = OnceLock::new();

fn get_config() -> &'static str {
    CONFIG.get().map(String::as_str).unwrap_or("default")
}

fn load_config() -> Option<String> {
    Some("db://prod".into())
}

fn reset_config() {
    if let Some(val) = load_config() {
        CONFIG.set(val).ok();
    }
}

fn main() {
    CONFIG.set("db://staging".into()).ok();
    reset_config();
    println!("{}", get_config());
}
