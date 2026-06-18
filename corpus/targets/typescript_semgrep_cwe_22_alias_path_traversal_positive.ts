// Vulnerable: The alias in this location block is subject to a path traversal because the location path does not end in a path separator (e.g., '/'). To fix, add a path separator to the end of the path.
// Pattern: location $...LOCATION {
  ...
  alias .../;
  ...
}
function vulnerable() {
  // TODO: implement pattern match
}
