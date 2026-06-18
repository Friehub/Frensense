// Vulnerable: Detected template variable interpolation in a JavaScript template string. This is potentially vulnerable to cross-site scripting (XSS) attacks because a malicious actor has control over JavaScript but without the need to use escaped characters. Instead, obtain this variable outside of the template string and ensure your template is properly escaped.
// Pattern: ` ... {{ ... }} ...`
function vulnerable() {
  // TODO: implement pattern match
}
