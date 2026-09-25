# Contributing to Frensense

Thank you for helping make Frensense better! This document explains how to contribute code and how your contribution is licensed.

## Licence

Frensense is licensed under **GPL-3.0-only** (see [LICENSE](LICENSE)). By opening a pull request you agree that your contribution is licensed to the project and its users under GPL-3.0-only.

## The Contributor Licence Agreement (CLA)

To keep the project sustainable, Friehub sells commercial licences of Frensense and paid `.frc` knowledge bundles. These sales fund development. To do this legally, we need your permission to also use your contribution in those commercial offerings.

The full agreement text lives in [CLA.md](CLA.md), it is deliberately lightweight. In summary:

1. **You own the contribution**: it is your original work, or you have the right to submit it.
2. **You grant Friehub a commercial licence**: you keep your copyright, but you grant Friehub a perpetual, worldwide, royalty-free licence to use, modify, and sublicense your contribution, including in products licensed on terms other than GPL-3.0.
3. **Users keep their GPL rights**: nothing above reduces any recipient's rights under GPL-3.0. Code merged into this repository is always available under GPL-3.0.
4. **No warranty**: your contribution is provided "as is".

You keep full ownership of your code and can use it anywhere else you like. You are **not** assigning copyright, you are granting a licence, the same model used by GitLab, Mattermost, and many others.

### How to sign

- Sign off every commit with the `-s` flag: `git commit -s -m "Your message"`. This adds a `Signed-off-by: Your Name <your@email>` line, certifying the Developer Certificate of Origin (DCO).
- On your first pull request, the **CLA bot** will ask you to sign [CLA.md](CLA.md) with one click using your GitHub account. It takes about 20 seconds and only needs doing once (again only when the CLA version changes).

## CLA bot setup (maintainers)

We use the [`contributor-assistant` GitHub Action](https://github.com/contributor-assistant/contributor-assistant), a maintained fork of the CLA Assistant lite action, running entirely inside your CI so no third-party service or account is needed:

1. Add `.github/workflows/cla.yml` invoking `contributor-assistant/contributor-assistant@v2` on `pull_request_target`, pointing `remote-organization-name` / `remote-repo-name` at `Friehub/Frensense`.
2. Set it to read the agreement text from `CLA.md` in this repo (the action posts its content as the signing comment).
3. Store signatures in the `cla-signatures` branch (the default): each contributor's GitHub login + timestamp + CLA version lands as a signed entry there, that branch *is* your signature ledger.
4. Add the action's status check (`CLAAssistant` / `cla-check`) as a **required status check** in branch protection so no unsigned PR can merge.
5. The workflow needs a repo secret `GITHUB_TOKEN` with write access to the `cla-signatures` branch (fine-grained PAT or the default token, the default is sufficient for a single-repo setup).
6. Export/backup the `cla-signatures` branch periodically; you will need those records if you ever relicense, sell, or incorporate the project.

Alternatives if you outgrow this: CLA Assistant (cla-assistant.io, free app, signs stored as a Gist) or a full CLA-management service (EasyCLA by the Linux Foundation) once Friehub is a registered legal entity, EasyCLA is the standard choice for foundations and companies, but is heavyweight for a pre-incorporation project.

## Development setup

```bash
git clone https://github.com/Friehub/Frensense.git
cd Frensense
cargo build --workspace
cargo test --workspace
```

Requirements: a recent stable Rust toolchain. The CLI entry point is `src/bin/frensense.rs`.

## Project rules

- **Engine purity**: `frensense-engine` contains analysis *mechanisms* only, no benchmark identifiers, product names, or dataset-specific knowledge. Learned knowledge lives in `SeedFacts` / `.frc` bundles produced by `frensense-bundler`.
- **Licence headers**: every source file starts with an `SPDX-License-Identifier: GPL-3.0-only` header block. Please add one to new files (copy from any existing file).
- **Tests welcome**: bug-fix PRs should include a regression test. The corpus of security test-cases is maintained privately; your tests live in the normal `tests` modules.
- Run `cargo fmt` and `cargo clippy` before submitting.

## Submitting

1. Fork, create a feature branch from `fix/release-v4` (or the current default branch).
2. Commit with `-s` (DCO sign-off).
3. Open a PR describing the *why*, not just the *what*.
4. CI must pass and the CLA check must be green.

## Questions

Open an issue or reach the maintainers at the address listed on https://friehub.com/licensing.
