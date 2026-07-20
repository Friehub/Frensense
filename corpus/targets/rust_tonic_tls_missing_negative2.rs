use tonic::transport::Server;
use tonic::transport::server::TlsAcceptor;
use hello::greeter_server::{GreeterServer, Greeter};
use hello::{HelloRequest, HelloResponse};

pub mod hello {
    tonic::include_proto!("hello");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();
    // SAFE: Using `TlsAcceptor` with a Rustls server config enforces encrypted transport.
    let rustls_config = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(vec![], std::sync::Arc::new(rustls::PrivateKey(vec![])))
        .unwrap();
    let tls_acceptor = TlsAcceptor::new(rustls_config);
    Server::builder()
        .tls_acceptor(tls_acceptor)
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;
    Ok(())
}

#[derive(Default)]
pub struct MyGreeter;

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(&self, request: tonic::Request<HelloRequest>) -> Result<tonic::Response<HelloResponse>, tonic::Status> {
        Ok(tonic::Response::new(HelloResponse { message: "hello".into() }))
    }
}
