// Vulnerable: Checks for lack of usage of the "secure: true" option when sending ftp requests through the nodejs ftp module. This leads to unencrypted traffic being sent to the ftp server. There are other options such as "implicit" that still does not encrypt all traffic. ftp is the most utilized npm ftp module.
// Pattern: $C.connect($OPTIONS,...);
function vulnerable() {
  // TODO: implement pattern match
}
