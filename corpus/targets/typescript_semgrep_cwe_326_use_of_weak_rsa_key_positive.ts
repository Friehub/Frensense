// Vulnerable: RSA keys should be at least 2048 bits based on NIST recommendation.
// Pattern: KeyPairGenerator $KEY = $G.getInstance("RSA");
...
$KEY.initialize($BITS);
function vulnerable() {
  // TODO: implement pattern match
}
