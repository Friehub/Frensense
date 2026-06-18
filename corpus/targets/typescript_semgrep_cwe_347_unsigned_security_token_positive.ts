// Vulnerable: Accepting unsigned security tokens as valid security tokens allows an attacker to remove its signature and potentially forge an identity. As a fix, set RequireSignedTokens to be true.
// Pattern: RequireSignedTokens = false
function vulnerable() {
  // TODO: implement pattern match
}
