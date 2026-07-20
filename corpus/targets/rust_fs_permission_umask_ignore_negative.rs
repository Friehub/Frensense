// SAFE: Explicit permissions (0o600) are set on the created file using std::os::unix::fs::PermissionsExt, regardless of umask.

use std::fs;
use std::io;
use std::os::unix::fs::PermissionsExt;

fn write_secret(path: &str, data: &str) -> io::Result<()> {
    fs::write(path, data)?;
    let perms = fs::Permissions::from_mode(0o600);
    fs::set_permissions(path, perms)?;
    Ok(())
}

fn create_token_file(path: &str, token: &str) -> io::Result<()> {
    fs::write(path, token)?;
    let perms = fs::Permissions::from_mode(0o600);
    fs::set_permissions(path, perms)?;
    Ok(())
}
