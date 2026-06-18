// Vulnerable: A Spring expression is built with a dynamic value. The source of the value(s) should be verified to avoid that unfiltered values fall into this risky code evaluation.
// Pattern: <spring:eval ... expression=...>
function vulnerable() {
  // TODO: implement pattern match
}
