// Vulnerable: Detected the decoding of a JWT token without a verify step. JWT tokens must be verified before use, otherwise the token's integrity is unknown. This means a malicious actor could forge a JWT token with any claims. Call '.verify()' before using the token.
// Pattern: com.auth0.jwt.JWT.decode(...);
function vulnerable() {
  // TODO: implement pattern match
}
