// SAFE: Only accepts PROXY protocol headers from trusted proxy IPs.
use std::net::{SocketAddr, IpAddr};

fn is_trusted_proxy(addr: IpAddr) -> bool {
    let trusted: Vec<IpAddr> = vec![
        "10.0.0.1".parse().unwrap(),
        "10.0.0.2".parse().unwrap(),
    ];
    trusted.contains(&addr)
}

pub fn extract_proxy_ip(header: &str, peer: SocketAddr) -> Option<SocketAddr> {
    if !is_trusted_proxy(peer.ip()) {
        return None;
    }
    let parts: Vec<&str> = header.split_whitespace().collect();
    if parts.len() >= 6 && parts[0] == "PROXY" {
        parts[2].parse::<SocketAddr>().ok()
    } else {
        None
    }
}
