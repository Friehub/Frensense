// Vulnerable: Default session middleware settings: `domain` not set. It indicates the domain of the cookie; use it to compare against the domain of the server in which the URL is being requested. If they match, then check the path attribute next.
// Pattern: $SESSION(...)
function vulnerable() {
  // TODO: implement pattern match
}
