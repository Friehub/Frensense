use std::fs;
use std::path::Path;

pub fn delete_if_exists(path: &Path) -> std::io::Result<()> {
    fs::remove_file(path).or_else(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            Ok(())
        } else {
            Err(e)
        }
    })
}
