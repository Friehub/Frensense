// Vulnerable: Checks for requests sent via Unirest to http:// URLS. This is dangerous because the server is attempting to connect to a website that does not encrypt traffic with TLS. Instead, send requests only to https:// URLS.
// Pattern: {'pattern': 'Unirest.get("=~/[hH][tT][tT][pP]://.*/")\n'} | {'pattern': 'Unirest.post("=~/[hH][tT][tT][pP]://.*/")\n'}
function vulnerable() {
  // TODO: implement pattern match
}
