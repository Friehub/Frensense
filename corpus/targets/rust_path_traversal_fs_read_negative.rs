// SAFE: Path is canonicalized and verified to stay within the base directory
use std::fs;
use std::path::{Path, PathBuf};

fn safe_read(base_dir: &str, user_path: &str) -> Result<String, String> {
    let base = Path::new(base_dir).canonicalize().map_err(|e| e.to_string())?;
    let full = base.join(user_path);
    let canonical = full.canonicalize().map_err(|e| format!("Invalid path: {}", e))?;
    if !canonical.starts_with(&base) {
        return Err("Path traversal detected".into());
    }
    fs::read_to_string(&canonical).map_err(|e| e.to_string())
}

fn read_user_file(base_dir: &str, user_path: &str) -> Result<String, String> {
    safe_read(base_dir, user_path)
}

fn get_config(dir: &str, file: &str) -> Result<Vec<u8>, String> {
    let data = safe_read(dir, file)?;
    Ok(data.into_bytes())
}
