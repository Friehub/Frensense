// Vulnerable: Cohere safety mode explicitly set to 'OFF', disabling all safety guardrails. Use 'STRICT' or 'CONTEXTUAL' instead. See https://docs.cohere.com/docs/safety-modes
// Pattern: $CLIENT.chat({..., safetyMode: "OFF", ...})
function vulnerable() {
  // TODO: implement pattern match
}
