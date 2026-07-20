use tonic::transport::{Server, Identity, Certificate};
use hello::greeter_server::{GreeterServer, Greeter};
use hello::{HelloRequest, HelloResponse};
use std::fs;

pub mod hello {
    tonic::include_proto!("hello");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cert = fs::read("server.crt")?;
    let key = fs::read("server.key")?;
    let identity = Identity::from_pem(cert, key);

    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();
    // SAFE: TLS is configured via `tls_config()` before `serve()`.
    Server::builder()
        .tls_config(identity)?
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
