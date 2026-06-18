// Vulnerable: Detected 'Fprintf' or similar writing to 'http.ResponseWriter'. This bypasses HTML escaping that prevents cross-site scripting vulnerabilities. Instead, use the 'html/template' package to render data to users.
// Pattern: fmt.$PRINTF($WRITER, ...)
function vulnerable() {
  // TODO: implement pattern match
}
