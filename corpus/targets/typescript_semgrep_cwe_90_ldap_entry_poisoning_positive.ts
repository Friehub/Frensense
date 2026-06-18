// Vulnerable: An object-returning LDAP search will allow attackers to control the LDAP response. This could lead to Remote Code Execution.
// Pattern: {'pattern': 'new SearchControls($S, $CL, $TL, $AT, true, $DEREF)\n'} | {'pattern': 'SearchControls $VAR = new SearchControls();\n...\n$VAR.setReturningObjFlag(true);\n'}
function vulnerable() {
  // TODO: implement pattern match
}
