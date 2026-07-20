// SAFE: TlsConnector builder sets danger_accept_invalid_certs(false) and relies on default verify hostname; server name is verified.

use native_tls::TlsConnector;
use std::net::TcpStream;

fn connect_redis(host: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let connector = TlsConnector::builder()
        .danger_accept_invalid_certs(false)
        .build()?;
    let stream = TcpStream::connect((host, port))?;
    let _tls = connector.connect(host, stream)?;
    Ok(())
}
