// SAFE: AWS SDK credentials are loaded from the environment chain, not hardcoded

use aws_sdk_s3::Client;

#[tokio::main]
async fn main() {
    let config = aws_sdk_s3::Config::builder()
        .region(aws_sdk_s3::config::Region::new("us-east-1"))
        .build();

    let client = Client::from_conf(config);
    // ...
}
