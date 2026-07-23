use tonic::transport::Server;
use tonic::{Request, Status, metadata::MetadataValue};
use hello::greeter_server::{GreeterServer, Greeter};
use hello::{HelloRequest, HelloResponse};

pub mod hello {
    tonic::include_proto!("hello");
}

#[derive(Clone)]
struct AuthSvc {
    token: String,
}

impl AuthSvc {
    fn check(&self, req: Request<()>) -> Result<Request<()>, Status> {
        // SAFE: Token validation via interceptor guards all methods.
        let token: Option<&MetadataValue<_>> = req.metadata().get("x-api-key");
        match token {
            Some(t) if t == &self.token.as_str().parse().unwrap() => Ok(req),
            _ => Err(Status::unauthenticated("missing or invalid api key")),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();
    let auth = AuthSvc { token: "sk-1234".into() };
    Server::builder()
        .add_service(GreeterServer::with_interceptor(greeter, move |req| auth.check(req)))
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
