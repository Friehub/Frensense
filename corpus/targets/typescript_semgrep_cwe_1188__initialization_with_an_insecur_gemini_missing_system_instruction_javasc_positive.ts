// Vulnerable: Gemini GenerativeModel created without 'systemInstruction' parameter. A system instruction helps establish behavioral guidelines and safety boundaries for the model. See https://ai.google.dev/gemini-api/docs/safety-guidance
// Pattern: $CLIENT.getGenerativeModel({...})
function vulnerable() {
  // TODO: implement pattern match
}
