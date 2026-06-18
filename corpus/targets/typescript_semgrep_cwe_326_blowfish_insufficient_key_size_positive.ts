// Vulnerable: Using less than 128 bits for Blowfish is considered insecure. Use 128 bits or more, or switch to use AES instead.
// Pattern: $KEYGEN = KeyGenerator.getInstance("Blowfish");
...
$KEYGEN.init($SIZE);
function vulnerable() {
  // TODO: implement pattern match
}
