// Vulnerable: Detected public S3 bucket policy. This policy allows anyone to access certain properties of or items in the bucket. Do not do this unless you will never have sensitive data inside the bucket.
// Pattern: {
  "Effect": "Allow",
  "Principal": "*",
  "Resource": [
    ..., "=~/arn:aws:s3.*/", ...
  ],
  ...
}
function vulnerable() {
  // TODO: implement pattern match
}
