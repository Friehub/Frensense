// Vulnerable: Anthropic API key is hardcoded in source code. Use environment variables or a secrets manager instead.
// Pattern: Anthropic::Client.new(api_key: "$KEY", ...)
function vulnerable() {
  // TODO: implement pattern match
}
