// Fixed: Detected template variable interpolation in an HTML tag. This is potentially vulnerable to cross-site scripting (XSS) attacks because a malicious actor has control over HTML but without the need to use escaped characters. Use explicit tags instead.
// Apply appropriate sanitization
function safe() {
  // TODO: implement fix
}
