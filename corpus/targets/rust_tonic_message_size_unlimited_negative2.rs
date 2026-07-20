use tonic::transport::{Server, Endpoint};
use hello::greeter_client::GreeterClient;
use hello::{HelloRequest, HelloResponse};

pub mod hello {
    tonic::include_proto!("hello");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // SAFE: Client-side limit of 1 MiB on incoming messages prevents server from sending oversized responses that cause OOM.
    let channel = Endpoint::from_static("http://[::1]:50051")
        .max_decoding_message_size(1 * 1024 * 1024)
        .connect()
        .await?;
    let mut client = GreeterClient::new(channel);
    let _resp = client.say_hello(HelloRequest { name: "world".into() }).await?;
    Ok(())
}
