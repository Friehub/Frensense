# GenSense CI Hardening & Security Recommendations
**Branch:** `feature/multi-file-rules` | **Date:** 2026-05-12

---

## 1. Prevent Broken Branches from Merging

### 1.1 Branch Protection Rules (GitHub Settings → Branches)

The CI YAML alone cannot block a merge — enforcement requires branch protection rules on `main`:

- **Require pull request before merging** — disable direct pushes to `main`
- **Require approvals: 1** — paired with a `CODEOWNERS` file (see below)
- **Require status checks to pass** — list all 6 jobs explicitly:
  - `Quality Assurance (Rust)` (fmt + clippy)
  - `Rust Tests`
  - `Node.js Binding Verification`
  - `Security Audit`
  - `cargo deny` *(new — see §2.2)*
  - `npm audit` *(new — see §2.3)*
- **Require branches to be up to date before merging**
- **Do not allow force pushes**
- **Do not allow deletions**
- **Require merge queue** — set merge method to squash-only

### 1.2 Add a CODEOWNERS File

Create `.github/CODEOWNERS`:

```
# Global ownership — all PRs require review from at least one of these
* @friehub/core-reviewers

# Extra scrutiny for security-sensitive paths
src/engine/suppression.rs   @friehub/security
src/rules/ir.rs             @friehub/security
.github/workflows/          @friehub/security
```

### 1.3 Fix Test Coverage Gap

The current `cargo test --features cli` misses `solidity`, `full`, and `node` feature
combinations. Add a matrix job:

```yaml
strategy:
  matrix:
    features: [cli, "full", "rust,typescript,solidity"]
steps:
  - run: cargo test --features ${{ matrix.features }}
```

### 1.4 Add MSRV Check

Pin `rust-version` in `Cargo.toml`:

```toml
[package]
rust-version = "1.75"
```

Add a CI job:

```yaml
msrv:
  name: MSRV Check
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@1.75
    - run: cargo check --all-features
```

---

## 2. Supply Chain Attack Prevention

This is the highest-impact risk area. The 2025 `tj-actions/changed-files` and
`reviewdog` attacks both exploited floating action tags.

### 2.1 Pin All Action References to Full SHAs

Replace every `uses: owner/action@vX` with a pinned SHA. Examples:

```yaml
# BEFORE (vulnerable)
- uses: actions/checkout@v4
- uses: dtolnay/rust-toolchain@stable
- uses: peaceiris/actions-gh-pages@v3

# AFTER (safe)
- uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683      # v4.2.2
- uses: dtolnay/rust-toolchain@4305bc73974b4c2e4df9e72db5c5bb65f09a27a  # stable 2025-04
- uses: peaceiris/actions-gh-pages@4f9cc6602d3f66b9c108549d231e62369c1bc84  # v3.10.0
```

Use `pin-github-action` or Dependabot's `update-actions` mode to automate this.
Add Dependabot config to keep pinned SHAs updated automatically:

```yaml
# .github/dependabot.yml
version: 2
updates:
  - package-ecosystem: github-actions
    directory: /
    schedule:
      interval: weekly
  - package-ecosystem: cargo
    directory: /
    schedule:
      interval: weekly
  - package-ecosystem: npm
    directory: /
    schedule:
      interval: weekly
```

### 2.2 Replace `cargo audit` with `cargo deny`

`cargo deny` is a strict superset of `cargo audit`. Add `deny.toml`:

```toml
[advisories]
vulnerability = "deny"
unmaintained = "warn"
yanked = "deny"

[licenses]
unlicensed = "deny"
allow = ["MIT", "Apache-2.0", "Apache-2.0 WITH LLVM-exception", "BSD-3-Clause", "ISC", "Unicode-DFS-2016"]

[bans]
multiple-versions = "warn"
deny = [
  { name = "openssl", reason = "use rustls instead" },
]
```

Replace the security job:

```yaml
security:
  name: Supply Chain Audit
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@<SHA>
    - uses: EmbarkStudios/cargo-deny-action@<SHA>
      with:
        command: check advisories licenses bans
```

### 2.3 Add `npm audit` to PR Checks

```yaml
npm-security:
  name: NPM Security Audit
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@<SHA>
    - uses: actions/setup-node@<SHA>
      with:
        node-version: '20'
    - run: npm ci
    - run: npm audit --omit=dev --audit-level=high
```

