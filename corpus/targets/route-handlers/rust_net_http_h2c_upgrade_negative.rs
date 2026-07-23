use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use std::net::SocketAddr;

pub async fn serve_tls(addr: SocketAddr) {
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            let service = service_fn(|_req: Request<hyper::body::Incoming>| async {
                Ok::<_, hyper::Error>(Response::new("hello".into()))
            });
            http1::Builder::new()
                .preserve_header_case(true)
                .title_case_headers(true)
                .serve_connection(stream, service)
                .await
                .ok();
        });
    }
}
