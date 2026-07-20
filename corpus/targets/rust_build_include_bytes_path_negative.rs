// SAFE: Path is relative to the crate's own source tree, not from an env var.
pub fn load_config() -> &'static [u8] {
    include_bytes!("../config/default.toml")
}
