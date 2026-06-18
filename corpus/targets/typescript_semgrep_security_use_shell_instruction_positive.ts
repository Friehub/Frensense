// Vulnerable: Use the SHELL instruction to set the default shell instead of overwriting '/bin/sh'.
// Pattern: RUN ln ... $SHELL /bin/sh
function vulnerable() {
  // TODO: implement pattern match
}
