// Vulnerable: The tag "http-method" is used to specify on which HTTP methods the java web security constraint apply. The target security constraints could be bypassed if a non listed HTTP method is used. Inverse the logic by using the tag "http-method-omission" to define for which HTTP methods the security constraint do not apply. Using this way, only expected allowed HTTP methods will be skipped by the security constraint.
// Pattern: <http-method>$X</http-method>
function vulnerable() {
  // TODO: implement pattern match
}
