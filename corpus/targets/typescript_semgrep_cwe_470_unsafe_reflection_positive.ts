// Vulnerable: If an attacker can supply values that the application then uses to determine which class to instantiate or which method to invoke, the potential exists for the attacker to create control flow paths through the application that were not intended by the application developers. This attack vector may allow the attacker to bypass authentication or access control checks or otherwise cause the application to behave in an unexpected manner.
// Pattern: Class.forName($CLASS,...)
function vulnerable() {
  // TODO: implement pattern match
}
