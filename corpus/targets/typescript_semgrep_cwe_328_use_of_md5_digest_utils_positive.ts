// Vulnerable: Detected MD5 hash algorithm which is considered insecure. MD5 is not collision resistant and is therefore not suitable as a cryptographic signature. Use HMAC instead.
// Pattern: $DU.$GET_ALGO().digest(...)
function vulnerable() {
  // TODO: implement pattern match
}
