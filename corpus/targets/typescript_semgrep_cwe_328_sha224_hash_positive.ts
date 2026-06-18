// Vulnerable: This code uses a 224-bit hash function, which is deprecated or disallowed in some security policies. Consider updating to a stronger hash function such as SHA-384 or higher to ensure compliance and security.
// Pattern: {'pattern': "hash('sha224', ...);"} | {'pattern': "hash('sha512/224', ...);"} | {'pattern': "hash('sha3-224', ...);"}
function vulnerable() {
  // TODO: implement pattern match
}
