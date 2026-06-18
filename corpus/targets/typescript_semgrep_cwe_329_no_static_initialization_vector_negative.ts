// Fixed: Initialization Vectors (IVs) for block ciphers should be randomly generated each time they are used. Using a static IV means the same plaintext encrypts to the same ciphertext every time, weakening the strength of the encryption.
// Apply appropriate sanitization
function safe() {
  // TODO: implement fix
}
