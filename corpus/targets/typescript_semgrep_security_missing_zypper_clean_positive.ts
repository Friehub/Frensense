// Vulnerable: This zypper command does not end with '&& zypper clean'. Running 'zypper clean' will remove cached data and reduce package size. (This must be performed in the same RUN step.)
// Pattern: RUN ... zypper $COMMAND ...
function vulnerable() {
  // TODO: implement pattern match
}
