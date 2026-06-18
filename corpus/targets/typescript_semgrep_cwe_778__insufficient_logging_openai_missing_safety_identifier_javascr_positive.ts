// Vulnerable: OpenAI Responses API called without 'safety_identifier' parameter. Include a hashed user identifier to enable abuse monitoring and safety checks. See https://developers.openai.com/api/docs/guides/safety-checks
// Pattern: $CLIENT.responses.create({...})
function vulnerable() {
  // TODO: implement pattern match
}
