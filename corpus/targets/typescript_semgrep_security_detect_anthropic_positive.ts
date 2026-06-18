// Vulnerable: Possibly found usage of AI: Anthropic
// Pattern: {'pattern': 'import "@anthropic-ai"'} | {'pattern': 'import $ANYTHING from "@anthropic-ai";'} | {'pattern': 'new Anthropic(...)'}
function vulnerable() {
  // TODO: implement pattern match
}
