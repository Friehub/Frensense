// [frensense]
// observation: A user-controlled path is canonicalized with fs::canonicalize but the result is not checked to be within the allowed base directory.
// impact: An attacker can bypass path restrictions by using symlinks or absolute paths, gaining access to arbitrary files.
// improvement: After canonicalizing, verify the resulting path starts with the canonicalized base directory.

use std::fs;
use std::path::Path;

fn read_safe_file(base: &str, user_path: &str) -> Result<String, std::io::Error> {
    let full = Path::new(base).join(user_path);
    let canonical = fs::canonicalize(&full)?;
    fs::read_to_string(&canonical)
}

fn delete_file(base: &str, name: &str) -> Result<(), std::io::Error> {
    let path = Path::new(base).join(name);
    let target = fs::canonicalize(&path)?;
    fs::remove_file(&target)
}
