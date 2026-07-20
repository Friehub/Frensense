// SAFE: Uses HTTPS with TLS via ALPN, no h2c upgrade is possible.
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use rustls::ServerConfig;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio_rustls::TlsAcceptor;

pub async fn serve_tls(addr: SocketAddr, tls: Arc<ServerConfig>) {
    let acceptor = TlsAcceptor::from(tls);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let acceptor = acceptor.clone();
        tokio::spawn(async move {
            let tls_stream = acceptor.accept(stream).await.unwrap();
            let service = service_fn(|_req: Request<hyper::body::Incoming>| async {
                Ok::<_, hyper::Error>(Response::new("hello".into()))
            });
            http1::Builder::new()
                .serve_connection(tls_stream, service)
                .await
                .ok();
        });
    }
}
