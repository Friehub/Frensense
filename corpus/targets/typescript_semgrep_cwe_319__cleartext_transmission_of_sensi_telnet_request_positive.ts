// Vulnerable: Checks for attempts to connect through telnet. This is insecure as the telnet protocol supports no encryption, and data passes through unencrypted.
// Pattern: $TELNETCLIENT = new TelnetClient(...);
...
$TELNETCLIENT.connect(...);
function vulnerable() {
  // TODO: implement pattern match
}
