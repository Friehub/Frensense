// Vulnerable: AI/LLM API key found hardcoded in source code. Detected key prefix matches a known AI provider (OpenAI, Anthropic, Google, Hugging Face). Use environment variables or a secrets manager instead.
// Pattern: {'patterns': [{'pattern': '$VAR := "$KEY"'}, {'metavariable-regex': {'metavariable': '$KEY', 'regex': '^(sk-[a-zA-Z0-9]{20,}|sk-ant-[a-zA-Z0-9-]{20,}|sk-proj-[a-zA-Z0-9-]{20,}|AIza[a-zA-Z0-9_-]{30,}|hf_[a-zA-Z0-9]{20,})'}}]} | {'patterns': [{'pattern': 'var $VAR = "$KEY"'}, {'metavariable-regex': {'metavariable': '$KEY', 'regex': '^(sk-[a-zA-Z0-9]{20,}|sk-ant-[a-zA-Z0-9-]{20,}|sk-proj-[a-zA-Z0-9-]{20,}|AIza[a-zA-Z0-9_-]{30,}|hf_[a-zA-Z0-9]{20,})'}}]}
function vulnerable() {
  // TODO: implement pattern match
}
