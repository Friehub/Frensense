// [frensense]
// observation: `compile_error!` is gated on an environment variable (via `env!` or `option_env!`) that may not be defined, causing a hard compile error when the variable is missing during a build that encounters this code path.
// impact: Innocent contributors or CI configurations that do not set the environment variable cannot compile the crate. This is especially frustrating when the `compile_error!` is deep in a dependency or behind a feature flag, and the error message does not explain how to fix it.
// improvement: Use `option_env!` and issue a warning instead of a hard error, or provide a clear error message with instructions for setting the variable.

#[cfg(feature = "feature_x")]
compile_error!("FEATURE_X_VERSION env var is not set correctly");

pub fn version_check() -> &'static str {
    env!("FEATURE_X_VERSION")
}
