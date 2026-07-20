// [frensense]
// observation: A TLS connection is established without verifying the server name (no set_verify_hostname or DangerousVerifier configured), allowing MITM attacks.
// impact: An attacker with a valid certificate for any domain (or a self-signed cert if verification is disabled) can intercept and modify TLS traffic, compromising data confidentiality and integrity.
// improvement: Use rustls::ServerName or native_tls TlsConnector::set_verify_hostname(true) to verify the server identity.

use native_tls::TlsConnector;
use std::net::TcpStream;

fn connect_redis(host: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let connector = TlsConnector::builder()
        .build()?;
    let stream = TcpStream::connect((host, port))?;
    let _tls = connector.connect(host, stream)?;
    Ok(())
}

fn connect_db(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let connector = TlsConnector::new()?;
    let stream = TcpStream::connect(addr)?;
    let _tls = connector.connect("db.internal", stream)?;
    Ok(())
}
