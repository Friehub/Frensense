// Vulnerable: Checks for requests sent via http.NewRequest to http:// URLS. This is dangerous because the server is attempting to connect to a website that does not encrypt traffic with TLS. Instead, send requests only to https:// URLS.
// Pattern: http.NewRequest(..., "=~/[hH][tT][tT][pP]://.*/", ...)
function vulnerable() {
  // TODO: implement pattern match
}
