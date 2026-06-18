// Vulnerable: Default session middleware settings: `path` not set. It indicates the path of the cookie; use it to compare against the request path. If this and domain match, then send the cookie in the request.
// Pattern: $SESSION(...)
function vulnerable() {
  // TODO: implement pattern match
}
