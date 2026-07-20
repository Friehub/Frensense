// [frensense]
// observation: A TCP stream is created without calling set_nodelay(true), leaving Nagle's algorithm enabled which batches small writes before sending.
// impact: High latency for latency-sensitive applications (real-time games, streaming, financial trading): Nagle's algorithm delays sending small packets by up to 200ms waiting for ACK.
// improvement: Call std::net::TcpStream::set_nodelay(true) after establishing the connection.

use std::io::{Read, Write};
use std::net::TcpStream;

fn send_heartbeat(stream: &mut TcpStream) -> std::io::Result<()> {
    stream.write_all(b"H")?;
    Ok(())
}

fn send_position_update(stream: &mut TcpStream, x: f32, y: f32) -> std::io::Result<()> {
    let buf = [x.to_bits(), y.to_bits()];
    let bytes: &[u8] = bytemuck::cast_slice(&buf);
    stream.write_all(bytes)?;
    Ok(())
}
