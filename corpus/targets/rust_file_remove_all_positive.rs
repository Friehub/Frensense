// [frensense]
// observation: `std::fs::remove_dir_all` is called with a path derived from user input without sanitization or confirmation.
// impact: An attacker can delete arbitrary directories and their contents by controlling the path, causing data loss or denial of service.
// improvement: Restrict deletion to an allowlisted directory and validate the path before removing.

use std::fs;

fn cleanup_user_data(user_id: String) -> std::io::Result<()> {
    let dir = format!("/var/data/users/{}", user_id);
    fs::remove_dir_all(&dir)?;
    Ok(())
}

fn delete_path(path: String) -> std::io::Result<()> {
    fs::remove_dir_all(&path)?;
    Ok(())
}
