// Vulnerable: Detected wildcard access granted to sts:AssumeRole. This means anyone with your AWS account ID and the name of the role can assume the role. Instead, limit to a specific identity in your account, like this: `arn:aws:iam::<account_id>:root`.
// Pattern: "Principal": {..., "AWS": "*", ...}
function vulnerable() {
  // TODO: implement pattern match
}
