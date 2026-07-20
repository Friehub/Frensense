// SAFE: certificate verification enabled (default)
use openssl::ssl::{SslConnector, SslMethod};

fn create_secure_connector() -> SslConnector {
    let builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.build()
}
