// SAFE: use resize or extend instead of set_len
fn recv_packet(socket: &mut std::net::TcpStream) -> std::io::Result<Vec<u8>> {
    let mut header = [0u8; 4];
    socket.read_exact(&mut header)?;
    let size = u32::from_be_bytes(header) as usize;

    let mut buf = vec![0u8; size];
    socket.read_exact(&mut buf)?;
    Ok(buf)
}

fn parse_custom_packet(data: &[u8]) -> Vec<u8> {
    let count = data[0] as usize;
    if count + 1 > data.len() { return Vec::new(); }
    data[1..=count].to_vec()
}
