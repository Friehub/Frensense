// SAFE: Uses `option_env!` to gracefully handle unset env vars.
pub fn version_check() -> Option<&'static str> {
    option_env!("FEATURE_X_VERSION")
}
