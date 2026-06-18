// Vulnerable: The 'final' call of a Decipher object checks the authentication tag in a mode for authenticated encryption. Failing to call 'final' will invalidate all integrity guarantees of the released ciphertext.
// Pattern: $DECIPHER = $CRYPTO.createDecipheriv('$ALGO', ...)
...
$DECIPHER.update(...)
function vulnerable() {
  // TODO: implement pattern match
}
