// [frensense]
// observation: A Unix domain socket is bound to an abstract namespace address (Linux-specific, prefix \0), which has no filesystem permissions and is accessible to any process on the system.
// impact: Any process on the same Linux host can connect to the abstract socket, bypassing filesystem permission controls, leading to unauthorized access to services (e.g., Docker socket, IPC).
// improvement: Use a filesystem-based Unix socket with proper directory permissions, or add authentication to the protocol.

use std::os::unix::net::UnixListener;

fn start_ipc_service() -> std::io::Result<()> {
    let listener = UnixListener::bind("\0myapp.sock")?;
    for stream in listener.incoming() {
        let _ = stream;
    }
    Ok(())
}

fn start_daemon_ipc() -> std::io::Result<()> {
    let addr = format!("\0daemon-{}", std::process::id());
    let listener = UnixListener::bind(addr)?;
    Ok(())
}
