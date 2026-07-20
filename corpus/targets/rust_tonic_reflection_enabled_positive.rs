// [frensense]
// observation: Tonic gRPC reflection is enabled in a production binary via `tonic_reflection::Server::new().serve()` or inclusion of the reflection service in the server. Reflection exposes the full protobuf schema, including method names, input/output types, and file descriptors.
// impact: Attackers can enumerate all available RPC methods and their parameter types without authentication, aiding reconnaissance for targeted attacks. In combination with other weaknesses, this enables method discovery and payload crafting.
// improvement: Disable reflection in production builds, or gate it behind a compile-time feature flag and authentication.

use tonic::transport::Server;
use tonic_reflection::server::Server as ReflectionServer;
use hello::greeter_server::{GreeterServer, Greeter};
use hello::{HelloRequest, HelloResponse};

pub mod hello {
    tonic::include_proto!("hello");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();
    let reflection = ReflectionServer::new()
        .set_service_name("helloworld.Greeter")
        .set_file_descriptor_set(hello::FILE_DESCRIPTOR_SET);
    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .add_service(reflection)
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
