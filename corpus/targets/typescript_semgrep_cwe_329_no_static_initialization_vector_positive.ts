// Vulnerable: Initialization Vectors (IVs) for block ciphers should be randomly generated each time they are used. Using a static IV means the same plaintext encrypts to the same ciphertext every time, weakening the strength of the encryption.
// Pattern: {'pattern': 'byte[] $IV = {\n    ...\n};\n...\nnew IvParameterSpec($IV, ...);\n'} | {'pattern': 'class $CLASS {\n    byte[] $IV = {\n        ...\n    };\n    ...\n    $METHOD(...) {\n        ...\n        new IvParameterSpec($IV, ...);\n        ...\n    }\n}\n'}
function vulnerable() {
  // TODO: implement pattern match
}
