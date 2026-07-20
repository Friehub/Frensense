// SAFE: PermissionDenied errors are logged and propagated, never ignored.
use std::fs;
use std::path::Path;

pub fn delete_if_exists(path: &Path) -> std::io::Result<()> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => {
            eprintln!("failed to delete {}: {}", path.display(), e);
            Err(e)
        }
    }
}
