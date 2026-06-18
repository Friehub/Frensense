// Vulnerable: Anthropic messages.create() called without 'max_tokens' parameter. Setting max_tokens prevents unexpectedly long or expensive responses.
// Pattern: $CLIENT.messages.create({...})
function vulnerable() {
  // TODO: implement pattern match
}
