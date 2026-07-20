// [frensense]
// observation: `TcpListener::bind` uses `0.0.0.0:3000` which listens on all network interfaces, exposing the service to the public internet.
// impact: An internal service intended only for localhost access becomes publicly reachable, increasing the attack surface and risk of unauthorized access.
// improvement: Bind to `127.0.0.1` when the service is meant to be local-only, or bind to a specific internal network interface.

use std::net::TcpListener;
use std::io::{Read, Write};

fn start_server() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:3000")?;
    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut buf = [0u8; 1024];
        stream.read(&mut buf)?;
        stream.write_all(b"Hello")?;
    }
    Ok(())
}

fn start_admin_server() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:9090")?;
    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut buf = [0u8; 1024];
        stream.read(&mut buf)?;
        stream.write_all(b"Admin panel")?;
    }
    Ok(())
}
