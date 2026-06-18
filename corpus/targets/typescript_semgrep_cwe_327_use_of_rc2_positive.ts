// Vulnerable: Use of RC2 was detected. RC2 is vulnerable to related-key attacks, and is therefore considered non-compliant. Instead, use a strong, secure cipher: Cipher.getInstance("AES/CBC/PKCS7PADDING"). See https://owasp.org/www-community/Using_the_Java_Cryptographic_Extensions for more information.
// Pattern: $CIPHER.getInstance("RC2")
function vulnerable() {
  // TODO: implement pattern match
}
