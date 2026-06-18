// Vulnerable: You are using the outdated PKCS#1 v1.5 encryption padding for your RSA key. Use the OAEP padding instead.
// Pattern: {'pattern': '(RSAPKCS1KeyExchangeFormatter $FORMATER).CreateKeyExchange(...);'} | {'pattern': '(RSAPKCS1KeyExchangeDeformatter $DEFORMATER).DecryptKeyExchange(...);'}
function vulnerable() {
  // TODO: implement pattern match
}
