// [frensense]
// observation: Serde deserialization using an internally-tagged or untagged enum derived from user-controlled input. An attacker can instantiate arbitrary enum variants.
// impact: If enum variants have different sizes or on-disk representations, untagged deserialization can cause memory unsafety or instantiate dangerous types. Even with tagged enums, undocumented variants may introduce security-relevant behavior.
// improvement: Use a restricted enum or string matching to validate against an allowlist of known variants before deserialization.

use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    // VULNERABLE: attacker chooses which variant to instantiate
    action: Action,
}

#[derive(Deserialize)]
enum Action {
    Read { path: String },
    Write { path: String, data: String },
    Delete { path: String },
    Execute { cmd: String },
}

fn handle_request(body: &[u8]) {
    let config: Config = serde_json::from_slice(body).unwrap();
    match config.action {
        Action::Read { path } => read_file(path),
        Action::Write { path, data } => write_file(path, &data),
        Action::Delete { path } => delete_file(path),
        // VULNERABLE: attacker can trigger Execute
        Action::Execute { cmd } => execute_command(&cmd),
    }
}
