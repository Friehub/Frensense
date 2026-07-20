// SAFE: Uses rustls with ServerName verification; the server identity is always checked against the expected hostname.

use rustls::pki_types::ServerName;
use std::sync::Arc;

fn connect_rustls(hostname: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let root_store = rustls::RootCertStore::from_iter(
        webpki_roots::TLS_SERVER_ROOTS.iter().cloned()
    );
    let config = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    let server_name = ServerName::try_from(hostname)?;
    let stream = std::net::TcpStream::connect((hostname, port))?;
    let _conn = rustls::Stream::new(
        Arc::new(config).connect(server_name, stream)?,
    );
    Ok(())
}
