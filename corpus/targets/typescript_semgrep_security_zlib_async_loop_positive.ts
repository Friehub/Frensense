// Vulnerable: Creating and using a large number of zlib objects simultaneously can cause significant memory fragmentation. It is strongly recommended that the results of compression operations be cached or made synchronous to avoid duplication of effort.
// Pattern: zlib.$METHOD(...);
function vulnerable() {
  // TODO: implement pattern match
}
