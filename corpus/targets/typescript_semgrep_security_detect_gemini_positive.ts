// Vulnerable: Possibly found usage of AI: Gemini
// Pattern: {'pattern': 'import "@google/generative-ai"'} | {'pattern': 'import $ANYTHING from "@google/generative-ai";'} | {'pattern': 'new GoogleGenerativeAI(...)'}
function vulnerable() {
  // TODO: implement pattern match
}
