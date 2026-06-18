// Vulnerable: found prompt() call; should this be in production code?
// Pattern: {'pattern': 'prompt()'} | {'pattern': 'prompt($X)'} | {'pattern': 'prompt($X, $Y)'}
function vulnerable() {
  // TODO: implement pattern match
}
