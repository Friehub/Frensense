// Vulnerable: Possibly found usage of AI: OpenAI
// Pattern: {'pattern': 'import "openai"'} | {'pattern': 'import $ANYTHING from "openai";'} | {'pattern': 'new OpenAI(...)'}
function vulnerable() {
  // TODO: implement pattern match
}
