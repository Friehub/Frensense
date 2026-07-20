// SAFE: Canonicalizes and checks prefix against base directory before metadata access
use std::fs;
use std::path::Path;

fn check_file_size(base_dir: &str, rel_path: &str) -> Result<u64, std::io::Error> {
    let base = fs::canonicalize(base_dir)?;
    let full = base.join(rel_path);
    let canonical = fs::canonicalize(&full)?;
    if !canonical.starts_with(&base) {
        return Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "access denied"));
    }
    let meta = fs::metadata(&canonical)?;
    Ok(meta.len())
}
