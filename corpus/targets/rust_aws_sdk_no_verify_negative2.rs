// SAFE: Uses STS AssumeRole with a session duration limit for temporary credentials

use aws_sdk_sts::Client as StsClient;
use aws_sdk_s3::Client as S3Client;

#[tokio::main]
async fn main() {
    let sts = StsClient::new(&aws_config::load_from_env().await);
    let creds = sts.assume_role()
        .role_arn("arn:aws:iam::123456789012:role/my-limited-role")
        .role_session_name("my-session")
        .duration_seconds(900)
        .send()
        .await
        .unwrap();

    let s3_config = aws_sdk_s3::Config::builder()
        .region(aws_sdk_s3::config::Region::new("us-east-1"))
        .build();

    let client = S3Client::from_conf(s3_config);
    // ...
}
