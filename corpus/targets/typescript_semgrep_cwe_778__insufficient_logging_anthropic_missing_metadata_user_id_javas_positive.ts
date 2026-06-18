// Vulnerable: Anthropic messages.create() called without 'metadata' parameter. Pass a metadata object with a hashed user_id to enable abuse tracking and policy enforcement. See https://docs.anthropic.com/en/api/messages
// Pattern: $CLIENT.messages.create({...})
function vulnerable() {
  // TODO: implement pattern match
}
