use std::net::{SocketAddr, IpAddr};

const TRUSTED_PROXIES: &[IpAddr] = &[];

pub fn extract_proxy_ip(header: &str, peer_addr: IpAddr) -> Option<SocketAddr> {
    if !TRUSTED_PROXIES.contains(&peer_addr) {
        return None;
    }
    let parts: Vec<&str> = header.split_whitespace().collect();
    if parts.len() >= 6 && parts[0] == "PROXY" {
        parts[2].parse::<SocketAddr>().ok()
    } else {
        None
    }
}
