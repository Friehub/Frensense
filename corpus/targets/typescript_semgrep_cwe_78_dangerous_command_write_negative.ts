// Fixed: Detected non-static command inside Write. Audit the input to '$CW.Write'. If unverified user data can reach this call site, this is a code injection vulnerability. A malicious actor can inject a malicious script to execute arbitrary code.
// Apply appropriate sanitization
function safe() {
  // TODO: implement fix
}
