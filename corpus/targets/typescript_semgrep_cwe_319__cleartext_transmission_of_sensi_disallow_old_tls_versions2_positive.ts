// Vulnerable: Detects creations of $HTTPS servers from option objects that don't disallow SSL v2, SSL v3, and TLS v1. These protocols are deprecated due to POODLE, man in the middle attacks, and other vulnerabilities.
// Pattern: $OPTIONS = {};
...
$HTTPS.createServer($OPTIONS, ...);
function vulnerable() {
  // TODO: implement pattern match
}
