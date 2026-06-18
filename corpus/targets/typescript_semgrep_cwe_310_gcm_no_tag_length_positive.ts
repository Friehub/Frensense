// Vulnerable: The call to 'createDecipheriv' with the Galois Counter Mode (GCM) mode of operation is missing an expected authentication tag length. If the expected authentication tag length is not specified or otherwise checked, the application might be tricked into verifying a shorter-than-expected authentication tag. This can be abused by an attacker to spoof ciphertexts or recover the implicit authentication key of GCM, allowing arbitrary forgeries.
// Pattern: $CRYPTO.createDecipheriv('$ALGO', $KEY, $IV)
function vulnerable() {
  // TODO: implement pattern match
}
