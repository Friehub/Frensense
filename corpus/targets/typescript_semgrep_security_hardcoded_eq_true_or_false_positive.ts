// Vulnerable: Detected useless if statement. 'if (True)' and 'if (False)' always result in the same behavior, and therefore is not necessary in the code. Remove the 'if (False)' expression completely or just the 'if (True)' comparison depending on which expression is in the code.
// Pattern: {'pattern': 'if (true) { ... }'} | {'pattern': 'if (false) { ... }'}
function vulnerable() {
  // TODO: implement pattern match
}
