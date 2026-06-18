// Vulnerable: Detects direct creations of $HTTPS servers that don't disallow SSL v2, SSL v3, and TLS v1. These protocols are deprecated due to POODLE, man in the middle attacks, and other vulnerabilities.
// Pattern: $HTTPS.createServer(...).$FUNC(...);
function vulnerable() {
  // TODO: implement pattern match
}
