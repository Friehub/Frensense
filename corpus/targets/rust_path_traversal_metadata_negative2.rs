// SAFE: Uses Path::components to ensure no parent-dir traversal before calling metadata
use std::fs;
use std::path::Path;

fn check_file_size(base_dir: &str, rel_path: &str) -> Result<u64, std::io::Error> {
    let path = Path::new(base_dir).join(rel_path);
    for component in path.components() {
        if let std::path::Component::ParentDir = component {
            return Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "parent dir in path"));
        }
    }
    let meta = fs::metadata(&path)?;
    Ok(meta.len())
}
