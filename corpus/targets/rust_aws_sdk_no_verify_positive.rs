// [frensense]
// observation: AWS SDK credentials are hardcoded or configured with overly permissive trust policies, exposing them to exfiltration.
// impact: An attacker who gains access to the binary or environment can extract long-term AWS credentials.
// improvement: Use short-lived credentials via STS AssumeRole, IAM roles for service accounts (IRSA), or environment-based credential chains.

use aws_sdk_s3::Client;
use aws_types::credentials::SharedCredentialsProvider;

#[tokio::main]
async fn main() {
    let creds = aws_sdk_s3::config::Credentials::new(
        "AKIAIOSFODNN7EXAMPLE",
        "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY",
        None,
        None,
        "hardcoded",
    );

    let config = aws_sdk_s3::Config::builder()
        .credentials_provider(SharedCredentialsProvider::new(creds))
        .region(aws_sdk_s3::config::Region::new("us-east-1"))
        .build();

    let client = Client::from_conf(config);
    // ...
}
