// SAFE: Sanitizes user input to alphanumeric-only before constructing the path
use std::fs;
use std::path::Path;

fn sanitize_user_id(raw: &str) -> String {
    raw.chars().filter(|c| c.is_alphanumeric() || *c == '_').collect()
}

fn cleanup_user_data(user_id: String) -> std::io::Result<()> {
    let safe_id = sanitize_user_id(&user_id);
    if safe_id.is_empty() {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid user id"));
    }
    let dir = format!("/var/data/users/{}", safe_id);
    if !Path::new(&dir).exists() {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "directory not found"));
    }
    fs::remove_dir_all(&dir)
}
