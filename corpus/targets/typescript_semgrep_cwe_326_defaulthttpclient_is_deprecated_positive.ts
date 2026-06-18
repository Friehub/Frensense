// Vulnerable: DefaultHttpClient is deprecated. Further, it does not support connections using TLS1.2, which makes using DefaultHttpClient a security hazard. Use SystemDefaultHttpClient instead, which supports TLS1.2.
// Pattern: DefaultHttpClient(...)
function vulnerable() {
  // TODO: implement pattern match
}
