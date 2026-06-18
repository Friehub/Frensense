// Vulnerable: SSLv3 is insecure because it has known vulnerabilities. Starting with go1.14, SSLv3 will be removed. Instead, use 'tls.VersionTLS13'.
// Pattern: tls.Config{..., MinVersion: $TLS.VersionSSL30, ...}
function vulnerable() {
  // TODO: implement pattern match
}
