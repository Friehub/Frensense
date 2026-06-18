// Vulnerable: Cohere chat called without explicit 'safety_mode' parameter. Set safetyMode to 'STRICT' or 'CONTEXTUAL' to explicitly configure content safety guardrails. See https://docs.cohere.com/docs/safety-modes
// Pattern: $CLIENT.chat({...})
function vulnerable() {
  // TODO: implement pattern match
}
