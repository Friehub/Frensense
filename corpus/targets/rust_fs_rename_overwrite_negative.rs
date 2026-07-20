use std::fs;
use std::path::Path;

pub fn move_file(src: &Path, dst: &Path) -> std::io::Result<()> {
    if dst.exists() {
        return Err(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "destination exists"));
    }
    fs::rename(src, dst)
}
