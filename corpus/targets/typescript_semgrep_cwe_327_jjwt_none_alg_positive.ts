// Vulnerable: Detected use of the 'none' algorithm in a JWT token. The 'none' algorithm assumes the integrity of the token has already been verified. This would allow a malicious actor to forge a JWT token that will automatically be verified. Do not explicitly use the 'none' algorithm. Instead, use an algorithm such as 'HS256'.
// Pattern: io.jsonwebtoken.Jwts.builder();
function vulnerable() {
  // TODO: implement pattern match
}
