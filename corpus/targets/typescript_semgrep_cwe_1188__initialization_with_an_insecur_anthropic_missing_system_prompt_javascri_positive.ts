// Vulnerable: Anthropic messages.create() called without a 'system' parameter. A system prompt helps set behavioral guidelines and safety boundaries for the model.
// Pattern: $CLIENT.messages.create({...})
function vulnerable() {
  // TODO: implement pattern match
}
