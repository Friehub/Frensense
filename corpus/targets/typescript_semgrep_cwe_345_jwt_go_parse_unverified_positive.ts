// Vulnerable: Detected the decoding of a JWT token without a verify step. Don't use `ParseUnverified` unless you know what you're doing This method parses the token but doesn't validate the signature. It's only ever useful in cases where you know the signature is valid (because it has been checked previously in the stack) and you want to extract values from it.
// Pattern: $JWT.ParseUnverified(...)
function vulnerable() {
  // TODO: implement pattern match
}
