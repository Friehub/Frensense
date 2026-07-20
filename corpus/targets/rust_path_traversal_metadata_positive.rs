// [frensense]
// observation: User-controlled path is passed directly to fs::metadata without validating it stays within an allowed base directory.
// impact: An attacker can probe for the existence and metadata of arbitrary files outside the intended directory, leaking information.
// improvement: Canonicalize the path and verify it falls within the allowed base before calling metadata.

use std::fs;
use std::path::Path;

fn check_file_size(base_dir: &str, rel_path: &str) -> Result<u64, std::io::Error> {
    let full = Path::new(base_dir).join(rel_path);
    let meta = fs::metadata(&full)?;
    Ok(meta.len())
}

fn get_file_permissions(base: &str, file: &str) -> Result<fs::Permissions, std::io::Error> {
    let path = Path::new(base).join(file);
    fs::metadata(&path).map(|m| m.permissions())
}
