// Vulnerable: Default session middleware settings: `httpOnly` not set. It ensures the cookie is sent only over HTTP(S), not client JavaScript, helping to protect against cross-site scripting attacks.
// Pattern: $SESSION(...)
function vulnerable() {
  // TODO: implement pattern match
}
