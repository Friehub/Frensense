// Vulnerable: Hugging Face API token is hardcoded in source code. Use environment variables or a secrets manager instead. See https://huggingface.co/docs/hub/en/security-tokens
// Pattern: new InferenceClient("$KEY", ...)
function vulnerable() {
  // TODO: implement pattern match
}
