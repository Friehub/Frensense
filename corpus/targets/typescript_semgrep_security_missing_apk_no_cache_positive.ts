// Vulnerable: This apk command is missing '--no-cache'. This forces apk to use a package index instead of a local package cache, removing the need for '--update' and the deletion of '/var/cache/apk/*'. Add '--no-cache' to your apk command.
// Pattern: RUN apk $COMMAND ...
function vulnerable() {
  // TODO: implement pattern match
}
