// SAFE: Uses filesystem socket with pre-existing directory and explicit permissions on the socket file itself.

use std::os::unix::net::UnixListener;
use std::os::unix::fs::PermissionsExt;
use std::fs;

fn start_daemon_ipc() -> std::io::Result<()> {
    let socket_path = format!("/tmp/daemon-{}.sock", std::process::id());
    if std::path::Path::new(&socket_path).exists() {
        fs::remove_file(&socket_path)?;
    }
    let listener = UnixListener::bind(&socket_path)?;
    fs::set_permissions(&socket_path, fs::Permissions::from_mode(0o600))?;
    Ok(())
}
