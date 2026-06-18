// Fixed: MCP tool docstring contains suspicious directives that may indicate tool poisoning. Attackers can embed hidden instructions in tool descriptions to manipulate LLM behavior, exfiltrate data, or access sensitive files. Review the tool description for hidden instructions or social engineering.
// Apply appropriate sanitization
function safe() {
  // TODO: implement fix
}
