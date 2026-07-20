// SAFE: set_nodelay(true) is called after connecting, disabling Nagle's algorithm for low-latency writes.

use std::io::{Read, Write};
use std::net::TcpStream;

fn send_heartbeat(stream: &mut TcpStream) -> std::io::Result<()> {
    stream.set_nodelay(true)?;
    stream.write_all(b"H")?;
    Ok(())
}

fn send_position_update(stream: &mut TcpStream, x: f32, y: f32) -> std::io::Result<()> {
    stream.set_nodelay(true)?;
    let buf = [x.to_bits(), y.to_bits()];
    let bytes: &[u8] = bytemuck::cast_slice(&buf);
    stream.write_all(bytes)?;
    Ok(())
}
