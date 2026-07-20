// SAFE: Canonicalizes both the base and the full path, then checks prefix
use std::fs;
use std::path::Path;

fn read_safe_file(base: &str, user_path: &str) -> Result<String, std::io::Error> {
    let base_canonical = fs::canonicalize(base)?;
    let full = base_canonical.join(user_path);
    let canonical = fs::canonicalize(&full)?;
    if !canonical.starts_with(&base_canonical) {
        return Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "path traversal"));
    }
    fs::read_to_string(&canonical)
}
