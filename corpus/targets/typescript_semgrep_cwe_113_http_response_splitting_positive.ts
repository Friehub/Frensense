// Vulnerable: Older Java application servers are vulnerable to HTTP response splitting, which may occur if an HTTP request can be injected with CRLF characters. This finding is reported for completeness; it is recommended to ensure your environment is not affected by testing this yourself.
// Pattern: {'pattern': '$VAR = $REQ.getParameter(...);\n...\n$COOKIE = new Cookie(..., $VAR, ...);\n...\n$RESP.addCookie($COOKIE, ...);\n'} | {'patterns': [{'pattern-inside': '$RETTYPE $FUNC(...,@PathVariable $TYPE $VAR, ...) {\n  ...\n}\n'}, {'pattern': '$COOKIE = new Cookie(..., $VAR, ...);\n...\n$RESP.addCookie($COOKIE, ...);\n'}]}
function vulnerable() {
  // TODO: implement pattern match
}
