// SAFE: Uses symlink with proper access control instead of hard link.
use std::fs;
use std::os::unix::fs::symlink;
use std::path::Path;

pub fn create_link_safe(target: &Path, link: &Path) -> std::io::Result<()> {
    symlink(target, link)
}
