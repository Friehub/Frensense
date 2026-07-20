// SAFE: Validates the hostname against an allowlist before DNS resolution
use std::net::{ToSocketAddrs, TcpStream};
use url::Url;

const ALLOWED_HOSTS: &[&str] = &["api.trusted.com", "data.trusted.com"];

fn is_host_allowed(host: &str) -> bool {
    ALLOWED_HOSTS.contains(&host)
}

fn connect_to_host(host: String, port: u16) -> Result<(), String> {
    if !is_host_allowed(&host) {
        return Err("host not allowed".into());
    }
    let addr = format!("{}:{}", host, port);
    let socket_addrs = addr.to_socket_addrs().map_err(|e| e.to_string())?;
    for addr in socket_addrs {
        TcpStream::connect(addr).map_err(|e| e.to_string())?;
    }
    Ok(())
}
