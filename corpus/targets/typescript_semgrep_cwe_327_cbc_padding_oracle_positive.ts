// Vulnerable: Using CBC with PKCS5Padding is susceptible to padding oracle attacks. A malicious actor could discern the difference between plaintext with valid or invalid padding. Further, CBC mode does not include any integrity checks. Use 'AES/GCM/NoPadding' instead.
// Pattern: "=~/.*\/CBC\/PKCS5Padding/"
function vulnerable() {
  // TODO: implement pattern match
}
