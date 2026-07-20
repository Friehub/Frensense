// [frensense]
// observation: A Tonic gRPC server does not call `.max_decoding_message_size()` on the server or channel, so it uses the default maximum message size (4 MiB for tonic). An attacker can send a message just under this limit repeatedly, causing the server to allocate large buffers.
// impact: Out-of-memory crash. A modest number of large messages can exhaust the server's memory, leading to denial of service.
// improvement: Set `max_decoding_message_size()` to a value appropriate for your application (typically 1–10 MiB depending on payload).

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
