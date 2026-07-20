// [frensense]
// observation: The PROXY protocol header from a network connection is parsed and trusted without validation. An attacker can spoof the source IP address by sending a forged PROXY header.
// impact: IP-based authentication, rate limiting, and audit logs are bypassed — the attacker claims a trusted IP and bypasses all IP-based controls.
// improvement: Validate the PROXY protocol header (source IP is from a trusted proxy) or restrict PROXY protocol to trusted subnets only.

use std::net::SocketAddr;

pub fn extract_proxy_ip(header: &str) -> Option<SocketAddr> {
    let parts: Vec<&str> = header.split_whitespace().collect();
    if parts.len() >= 6 && parts[0] == "PROXY" {
        parts[2].parse::<SocketAddr>().ok()
    } else {
        None
    }
}
