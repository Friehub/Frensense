// SAFE: Strips directory separators from the filename and ensures the path resolves inside the uploads directory
use std::fs;
use std::path::{Path, PathBuf};

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .filter(|c| c.is_alphanumeric() || *c == '.' || *c == '-' || *c == '_')
        .collect()
}

fn save_upload(filename: String, data: Vec<u8>) -> std::io::Result<()> {
    let safe_name = sanitize_filename(&filename);
    if safe_name.is_empty() || safe_name.starts_with('.') {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid filename"));
    }
    let path = Path::new("/var/www/uploads").join(&safe_name);
    fs::write(&path, &data)
}
