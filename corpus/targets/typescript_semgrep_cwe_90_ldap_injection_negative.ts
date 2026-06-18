// Fixed: Detected non-constant data passed into an LDAP query. If this data can be controlled by an external user, this is an LDAP injection. Ensure data passed to an LDAP query is not controllable; or properly sanitize the data.
// Apply appropriate sanitization
function safe() {
  // TODO: implement fix
}
