// Vulnerable: Named Credentials (and callout endpoints) should be used instead of hard-coding credentials. 1. Hard-coded credentials are hard to maintain when mixed in with application code. 2. It is particularly hard to update hard-coded credentials when they are used amongst different classes. 3. Granting a developer access to the codebase means granting knowledge of credentials, and thus keeping a two-level access is not possible. 4. Using different credentials for different environments is troublesome and error-prone.
// Pattern: $REQUEST.setHeader('Authorization', $AUTHSTRING);
function vulnerable() {
  // TODO: implement pattern match
}
