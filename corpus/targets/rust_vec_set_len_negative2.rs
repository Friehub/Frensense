// SAFE alternative: initialize elements before set_len
fn recv_packet(socket: &mut std::net::TcpStream) -> std::io::Result<Vec<u8>> {
    let mut header = [0u8; 4];
    socket.read_exact(&mut header)?;
    let size = u32::from_be_bytes(header) as usize;

    let mut buf = Vec::with_capacity(size);
    buf.resize(size, 0u8);
    socket.read_exact(&mut buf)?;
    Ok(buf)
}
