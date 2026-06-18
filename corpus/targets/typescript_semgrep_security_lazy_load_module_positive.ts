// Vulnerable: Lazy loading can complicate code bundling if care is not taken, also `require`s are run synchronously by Node.js. If they are called from within a function, it may block other requests from being handled at a more critical time. The best practice is to `require` modules at the beginning of each file, before and outside of any functions.
// Pattern: require(...)
function vulnerable() {
  // TODO: implement pattern match
}
