// Fixed: A server-side template injection occurs when an attacker is able to use native template syntax to inject a malicious payload into a template, which is then executed server-side. When using "html/template" always check that user inputs are validated and sanitized before included within the template.
// Apply appropriate sanitization
function safe() {
  // TODO: implement fix
}
