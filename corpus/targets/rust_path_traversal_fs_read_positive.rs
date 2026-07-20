// [frensense]
// observation: A user-controlled path is joined with a base directory and passed to std::fs::read_to_string or std::fs::read without checking for path traversal.
// impact: An attacker can read arbitrary files on the filesystem by providing "../etc/passwd" or symlink paths.
// improvement: Canonicalize the path and verify it stays within the allowed base directory before reading.

use std::fs;
use std::path::Path;

fn read_user_file(base_dir: &str, user_path: &str) -> Result<String, std::io::Error> {
    let full_path = Path::new(base_dir).join(user_path);
    fs::read_to_string(&full_path)
}

fn get_config(dir: &str, file: &str) -> Result<Vec<u8>, std::io::Error> {
    let path = Path::new(dir).join(file);
    fs::read(&path)
}
