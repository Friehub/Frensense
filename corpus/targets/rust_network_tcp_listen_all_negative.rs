// SAFE: Binds only to localhost, preventing external network access
use std::net::TcpListener;
use std::io::{Read, Write};

fn start_server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000")?;
    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut buf = [0u8; 1024];
        stream.read(&mut buf)?;
        stream.write_all(b"Hello")?;
    }
    Ok(())
}
