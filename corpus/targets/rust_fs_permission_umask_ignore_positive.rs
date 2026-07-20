// [frensense]
// observation: A file is created with std::fs::write or OpenOptions::new().create(true) without setting explicit permissions, relying on the process umask which may be 000 or permissive.
// impact: If umask is 000 or very permissive, created files may be world-readable or world-writable, exposing sensitive data to other users on the system.
// improvement: Set explicit permissions using std::os::unix::fs::PermissionsExt or create files in a directory with restricted ACL.

use std::fs;
use std::io;

fn write_secret(path: &str, data: &str) -> io::Result<()> {
    fs::write(path, data)?;
    Ok(())
}

fn create_token_file(path: &str, token: &str) -> io::Result<()> {
    fs::write(path, token)?;
    Ok(())
}
