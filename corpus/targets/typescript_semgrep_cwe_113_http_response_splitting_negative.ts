// Fixed: Older Java application servers are vulnerable to HTTP response splitting, which may occur if an HTTP request can be injected with CRLF characters. This finding is reported for completeness; it is recommended to ensure your environment is not affected by testing this yourself.
// Apply appropriate sanitization
function safe() {
  // TODO: implement fix
}
