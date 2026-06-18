// Vulnerable: Detected MD5 hash algorithm which is considered insecure. MD5 is not collision resistant and is therefore not suitable as a cryptographic signature. Use SHA256 or SHA3 instead.
// Pattern: {'pattern': 'java.security.MessageDigest.getInstance("MD5")\n'} | {'pattern': 'org.apache.commons.codec.digest.DigestUtils.getMd5Digest()\n'}
function vulnerable() {
  // TODO: implement pattern match
}
