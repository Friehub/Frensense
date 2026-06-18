// Vulnerable: The function `openssl_decrypt` returns either a string of the decrypted data on success or `false` on failure. If the failure case is not handled, this could lead to undefined behavior in your application. Please handle the case where `openssl_decrypt` returns `false`.
// Pattern: openssl_decrypt(...);
function vulnerable() {
  // TODO: implement pattern match
}
