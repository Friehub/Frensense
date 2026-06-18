// Vulnerable: Detected 'io.WriteString()' writing directly to 'http.ResponseWriter'. This bypasses HTML escaping that prevents cross-site scripting vulnerabilities. Instead, use the 'html/template' package to render data to users.
// Pattern: io.WriteString($WRITER, $STRING)
function vulnerable() {
  // TODO: implement pattern match
}
