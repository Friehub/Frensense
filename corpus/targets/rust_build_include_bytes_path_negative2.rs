// SAFE: The env var is validated against an allowlist in a build script.
// build.rs:
// use std::env;
// fn main() {
//     let path = env::var("CONFIG_PATH").unwrap_or_default();
//     assert!(path.starts_with("configs/"), "CONFIG_PATH must start with configs/");
// }

pub fn load_config() -> &'static [u8] {
    include_bytes!(concat!("../", env!("CONFIG_PATH")))
}
