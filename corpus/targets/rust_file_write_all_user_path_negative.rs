// SAFE: Canonicalizes the path and checks it stays within the allowed base directory
use std::fs;
use std::path::PathBuf;

const ALLOWED_BASE: &str = "/var/www/uploads";

fn safe_path(base: &str, user_filename: &str) -> Result<PathBuf, String> {
    let filename = user_filename.replace('/', "").replace("..", "");
    if filename.is_empty() {
        return Err("invalid filename".into());
    }
    let path = PathBuf::from(base).join(&filename);
    let canonical = path.canonicalize().map_err(|_| "invalid path".to_string())?;
    if canonical.starts_with(ALLOWED_BASE) {
        Ok(canonical)
    } else {
        Err("path traversal detected".into())
    }
}

fn save_upload(filename: String, data: Vec<u8>) -> Result<(), String> {
    let path = safe_path(ALLOWED_BASE, &filename)?;
    fs::write(&path, &data).map_err(|e| e.to_string())?;
    Ok(())
}
