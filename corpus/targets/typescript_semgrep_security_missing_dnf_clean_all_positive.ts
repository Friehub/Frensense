// Vulnerable: This dnf command does not end with '&& dnf clean all'. Running 'dnf clean all' will remove cached data and reduce package size. (This must be performed in the same RUN step.)
// Pattern: RUN ... dnf ...
function vulnerable() {
  // TODO: implement pattern match
}
