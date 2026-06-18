// Vulnerable: Calling `empty` on a boolean expression may be an indication that a parenthesis is misplaced.
// Pattern: {'pattern': 'empty($A && $B)\n'} | {'pattern': 'empty($A || $B)\n'}
function vulnerable() {
  // TODO: implement pattern match
}
