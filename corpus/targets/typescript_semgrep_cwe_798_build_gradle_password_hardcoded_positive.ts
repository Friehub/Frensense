// Vulnerable: A secret is hard-coded in the application. Secrets stored in source code, such as credentials, identifiers, and other types of sensitive data, can be leaked and used by internal or external malicious actors. It is recommended to rotate the secret and retrieve them from a secure secret vault or Hardware Security Module (HSM), alternatively environment variables can be used if allowed by your company policy.
// Pattern: password = env[...] ?: "$SECRET"
function vulnerable() {
  // TODO: implement pattern match
}
