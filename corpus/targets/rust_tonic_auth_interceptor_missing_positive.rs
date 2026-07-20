// [frensense]
// observation: A Tonic gRPC service is registered without an authentication interceptor. The `Server::builder().add_service(GreeterServer::new(svc))` does not include `tonic::service::interceptor` or any middleware that validates credentials before the RPC handler executes.
// impact: Unauthenticated access to all RPC methods. Any client with network access can invoke protected procedures, leading to data exposure, privilege escalation, or unauthorized mutations.
// improvement: Use `tonic::service::interceptor` to validate an auth token (e.g., JWT, bearer token) from the gRPC metadata before the request reaches the handler.

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
