// [frensense]
// observation: A Unix domain socket is created with world-writable permissions (0o777 or `umask` 0), allowing any local user to connect or send data to the socket.
// impact: Any unprivileged user on the system can send requests to the socket, potentially accessing privileged functionality or data.
// improvement: Set restrictive permissions on the socket (e.g. 0o700) or use an abstract socket namespace with proper access controls.

use std::os::unix::net::UnixListener;
use std::fs;

fn start_ipc_server() -> std::io::Result<()> {
    let socket_path = "/var/run/app.sock";
    let _ = fs::remove_file(socket_path);
    let listener = UnixListener::bind(socket_path)?;
    fs::set_permissions(socket_path, std::fs::Permissions::from_mode(0o777))?;
    for stream in listener.incoming() {
        let _stream = stream?;
    }
    Ok(())
}

fn start_admin_socket() -> std::io::Result<()> {
    let socket_path = "/tmp/admin.sock";
    let _ = fs::remove_file(socket_path);
    let listener = UnixListener::bind(socket_path)?;
    for stream in listener.incoming() {
        let mut stream = stream?;
        use std::io::Read;
        let mut buf = [0u8; 1024];
        stream.read(&mut buf)?;
    }
    Ok(())
}
