// Vulnerable: The default core Clojure read-string method is dangerous and can lead to deserialization vulnerabilities. Use the edn/read-string instead.
// Pattern: (read-string $X)
function vulnerable() {
  // TODO: implement pattern match
}
