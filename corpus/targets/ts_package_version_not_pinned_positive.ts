// [frensense]
// observation: Package version specified with a range (^, ~, *) instead of an exact pinned version in production.
// impact: A minor or patch update to a dependency may introduce breaking changes, security vulnerabilities, or malicious code. Supply chain attacks often target popular packages by publishing compromised newer versions.
// improvement: Pin exact versions in production dependencies. Use package-lock.json or yarn.lock for reproducible builds together with exact version pins.
// cwe: CWE-1104
// cvss: 5.3
// owasp: A06:2021
// severity: Medium

{
  "dependencies": {
    "express": "^4.17.1",
    "lodash": "^4.17.21",
    "axios": "~0.24.0",
    "jsonwebtoken": "*"
  }
}
