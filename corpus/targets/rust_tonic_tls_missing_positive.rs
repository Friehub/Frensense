// [frensense]
// observation: A Tonic gRPC server is constructed without TLS configuration. The `Server::builder()` is used without `.tls_config()` or a `rustls` certificate, exposing plaintext h2c traffic.
// impact: All gRPC communication, including RPC metadata and payloads, is sent in cleartext. An on-path attacker can eavesdrop, tamper with messages, or hijack the connection.
// improvement: Configure TLS via `Server::builder().tls_config()` using a valid Rustls certificate and private key.

use tonic::transport::Server;
use hello::greeter_server::{GreeterServer, Greeter};
use hello::{HelloRequest, HelloResponse};

pub mod hello {
    tonic::include_proto!("hello");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();
    Server::builder()
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
