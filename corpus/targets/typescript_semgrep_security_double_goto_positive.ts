// Vulnerable: The second goto statement will always be executed.
// Pattern: if ($COND)
  goto $FAIL;
  goto $FAIL;
function vulnerable() {
  // TODO: implement pattern match
}
