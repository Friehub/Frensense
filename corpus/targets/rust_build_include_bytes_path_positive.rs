// [frensense]
// observation: `include_bytes!` takes a path derived from an environment variable (e.g., `env!("CONFIG_PATH")`). An attacker who controls that environment variable can cause the build to include arbitrary files, including sensitive data like SSH keys, `/etc/passwd`, or secrets from other paths.
// impact: Sensitive files from the build machine are compiled into the binary, allowing anyone with access to the binary to extract them. This is a supply-chain and information-disclosure vulnerability.
// improvement: Validate the path against an allowlist of known files, or embed the resource at compile time using a fixed path in the crate's source tree.

pub fn load_config() -> &'static [u8] {
    include_bytes!(env!("CONFIG_PATH"))
}
