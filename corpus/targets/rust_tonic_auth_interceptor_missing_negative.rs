use tonic::transport::Server;
use tonic::{Request, Status};
use hello::greeter_server::{GreeterServer, Greeter};
use hello::{HelloRequest, HelloResponse};

pub mod hello {
    tonic::include_proto!("hello");
}

fn auth_interceptor(mut req: Request<()>) -> Result<Request<()>, Status> {
    let token = req.metadata().get("authorization").and_then(|v| v.to_str().ok());
    // SAFE: Every RPC is checked for a valid bearer token before the handler runs.
    match token {
        Some(t) if t == "Bearer valid-token-123" => Ok(req),
        _ => Err(Status::unauthenticated("invalid token")),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();
    Server::builder()
        .add_service(GreeterServer::with_interceptor(greeter, auth_interceptor))
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
