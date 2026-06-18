// Vulnerable: Detected a hardcoded hmac key. Avoid hardcoding secrets and consider using an alternate option such as reading the secret from a config file or using an environment variable.
// Pattern: {'pattern': "$CRYPTO.createHmac($ALGO, '...')"} | {'patterns': [{'pattern-inside': "const $SECRET = '...'\n...\n"}, {'pattern': '$CRYPTO.createHmac($ALGO, $SECRET)'}]}
function vulnerable() {
  // TODO: implement pattern match
}
