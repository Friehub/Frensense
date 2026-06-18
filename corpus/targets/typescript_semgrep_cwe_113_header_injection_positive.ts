// Vulnerable: The $$VARIABLE path parameter is added as a header in the response. This could allow an attacker to inject a newline and add a new header into the response. This is called HTTP response splitting. To fix, do not allow whitespace in the path parameter: '[^\s]+'.
// Pattern: location ... <$VARIABLE> ... {
  ...
  add_header ... $$VARIABLE
  ...
}
function vulnerable() {
  // TODO: implement pattern match
}
