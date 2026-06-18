// Vulnerable: Cannot determine what '$UNK' is and it is used with a '<script>' tag. This could be susceptible to cross-site scripting (XSS). Ensure '$UNK' is not externally controlled, or sanitize this data.
// Pattern: $OTHERFUNC(..., <... "=~/.*<script.*/" ...>, ...)
function vulnerable() {
  // TODO: implement pattern match
}
