// Vulnerable: Found a configuration file where the HttpOnly attribute is not set to true. Setting `http_only` to true makes sure that your cookies are inaccessible from Javascript, which mitigates XSS attacks. Instead, set the 'http_only' like so: `http_only` => true 
// Pattern: 'cookie'
function vulnerable() {
  // TODO: implement pattern match
}
