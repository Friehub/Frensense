// Vulnerable: OpenAI chat completion created without a system message. A system message helps establish behavioral guidelines and safety boundaries for the model.
// Pattern: $CLIENT.chat.completions.create({..., messages: $MSGS, ...})
function vulnerable() {
  // TODO: implement pattern match
}
