// SAFE alternative: custom CA store with verification still on
use openssl::ssl::{SslConnector, SslMethod};
use openssl::x509::X509StoreBuilder;

fn create_connector_with_custom_ca(ca_cert_pem: &[u8]) -> SslConnector {
    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    let cert = openssl::x509::X509::from_pem(ca_cert_pem).unwrap();
    let mut store_builder = X509StoreBuilder::new().unwrap();
    store_builder.add_cert(cert).unwrap();
    builder.set_verify_cert_store(store_builder.build()).unwrap();
    builder.build()
}
