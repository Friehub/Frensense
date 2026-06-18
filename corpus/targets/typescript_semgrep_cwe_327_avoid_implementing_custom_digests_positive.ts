// Vulnerable: Cryptographic algorithms are notoriously difficult to get right. By implementing a custom message digest, you risk introducing security issues into your program. Use one of the many sound message digests already available to you: MessageDigest sha256Digest = MessageDigest.getInstance("SHA256");
// Pattern: class $CLASS extends MessageDigest {
  ...
}
function vulnerable() {
  // TODO: implement pattern match
}
