// Vulnerable: Cohere API key is hardcoded in source code. Use environment variables or a secrets manager instead.
// Pattern: new CohereClient({token: "$KEY", ...})
function vulnerable() {
  // TODO: implement pattern match
}
