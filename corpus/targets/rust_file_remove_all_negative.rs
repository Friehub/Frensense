// SAFE: Canonicalizes the path and verifies it stays within the allowed data directory
use std::fs;
use std::path::Path;

const ALLOWED_DATA_DIR: &str = "/var/data/users";

fn validate_remove_path(user_path: &str) -> Result<String, String> {
    let user_id = user_path.trim_start_matches('/').replace('/', "");
    if user_id.is_empty() || user_id.contains("..") {
        return Err("invalid user id".into());
    }
    let full_path = format!("{}/{}", ALLOWED_DATA_DIR, user_id);
    let canonical = Path::new(&full_path).canonicalize().map_err(|_| "invalid path".to_string())?;
    if canonical.starts_with(ALLOWED_DATA_DIR) {
        Ok(canonical.to_string_lossy().into_owned())
    } else {
        Err("path traversal detected".into())
    }
}

fn cleanup_user_data(user_id: String) -> Result<(), String> {
    let safe_path = validate_remove_path(&user_id)?;
    fs::remove_dir_all(&safe_path).map_err(|e| e.to_string())
}
