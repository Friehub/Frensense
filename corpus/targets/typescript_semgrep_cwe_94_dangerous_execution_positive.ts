// Vulnerable: Detected non-static script inside otto VM. Audit the input to 'VM.Run'. If unverified user data can reach this call site, this is a code injection vulnerability. A malicious actor can inject a malicious script to execute arbitrary code.
// Pattern: $VM.Run(...)
function vulnerable() {
  // TODO: implement pattern match
}
