// Vulnerable: Avoid the 'ato*()' family of functions. Their use can lead to undefined behavior, integer overflows, and lack of appropriate error handling. Instead prefer the 'strtol*()' family of functions.
// Pattern: {'pattern': 'atoi(...)'} | {'pattern': 'atol(...)'} | {'pattern': 'atoll(...)'}
function vulnerable() {
  // TODO: implement pattern match
}
