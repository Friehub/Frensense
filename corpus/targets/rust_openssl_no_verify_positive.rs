// [frensense]
// observation: SSL/TLS connector configured with certificate verification disabled via OpenSSL or rustls.
// impact: Without certificate verification, any attacker on the network path can present a self-signed certificate and intercept the connection.
// improvement: Keep certificate verification enabled. If using self-signed certs, use a custom root CA store.

use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};

fn create_insecure_connector() -> SslConnector {
    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    // VULNERABLE: disables certificate verification
    builder.set_verify(SslVerifyMode::NONE);
    builder.build()
}
