// Vulnerable: OpenAI API key is hardcoded in source code. Use environment variables or a secrets manager instead.
// Pattern: OpenAI::Client.new(access_token: "$KEY", ...)
function vulnerable() {
  // TODO: implement pattern match
}
