// Vulnerable: Detected an insecure CipherSuite via the 'tls' module. This suite is considered weak. Use the function 'tls.CipherSuites()' to get a list of good cipher suites. See https://golang.org/pkg/crypto/tls/#InsecureCipherSuites for why and what other cipher suites to use.
// Pattern: {'pattern': 'tls.Config{..., CipherSuites: []$TYPE{..., tls.TLS_RSA_WITH_RC4_128_SHA, ...}}\n'} | {'pattern': 'tls.Config{..., CipherSuites: []$TYPE{..., tls.TLS_RSA_WITH_3DES_EDE_CBC_SHA, ...}}\n'} | {'pattern': 'tls.Config{..., CipherSuites: []$TYPE{..., tls.TLS_RSA_WITH_AES_128_CBC_SHA256, ...}}\n'}
function vulnerable() {
  // TODO: implement pattern match
}
