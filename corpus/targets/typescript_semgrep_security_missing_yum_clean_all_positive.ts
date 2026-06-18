// Vulnerable: This yum command does not end with '&& yum clean all'. Running 'yum clean all' will remove cached data and reduce package size. (This must be performed in the same RUN step.)
// Pattern: yum $COMMAND
function vulnerable() {
  // TODO: implement pattern match
}
