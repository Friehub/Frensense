// Vulnerable: `undefined` is not a reserved keyword in Javascript, so this is "valid" Javascript but highly confusing and likely to result in bugs.
// Pattern: {'pattern': 'undefined = $X;'} | {'pattern': 'var undefined = $X;'} | {'pattern': 'let undefined = $X;'}
function vulnerable() {
  // TODO: implement pattern match
}