### 2.4 Lock Down Workflow Permissions

Add `permissions` blocks to every job. The default implicit `GITHUB_TOKEN` has
write scope on contents, pull requests, and packages:

```yaml
# Top-level default — most restrictive
permissions:
  contents: read

jobs:
  test-rust:
    permissions:
      contents: read   # only what's needed

  github-release:
    permissions:
      contents: write  # explicitly opt into write only where required
      id-token: write  # for OIDC
```

### 2.5 Replace Long-Lived Secrets with OIDC

Your `release.yml` stores `NPM_TOKEN` and `CARGO_REGISTRY_TOKEN` as long-lived
repository secrets. Replace with short-lived OIDC tokens:

```yaml
publish-npm:
  permissions:
    id-token: write
    contents: read
  steps:
    - uses: actions/setup-node@<SHA>
      with:
        registry-url: 'https://registry.npmjs.org'
    - run: npm publish --provenance --access public
      env:
        NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}  # can be scoped to automation role
```

For crates.io, enable the "trusted publishing" beta which uses OIDC and needs no
stored secret at all.

### 2.6 Add SLSA Provenance to Release Artifacts

Gives users a cryptographic guarantee that a binary came from your workflow and
wasn't tampered with:

```yaml
build-native:
  # ... existing build steps ...
  - uses: slsa-framework/slsa-github-generator/.github/workflows/generator_generic_slsa3.yml@<SHA>
    with:
      base64-subjects: ${{ steps.hash.outputs.hashes }}
      upload-assets: true
```

---

## 3. Static Analysis (Catching What Tests Miss)

### 3.1 Add Semgrep SAST

```yaml
semgrep:
  name: SAST
  runs-on: ubuntu-latest
  container:
    image: semgrep/semgrep
  steps:
    - uses: actions/checkout@<SHA>
    - run: semgrep scan --config=p/rust --config=p/secrets --error
```

The `p/secrets` ruleset catches accidental credential patterns in source files.
The `p/rust` ruleset catches unsafe patterns beyond what Clippy covers. Free for
open source.

### 3.2 Harden the `sync.yml` Workflow

`sync.yml` does a `git push` from CI using `GITHUB_TOKEN`. This can modify
`main` outside the normal PR/review path. Replace with a pre-commit hook or make
version syncing a required manual step in the release checklist.

---

## 4. Release Workflow Hardening

### 4.1 Protect the Release Environment

`workflow_dispatch: {}` with no restrictions means any maintainer can ship a
release pointing at any commit. Add a GitHub Environment:

```yaml
publish-cargo:
  environment: production   # requires manual approval from security team
  ...
publish-npm:
  environment: production
  ...
```

Configure the `production` environment in Settings → Environments to:
- Require review from a designated team before deployment
- Restrict to the `main` branch only
- Set a 5-minute wait timer (catches accidental triggers)

### 4.2 Add StepSecurity Harden Runner (Optional but Recommended)

Audits outbound network calls from each job and can block unexpected egress:

```yaml
steps:
  - uses: step-security/harden-runner@<SHA>
    with:
      egress-policy: audit   # start in audit mode, switch to block after baselining
  - uses: actions/checkout@<SHA>
```

---

## 5. Priority Order

| Priority | Action | Effort |
|----------|--------|--------|
| 🔴 Critical | Enable branch protection + require status checks | 10 min (GitHub UI) |
| 🔴 Critical | Pin all action SHAs | 30 min (one PR) |
| 🔴 Critical | Add `permissions: contents: read` to all jobs | 15 min |
| 🟠 High | Add `CODEOWNERS` file | 5 min |
| 🟠 High | Replace `cargo audit` with `cargo deny` | 1 hour |
| 🟠 High | Add `npm audit` job | 15 min |
| 🟠 High | Add Dependabot config | 10 min |
| 🟡 Medium | Add Semgrep SAST | 30 min |
| 🟡 Medium | Add MSRV check + pin `rust-version` | 30 min |
| 🟡 Medium | Add release environment with manual approval | 20 min (GitHub UI) |
| 🟢 Low | OIDC for NPM/crates.io | 1–2 hours |
| 🟢 Low | SLSA provenance attestation | 2 hours |
| 🟢 Low | StepSecurity Harden Runner | 1 hour |
