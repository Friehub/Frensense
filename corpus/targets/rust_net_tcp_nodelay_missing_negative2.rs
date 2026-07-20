// SAFE: Uses TcpStream::set_nodelay in a connection builder pattern, ensuring all connections have Nagle disabled.

use std::net::TcpStream;

fn create_low_latency_connection(addr: &str) -> std::io::Result<TcpStream> {
    let stream = TcpStream::connect(addr)?;
    stream.set_nodelay(true)?;
    Ok(stream)
}
