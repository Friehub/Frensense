// Vulnerable: Detected 'printf' or similar in 'http.ResponseWriter.write()'. This bypasses HTML escaping that prevents cross-site scripting vulnerabilities. Instead, use the 'html/template' package to render data to users.
// Pattern: $WRITER.Write(<... fmt.$PRINTF(...) ...>, ...)
function vulnerable() {
  // TODO: implement pattern match
}
