use rustls::ServerConfig;
use std::sync::Arc;

pub fn load_tls_production(cert_der: Vec<u8>, key_der: Vec<u8>, ca_der: Vec<u8>) -> Arc<ServerConfig> {
    let cert = rustls::Certificate(cert_der);
    let key = rustls::PrivateKey(key_der);
    let ca = rustls::Certificate(ca_der);
    let mut roots = rustls::RootCertStore::empty();
    roots.add(&ca).unwrap();
    let verifier = rustls::server::WebPkiClientVerifier::builder(Arc::new(roots)).build().unwrap();
    Arc::new(ServerConfig::builder()
        .with_client_verifier(verifier)
        .with_single_cert(vec![cert], key)
        .unwrap())
}
