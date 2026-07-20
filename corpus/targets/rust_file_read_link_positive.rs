// [frensense]
// observation: `std::fs::read_link` is called on a path derived from user input without validating that the resolved link target is safe, allowing symlink traversal attacks.
// impact: An attacker can create a symlink pointing to a sensitive file (e.g. /etc/passwd, /var/db/private.key), and the application will read and expose the target.
// improvement: After reading the link, validate that the resolved path is within an allowed directory.

use std::fs;

fn resolve_user_symlink(link_name: String) -> std::io::Result<String> {
    let target = fs::read_link(&link_name)?;
    Ok(target.to_string_lossy().into_owned())
}

fn get_symlink_target(user_path: String) -> std::io::Result<String> {
    let path = format!("/var/linkpool/{}", user_path);
    let target = fs::read_link(&path)?;
    Ok(target.to_string_lossy().into_owned())
}
