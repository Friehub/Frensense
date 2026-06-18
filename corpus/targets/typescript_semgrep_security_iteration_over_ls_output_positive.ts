// Vulnerable: Iterating over ls output is fragile. Use globs, e.g. 'dir/*' instead of '$(ls dir)'.
// Pattern: for $VAR in $LIST; do
  ...
done
function vulnerable() {
  // TODO: implement pattern match
}
