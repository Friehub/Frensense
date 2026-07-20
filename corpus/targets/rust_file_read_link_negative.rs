// SAFE: Canonicalizes the symlink target and verifies it stays within the allowed pool directory
use std::fs;
use std::path::Path;

const LINK_POOL: &str = "/var/linkpool";

fn resolve_safe_link(user_path: &str) -> Result<String, String> {
    let safe_name = user_path.replace('/', "").replace("..", "");
    if safe_name.is_empty() {
        return Err("invalid path".into());
    }
    let link_path = format!("{}/{}", LINK_POOL, safe_name);
    let target = fs::read_link(&link_path).map_err(|e| e.to_string())?;
    let canonical = target.canonicalize().map_err(|_| "cannot resolve target".to_string())?;
    if canonical.starts_with(LINK_POOL) {
        Ok(canonical.to_string_lossy().into_owned())
    } else {
        Err("symlink target escapes allowed directory".into())
    }
}

fn get_symlink_target(user_path: String) -> Result<String, String> {
    resolve_safe_link(&user_path)
}
