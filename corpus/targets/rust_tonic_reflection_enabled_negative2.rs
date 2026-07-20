use tonic::transport::Server;
use hello::greeter_server::{GreeterServer, Greeter};
use hello::{HelloRequest, HelloResponse};

pub mod hello {
    tonic::include_proto!("hello");
}

#[cfg(not(feature = "production"))]
fn add_reflection(builder: Server) -> Server {
    builder.add_service(
        tonic_reflection::server::Server::new()
            .set_service_name("helloworld.Greeter")
            .set_file_descriptor_set(hello::FILE_DESCRIPTOR_SET),
    )
}

#[cfg(feature = "production")]
fn add_reflection(builder: Server) -> Server {
    // SAFE: Reflection disabled in production builds via feature flag.
    builder
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();
    let builder = Server::builder().add_service(GreeterServer::new(greeter));
    add_reflection(builder).serve(addr).await?;
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
