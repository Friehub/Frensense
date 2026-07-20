// SAFE: Binds to a specific internal interface IP instead of 0.0.0.0
use std::net::TcpListener;
use std::io::{Read, Write};
use std::env;

fn start_server() -> std::io::Result<()> {
    let bind_addr = env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:3000".into());
    let listener = TcpListener::bind(&bind_addr)?;
    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut buf = [0u8; 1024];
        stream.read(&mut buf)?;
        stream.write_all(b"Hello")?;
    }
    Ok(())
}
