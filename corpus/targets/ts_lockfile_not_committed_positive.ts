// [frensense]
// observation: Package manager lock file (package-lock.json, yarn.lock, pnpm-lock.yaml) not committed to version control.
// impact: Without a lockfile, every `npm install` may resolve different dependency versions, even within the same semver range. This causes non-reproducible builds, 'works on my machine' bugs, and potential introduction of malicious package versions.
// improvement: Commit the lockfile to version control. Run `npm install --package-lock-only` to generate it if missing.
// cwe: CWE-1104
// cvss: 5.3
// owasp: A06:2021
// severity: Medium

// Missing files in repository:
// - package-lock.json
// - .gitignore includes package-lock.json

// .gitignore
node_modules/
package-lock.json  # VULNERABLE: lockfile excluded
