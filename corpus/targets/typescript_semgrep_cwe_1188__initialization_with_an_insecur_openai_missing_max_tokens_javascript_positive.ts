// Vulnerable: OpenAI chat completion created without 'max_tokens' parameter. Setting max_tokens prevents unexpectedly long or expensive responses and limits potential abuse. See https://developers.openai.com/api/docs/guides/safety-best-practices
// Pattern: $CLIENT.chat.completions.create({...})
function vulnerable() {
  // TODO: implement pattern match
}
