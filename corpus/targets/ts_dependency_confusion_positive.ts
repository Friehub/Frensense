// [frensense]
// observation: Internal package name published in package.json without a registry scope (e.g., @company/pkg), making it vulnerable to dependency confusion.
// impact: An attacker can publish a public package with the same name to npm. If the registry resolution prefers public over private, the attacker's malicious code executes during installation or runtime.
// improvement: Scope all internal packages under a private npm organization scope (e.g., @mycompany/package-name).

{
  "dependencies": {
    "internal-auth-lib": "^1.0.0",
    "config-utils": "^2.3.1",
    "session-manager": "^0.5.0"
  }
}
