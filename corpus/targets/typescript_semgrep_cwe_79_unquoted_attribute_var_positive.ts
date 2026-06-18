// Vulnerable: Detected a unquoted template variable as an attribute. If unquoted, a malicious actor could inject custom JavaScript handlers. To fix this, add quotes around the template expression, like this: "{{ expr }}".
// Pattern: {{ ... }}
function vulnerable() {
  // TODO: implement pattern match
}
