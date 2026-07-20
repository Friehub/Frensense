// SAFE: Resolves the hostname and filters out private/reserved IPs from the result
use std::net::{ToSocketAddrs, TcpStream, IpAddr};

fn is_public_ip(addr: &IpAddr) -> bool {
    match addr {
        IpAddr::V4(v4) => !(v4.is_loopback() || v4.is_private() || v4.is_link_local() || v4.is_unspecified()),
        IpAddr::V6(v6) => !(v6.is_loopback() || v6.is_unspecified()),
    }
}

fn connect_to_host(host: String, port: u16) -> Result<(), String> {
    let addr = format!("{}:{}", host, port);
    let socket_addrs = addr.to_socket_addrs().map_err(|e| e.to_string())?;
    for sock_addr in socket_addrs {
        if !is_public_ip(&sock_addr.ip()) {
            return Err(format!("blocked private IP: {}", sock_addr.ip()));
        }
        TcpStream::connect(sock_addr).map_err(|e| e.to_string())?;
    }
    Ok(())
}
