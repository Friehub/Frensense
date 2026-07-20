// SAFE: Strips path components before canonicalization and verifies prefix constraint
use std::fs;
use std::path::Path;

fn read_safe_file(base: &str, user_path: &str) -> Result<String, std::io::Error> {
    let base_canonical = fs::canonicalize(base)?;
    let joined = Path::new(base_canonical.as_path()).join(user_path);
    let canonical = joined.canonicalize()?;
    if !canonical.starts_with(&base_canonical) {
        return Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "traversal blocked"));
    }
    fs::read_to_string(&canonical)
}
