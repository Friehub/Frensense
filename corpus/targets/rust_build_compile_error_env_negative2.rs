// SAFE: `compile_error!` has a clear message telling the user what to set.
#[cfg(feature = "feature_x")]
const _: () = {
    if !option_env!("FEATURE_X_VERSION").is_some() {
        panic!("feature_x requires FEATURE_X_VERSION env var; set it to the semver of your deployment");
    }
};
