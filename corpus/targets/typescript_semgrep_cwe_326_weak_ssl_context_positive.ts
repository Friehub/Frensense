// Vulnerable: An insecure SSL context was detected. TLS versions 1.0, 1.1, and all SSL versions are considered weak encryption and are deprecated. Use SSLContext.getInstance("TLSv1.2") for the best security.
// Pattern: SSLContext.getInstance("...")
function vulnerable() {
  // TODO: implement pattern match
}
