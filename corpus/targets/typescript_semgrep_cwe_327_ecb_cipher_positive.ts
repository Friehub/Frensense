// Vulnerable: Cipher in ECB mode is detected. ECB mode produces the same output for the same input each time which allows an attacker to intercept and replay the data. Further, ECB mode does not provide any integrity checking. See https://find-sec-bugs.github.io/bugs.htm#CIPHER_INTEGRITY.
// Pattern: Cipher $VAR = $CIPHER.getInstance($MODE);
function vulnerable() {
  // TODO: implement pattern match
}
