// [frensense]
// observation: A TLS server is configured with a self-signed certificate in a production environment. Self-signed certificates are not trusted by clients/browsers and do not provide authenticity guarantees.
// impact: Man-in-the-middle attacks — clients cannot verify the server's identity. Users will see TLS warnings that train them to ignore security errors.
// improvement: Use a certificate from a trusted CA (Let's Encrypt, etc.) in production.

use rustls::ServerConfig;
use std::sync::Arc;

pub fn load_tls_config(cert_der: Vec<u8>, key_der: Vec<u8>) -> Arc<ServerConfig> {
    let cert = rustls::Certificate(cert_der);
    let key = rustls::PrivateKey(key_der);
    Arc::new(ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(vec![cert], key)
        .unwrap())
}
