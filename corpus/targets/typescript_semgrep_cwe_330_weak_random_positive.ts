// Vulnerable: Detected use of the functions `Math.random()` or `java.util.Random()`. These are both not cryptographically strong random number generators (RNGs). If you are using these RNGs to create passwords or secret tokens, use `java.security.SecureRandom` instead.
// Pattern: {'pattern': 'new java.util.Random(...).$FUNC(...)\n'} | {'pattern': 'java.lang.Math.random(...)\n'}
function vulnerable() {
  // TODO: implement pattern match
}
