// Vulnerable: Default session middleware settings: `setSecure` not set to true. This ensures that the cookie is sent only over HTTPS to prevent cross-site scripting attacks.
// Pattern: $COOKIE = new Cookie($...ARGS);
function vulnerable() {
  // TODO: implement pattern match
}
