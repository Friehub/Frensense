// Vulnerable: The rule makes sure you are using randomly generated IVs and keys for Crypto calls. Hard-coding these values greatly compromises the security of encrypted data.
// Pattern: {'pattern': "Blob $IV = Blob.valueOf('$STRING');...Crypto.encrypt($ONE, $TWO, $IV, $FOUR);"} | {'pattern': "Blob $IV = Blob.valueOf('$STRING');...Crypto.decrypt($ONE, $TWO, $IV, $FOUR);"} | {'pattern': "Blob $KEY = Blob.valueOf('$STRING');...Crypto.encrypt($ONE, $KEY, $THREE, $FOUR);"}
function vulnerable() {
  // TODO: implement pattern match
}
