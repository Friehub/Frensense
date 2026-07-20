// SAFE: Uses an abstract socket namespace (Linux) which does not create a filesystem entry with permissions
use std::os::unix::net::UnixListener;

fn start_ipc_server() -> std::io::Result<()> {
    let listener = UnixListener::bind("\0app.sock")?;
    for stream in listener.incoming() {
        let _stream = stream?;
    }
    Ok(())
}
