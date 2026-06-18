// Vulnerable: The top level wildcard bindings $PREFIX leaves your application open to security vulnerabilities and give attackers more control over where traffic is routed. If you must use wildcards, consider using subdomain wildcard binding. For example, you can use "*.asdf.gov" if you own all of "asdf.gov".
// Pattern: $LISTENER.Prefixes.Add("$PREFIX")
function vulnerable() {
  // TODO: implement pattern match
}
