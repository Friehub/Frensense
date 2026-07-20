// [frensense]
// observation: HTTP/1.1 requests are sent with Connection: close or through a client configured with no connection reuse, opening a new TCP connection per request.
// impact: High latency overhead from TCP handshake + TLS negotiation per request; increased server load from connection setup; wasteful for repeated requests to the same host.
// improvement: Enable HTTP keep-alive (default in HTTP/1.1) or use a connection pool (e.g., reqwest::Client with default pool).

use std::io::{Read, Write};
use std::net::TcpStream;

fn fetch_url(host: &str, path: &str) -> std::io::Result<String> {
    let mut stream = TcpStream::connect(format!("{}:80", host))?;
    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n", path, host);
    stream.write_all(request.as_bytes())?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    Ok(response)
}

fn fetch_two_urls(host: &str) -> std::io::Result<()> {
    let r1 = fetch_url(host, "/api/data1")?;
    let r2 = fetch_url(host, "/api/data2")?;
    println!("{}{}", r1, r2);
    Ok(())
}
