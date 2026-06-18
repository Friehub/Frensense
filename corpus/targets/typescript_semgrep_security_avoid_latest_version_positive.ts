// Vulnerable: Images should be tagged with an explicit version to produce deterministic container images. The 'latest' tag may change the base container without warning.
// Pattern: FROM $FROM:latest
function vulnerable() {
  // TODO: implement pattern match
}
