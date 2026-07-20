// SAFE: Sets restrictive permissions on the Unix socket to owner-only access
use std::os::unix::net::UnixListener;
use std::os::unix::fs::PermissionsExt;
use std::fs;

fn start_ipc_server() -> std::io::Result<()> {
    let socket_path = "/var/run/app.sock";
    let _ = fs::remove_file(socket_path);
    let listener = UnixListener::bind(socket_path)?;
    fs::set_permissions(socket_path, std::fs::Permissions::from_mode(0o700))?;
    for stream in listener.incoming() {
        let _stream = stream?;
    }
    Ok(())
}
