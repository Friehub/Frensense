// Vulnerable: Possibly found usage of AI: Mistral
// Pattern: {'pattern': 'import "@mistralai"'} | {'pattern': 'new MistralClient(...)'} | {'pattern': '$CLIENT.chat({model: ...})'}
function vulnerable() {
  // TODO: implement pattern match
}
