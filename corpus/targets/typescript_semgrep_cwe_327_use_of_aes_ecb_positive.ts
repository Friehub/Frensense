// Vulnerable: Use of AES with ECB mode detected. ECB doesn't provide message confidentiality and  is not semantically secure so should not be used. Instead, use a strong, secure cipher: Cipher.getInstance("AES/CBC/PKCS7PADDING"). See https://owasp.org/www-community/Using_the_Java_Cryptographic_Extensions for more information.
// Pattern: $CIPHER.getInstance("=~/AES/ECB.*/")
function vulnerable() {
  // TODO: implement pattern match
}
