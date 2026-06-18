// Vulnerable: OpenAI chat completion created without a 'user' parameter. Including a unique user identifier helps OpenAI detect and prevent abuse. See https://platform.openai.com/docs/guides/safety-best-practices
// Pattern: $CLIENT.chat.completions.create({...})
function vulnerable() {
  // TODO: implement pattern match
}
