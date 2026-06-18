// Vulnerable: In addition to debug statements potentially logging data excessively, debug statements also contribute to longer transactions and consume Apex CPU time even when debug logs are not being captured.
// Pattern: System.debug(...)
function vulnerable() {
  // TODO: implement pattern match
}
