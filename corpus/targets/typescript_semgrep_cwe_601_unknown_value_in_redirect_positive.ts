// Vulnerable: It looks like '$UNK' is read from user input and it is used to as a redirect. Ensure '$UNK' is not externally controlled, otherwise this is an open redirect.
// Pattern: $RES.redirect(..., <... $UNK ...>, ...)
function vulnerable() {
  // TODO: implement pattern match
}
