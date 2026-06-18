// Fixed: Anthropic response content accessed without checking stop_reason. Check response.stop_reason to handle cases where the model stops unexpectedly (e.g., due to max_tokens or content filtering).
// Apply appropriate sanitization
function safe() {
  // TODO: implement fix
}
