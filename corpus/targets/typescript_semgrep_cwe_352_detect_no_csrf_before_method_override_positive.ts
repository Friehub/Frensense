// Vulnerable: Detected use of express.csrf() middleware before express.methodOverride(). This can allow GET requests (which are not checked by csrf) to turn into POST requests later.
// Pattern: express.csrf();
...
express.methodOverride();
function vulnerable() {
  // TODO: implement pattern match
}
