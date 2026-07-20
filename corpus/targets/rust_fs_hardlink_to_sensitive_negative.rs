use std::fs;
use std::path::Path;

pub fn create_link_safe(target: &Path, link: &Path) -> std::io::Result<()> {
    let sensitive = ["/etc/shadow", "/etc/passwd", "/etc/ssl/private"];
    let target_str = target.to_string_lossy();
    if sensitive.iter().any(|s| target_str.contains(s)) {
        return Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "target is sensitive"));
    }
    fs::hard_link(target, link)
}
