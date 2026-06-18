// Vulnerable: Detected non-static command inside Write. Audit the input to '$CW.Write'. If unverified user data can reach this call site, this is a code injection vulnerability. A malicious actor can inject a malicious script to execute arbitrary code.
// Pattern: $CW.Write($BYTE)
function vulnerable() {
  // TODO: implement pattern match
}
