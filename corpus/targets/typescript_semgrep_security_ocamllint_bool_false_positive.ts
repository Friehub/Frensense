// Vulnerable: Comparison to boolean. Just use `not $X`
// Pattern: {'pattern': '$X = false'} | {'pattern': '$X == false'} | {'pattern': '$X <> true'}
function vulnerable() {
  // TODO: implement pattern match
}
