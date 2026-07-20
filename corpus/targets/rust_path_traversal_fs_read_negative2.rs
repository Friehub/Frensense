// SAFE: Uses path.join with basename to strip directory components; allows only safe filenames
use std::fs;
use std::path::{Path, PathBuf};

fn safe_filename(input: &str) -> String {
    Path::new(input)
        .file_name()
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or_default()
}

fn read_user_file(base_dir: &str, user_path: &str) -> Result<String, String> {
    let safe = safe_filename(user_path);
    if safe.is_empty() {
        return Err("Invalid filename".into());
    }
    let base = Path::new(base_dir).canonicalize().map_err(|e| e.to_string())?;
    let full_path = base.join(&safe);
    if !full_path.starts_with(&base) {
        return Err("Path traversal detected".into());
    }
    fs::read_to_string(&full_path).map_err(|e| e.to_string())
}
