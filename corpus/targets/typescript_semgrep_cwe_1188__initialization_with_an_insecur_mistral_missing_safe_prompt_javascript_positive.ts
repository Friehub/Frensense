// Vulnerable: Mistral chat completion called without 'safe_prompt' parameter. Setting safePrompt=true enables Mistral's built-in safety guardrailing. See https://docs.mistral.ai/capabilities/guardrailing/
// Pattern: $CLIENT.chat.complete({...})
function vulnerable() {
  // TODO: implement pattern match
}
