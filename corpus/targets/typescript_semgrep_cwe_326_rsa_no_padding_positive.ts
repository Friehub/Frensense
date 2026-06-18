// Vulnerable: Using RSA without OAEP mode weakens the encryption.
// Pattern: $CIPHER.getInstance("=~/RSA/[Nn][Oo][Nn][Ee]/NoPadding/")
function vulnerable() {
  // TODO: implement pattern match
}
