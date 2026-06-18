// Vulnerable: The package `net/http/cgi` is on the import blocklist.  The package is vulnerable to httpoxy attacks (CVE-2015-5386). It is recommended to use `net/http` or a web framework to build a web application instead.
// Pattern: {'patterns': [{'pattern-inside': 'import "net/http/cgi"\n...\n'}, {'pattern': 'cgi.$FUNC(...)\n'}]}
function vulnerable() {
  // TODO: implement pattern match
}
