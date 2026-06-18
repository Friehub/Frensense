// Vulnerable: This code uses a 224-bit hash function, which is deprecated or disallowed in some security policies. Consider updating to a stronger hash function such as SHA-384 or higher to ensure compliance and security.
// Pattern: {'pattern': 'org.apache.commons.codec.digest.DigestUtils.getSha3_224Digest()'} | {'pattern': 'org.apache.commons.codec.digest.DigestUtils.getSha512_224Digest()'} | {'pattern': 'org.apache.commons.codec.digest.DigestUtils.sha3_224(...)'}
function vulnerable() {
  // TODO: implement pattern match
}
