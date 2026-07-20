// SAFE: Uses `fs::symlink_metadata` to check for symlinks first, then validates the target is not in a restricted path
use std::fs;
use std::path::Path;

const ALLOWED_DIR: &str = "/var/linkpool";
const RESTRICTED_DIRS: &[&str] = &["/etc", "/var/db", "/home"];

fn get_symlink_target(user_path: String) -> Result<String, String> {
    let safe_name: String = user_path.chars().filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-').collect();
    if safe_name.is_empty() {
        return Err("invalid path".into());
    }
    let link_path = format!("{}/{}", ALLOWED_DIR, safe_name);
    let meta = fs::symlink_metadata(&link_path).map_err(|e| e.to_string())?;
    if !meta.file_type().is_symlink() {
        return Err("not a symlink".into());
    }
    let target = fs::read_link(&link_path).map_err(|e| e.to_string())?;
    let target_str = target.to_string_lossy();
    for restricted in RESTRICTED_DIRS {
        if target_str.starts_with(restricted) {
            return Err(format!("symlink to restricted directory: {}", restricted));
        }
    }
    Ok(target_str.into_owned())
}
