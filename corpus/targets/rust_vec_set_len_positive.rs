// [frensense]
// observation: Vec::set_len() called with a length value derived from external data without ensuring the memory is initialized.
// impact: set_len() bypasses Rust's memory initialization guarantee. Reading uninitialized elements causes undefined behavior. If the attacker controls the length, they may read sensitive heap memory.
// improvement: Use Vec::resize() to allocate and initialize elements, or push elements one-by-one. Only use set_len after initializing every element.

fn recv_packet(socket: &mut std::net::TcpStream) -> Vec<u8> {
    let mut header = [0u8; 4];
    socket.read_exact(&mut header).unwrap();
    let size = u32::from_be_bytes(header) as usize;

    let mut buf = Vec::with_capacity(size);
    // VULNERABLE: attacker-controlled size leaves uninitialized memory
    unsafe { buf.set_len(size); }
    socket.read_exact(&mut buf).unwrap();
    buf
}

fn parse_custom_packet(data: &[u8]) -> Vec<u8> {
    let count = data[0] as usize;
    let mut out = Vec::with_capacity(count);
    // VULNERABLE: count may exceed data
    unsafe { out.set_len(count); }
    out.copy_from_slice(&data[1..=count]);
    out
}
