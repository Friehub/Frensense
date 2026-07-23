// [frensense]
// observation: An HTTP/1.1 server handles h2c (HTTP/2 cleartext) upgrade requests, allowing clients to upgrade to HTTP/2 over a plaintext (non-TLS) connection. This bypasses TLS and may expose traffic.
// impact: Traffic is sent in cleartext, violating security requirements. Attackers on the network can read or modify HTTP/2 frames.
// improvement: Disable h2c upgrade support, or require TLS for HTTP/2 connections via ALPN (h2).

use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use std::net::SocketAddr;

pub async fn serve(addr: SocketAddr) {
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            let service = service_fn(|_req: Request<hyper::body::Incoming>| async {
                Ok::<_, hyper::Error>(Response::new("hello".into()))
            });
            http1::Builder::new()
                .serve_connection(stream, service)
                .await
                .ok();
        });
    }
}
