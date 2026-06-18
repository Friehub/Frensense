// Vulnerable: Digest uses MD5 and should not be used for security purposes. Consider using SHA256 instead.
// Pattern: {'pattern': 'Digest.string'} | {'pattern': 'Digest.bytes'} | {'pattern': 'Digest.substring'}
function vulnerable() {
  // TODO: implement pattern match
}
