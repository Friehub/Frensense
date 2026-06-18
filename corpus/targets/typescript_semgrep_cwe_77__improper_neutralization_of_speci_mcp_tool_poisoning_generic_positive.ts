// Vulnerable: MCP tool docstring contains suspicious directives that may indicate tool poisoning. Attackers can embed hidden instructions in tool descriptions to manipulate LLM behavior, exfiltrate data, or access sensitive files. Review the tool description for hidden instructions or social engineering.
// Pattern: {'pattern-regex': '"""[^"]*<IMPORTANT>[^"]*"""'} | {'pattern-regex': '"""[^"]*~/\\.ssh[^"]*"""'} | {'pattern-regex': '"""[^"]*~/\\.cursor/mcp\\.json[^"]*"""'}
function vulnerable() {
  // TODO: implement pattern match
}
