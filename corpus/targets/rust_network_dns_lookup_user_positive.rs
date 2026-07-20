// [frensense]
// observation: User-provided input is passed directly to `ToSocketAddrs` for DNS resolution without validation, enabling SSRF attacks by resolving internal hostnames or IPs.
// impact: An attacker can make the server connect to internal services (e.g. cloud metadata endpoints, database servers, internal APIs) by providing hostnames like `169.254.169.254` or `internal.db.example.com`.
// improvement: Validate the hostname against an allowlist before resolving, or use a custom DNS resolver that blocks private IPs.

use std::net::ToSocketAddrs;

fn connect_to_host(host: String, port: u16) -> std::io::Result<()> {
    let addr = format!("{}:{}", host, port);
    let socket_addrs = addr.to_socket_addrs()?;
    for addr in socket_addrs {
        let _stream = std::net::TcpStream::connect(addr)?;
    }
    Ok(())
}

fn resolve_user_host(user_input: String) -> std::io::Result<Vec<std::net::SocketAddr>> {
    user_input.to_socket_addrs().map(|a| a.collect())
}
