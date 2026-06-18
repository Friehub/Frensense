// Vulnerable: Plaintext API key or token detected in MCP configuration file. Hardcoded secrets in config files risk exposure through version control or file sharing. Use environment variable references or a secrets manager instead.
// Pattern: {'pattern-regex': ':\\s*"sk-[a-zA-Z0-9]{20,}"'} | {'pattern-regex': ':\\s*"sk-ant-[a-zA-Z0-9\\-]{20,}"'} | {'pattern-regex': ':\\s*"sk-proj-[a-zA-Z0-9\\-]{20,}"'}
function vulnerable() {
  // TODO: implement pattern match
}
