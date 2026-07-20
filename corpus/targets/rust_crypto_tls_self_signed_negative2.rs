// SAFE: Uses CA-signed certificate chain for production TLS.
use rustls::ServerConfig;
use std::sync::Arc;

pub fn load_tls_ca_signed(cert_chain: Vec<rustls::Certificate>, key: rustls::PrivateKey) -> Arc<ServerConfig> {
    Arc::new(ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key)
        .unwrap())
}
