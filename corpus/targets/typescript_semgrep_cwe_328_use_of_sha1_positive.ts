// Vulnerable: Detected SHA1 hash algorithm which is considered insecure. SHA1 is not collision resistant and is therefore not suitable as a cryptographic signature. Instead, use PBKDF2 for password hashing or SHA256 or SHA512 for other hash function applications.
// Pattern: {'patterns': [{'pattern': 'java.security.MessageDigest.getInstance("$ALGO", ...);\n'}, {'metavariable-regex': {'metavariable': '$ALGO', 'regex': '(SHA1|SHA-1)'}}]} | {'pattern': '$DU.getSha1Digest().digest(...)\n'}
function vulnerable() {
  // TODO: implement pattern match
}
