# Frensense: Next Direction Research

> Author: research session, 2026-07-10
> Baseline: HEAD = 62bed13 (2026-07-08), branch v0.4.0-tasks
> Stale documents: BENCHMARK_REPORT.md (2026-06-18), BENCHMARK.md (2026-06-18),
>   tasks.md corpus phase statuses (written before v0.5.0 corpus restructuring)

---

## Verified current state (as of 2026-07-10)

All numbers verified from git, live binary, and filesystem — not from stale reports.

| Fact | Value | Source |
|------|-------|--------|
| Corpus source files (corpus/targets/) | 93 total: 82 .rs/.ts + 9 .toml sidecars | `ls` |
| Unique pattern names | 51 | deduped source file stems |
| Patterns loaded by binary (from FRC + source) | 42 from source dir | `--list-patterns` |
| FRC bundle size | 499 KB | `ls -lh frensense-corpus.frc` |
| Test suite | 114 passed, 0 failed, 15 ignored | `cargo test` live |
| Last code commit | 2026-07-08 (e2e fixes, clippy clean) | git log |
| v0.5.0 tagged | 2026-07-07 | git log |

**CHANGELOG.md v0.5.0 entry claims "603 positive corpus patterns, 1214 fingerprints, 3.0 MB FRC
bundle."** The current filesystem does not match this. The current FRC is 499 KB and 42 patterns
load from source. The CHANGELOG number appears to have been written reflecting a state that was
later pruned, or it counted fingerprints differently. Treat all corpus count claims in the
CHANGELOG as documentation debt, not current fact.

**The OWASP Juice Shop result is real and documented.** The v0.5.0 CHANGELOG records:
"Scanner generalizes to find 77 true-positive vulnerabilities in the unseen OWASP Juice Shop
repository without explicit fine-tuning." This is the only known external ground truth result.
It was not formally benchmarked (no per-pattern breakdown, no precision/recall table, no FP count).
It is evidence that the engine generalizes. It is not a precision/recall baseline.

---

## What the Juice Shop result actually tells us

The 77 TP number is meaningful but incomplete as a quality signal:

- **It proves generalization works.** The engine found real bugs in a codebase it was not trained
  on. That is the core claim of the corpus approach validated.
- **It does not tell us FP rate.** 77 TPs means nothing without knowing how many findings were
  FPs. If the engine produced 500 findings of which 77 were TP, precision is 15%. If it produced
  80 findings, precision is 96%. These lead to very different conclusions.
- **It does not attribute findings to patterns.** Which of the 42 current patterns fired? Which
  fired repeatedly? Which never fired? Without per-pattern breakdown, the 77 number cannot guide
  which patterns to improve, which to drop, or which new patterns would add coverage.
- **It is from the v0.5.0 era.** The current engine post-July 8 (taint seeder, CSA rework,
  clippy/e2e fixes) is a different state. The 77 number may be higher or lower now.

The right way to use Juice Shop is not to re-run and report a single headline number. It is to
use it as a reference codebase in a structured measurement pass that produces per-pattern signal.

---

## Revised Phase 1: use Juice Shop as the primary reference, not CVEfixes

The previous draft proposed picking 3 CVEfixes repositories. That has a real problem: you need
to know which functions in those repositories are actually buggy, which requires manual labeling
or relying on commit-level diffs as a proxy. Commit-level diffs are noisy — they include refactors,
renames, and comment changes alongside actual vulnerability fixes.

Juice Shop is better as the primary reference because:

1. **The CVEs are published and mapped to files.** OWASP maintains a list of intentional
   vulnerabilities by category. These map to specific files and functions in the codebase.
   This is pre-labeled ground truth without needing to parse git diffs.
2. **The engine already ran on it.** The 77 TP result means the current patterns do fire.
   Re-running with structured output gives per-pattern attribution for those 77 hits.
3. **It is TypeScript.** The majority of current corpus patterns are TypeScript (ts_*). The
   coverage match is better than Rust-heavy CVEfixes repositories.
4. **Clean baseline is well-defined.** A "clean" baseline is any function not in the known
   vulnerability list. This is more reliable than a post-fix commit diff.

### Revised Phase 1 plan (unchanged tools, different reference)

1. Clone OWASP Juice Shop at a pinned commit (consistent reference).
2. Obtain the published vulnerability map (OWASP provides this as part of the project):
   which files/routes contain known intentional vulnerabilities, by CWE category.
3. Run `frensense --json` on the full repository.
4. For each finding in the JSON output:
   - If the finding's file/line is in the vulnerability map: TP.
   - If not: FP.
5. Aggregate per-pattern: recall (TPs caught / total known vulns in that CWE), precision
   (TPs / total findings from that pattern).
6. Output ranked table: patterns by F1 = 2 * precision * recall / (precision + recall).

This gives the per-pattern signal profile without needing to manually label anything.

### Adding a clean reference

Juice Shop alone only measures recall + precision on buggy code. To measure FP rate on clean
code, add one known-clean TypeScript repository of similar size (a well-maintained framework
or library with no known CVEs). The combination gives:

- Juice Shop (buggy reference): recall + precision per pattern
- Clean TS repo: FP rate per pattern on genuinely clean code

Together these are the two halves of the signal profile.

---

## The CSG measurement framework (unchanged)

For each corpus pair, the goal is a four-number signal profile:

```
signal_profile(pattern_id) = (
    recall,       # TPs caught / total known vulns in that CWE (from Juice Shop map)
    precision,    # TPs / total findings from that pattern (from Juice Shop scan)
    fp_rate,      # findings on clean repo / total functions scanned (from clean repo)
    coverage_rank # unique TPs not covered by any other pattern in the same CWE class
)
```

The small engine change needed for Phase 2: surface `sim_to_positive` and `sim_to_negative`
in the JSON output per finding (behind `--verbose-scores`). The scorer computes both already;
they are just not in the output. This is a one-field addition to the JSON serializer in runner.rs.
With these two numbers in the output, `delta_pos_neg` per function can also be computed across
the full scan, not just on findings — patterns with near-zero delta across the whole codebase
are non-discriminating regardless of threshold.

---

## What is actually stale in tasks.md

The following tasks.md entries are status conflicts with what exists on disk post-v0.5.0:

**CHANGELOG.md v0.5.0 says these were removed:**
- `dead_branch.rs`, `unused_variable.rs`, `atomic_section.rs`, `secrets.rs` — all removed.
  tasks.md still has W2/W3/W9 marked Done for wiring these. Those modules no longer exist.
- `check_then_act.rs` — removed. tasks.md CSA items that reference this are resolved differently.
- `temporal_rules.toml` active rules — removed. `temporal_rules.toml` exists but is empty.
  tasks.md W1 says temporal detection is Done via this TOML, but the TOML is empty; detection
  is now corpus-driven via temporal corpus pairs.

**tasks.md corpus phase counts do not match current corpus:**
- tasks.md records SP1 (Command Injection, 10 pairs), SP2 (SQL Injection, 10 pairs),
  SP3 (Path Traversal, 10 pairs) as Done with specific file names like
  `ts_sec_cmd_injection_{1-10}_{positive,negative}.ts`. None of those files exist in
  `corpus/targets/`. The current corpus has single pairs (`ts_command_injection_positive.ts`),
  not the 10-variation sets tasks.md describes as Done.
  **This is the largest stale discrepancy.** SP1/SP2/SP3 are not Done; they are at 1-pair
  coverage, not 10-pair coverage.

**tasks.md Phase 5 taint precision claims v. current state:**
- Phase 5 says "Scanned Axum json.rs, form.rs, lib.rs. Taint findings: 0 (was 585)." This
  result is from a specific state that was then rebuilt. Whether the current build reproduces
  this is unverified. The taint seeder work is committed; whether axum still produces 0 taint
  findings needs a re-run to confirm.

**tasks.md B10/F7/VP2 remain Open with no date or blocker update:**
- B10: Benchmark on real open-source projects — Open, no progress since v0.5.0.
- F7: Validate on real-world LLM-generated code — Open.
- VP2: Full corpus validation — Open.
All three are now superseded by the CSG plan (Juice Shop + clean ref as structured B10/VP2).

---

## What not to do next

- **Do not treat the 77 Juice Shop TPs as a precision/recall claim.** Without FP count and
  per-pattern breakdown it is a demo result, not a benchmark. Re-run with structured output.

- **Do not continue authoring SP4-SP8, HP1-HP4, LP1-LP4 before Phase 1 CSG runs.** Tasks.md
  shows SP1/SP2/SP3 as Done at "10 pairs each" but the corpus has 1 pair each. The right move
  is to clarify actual coverage via measurement before expanding further.

- **Do not trust the CHANGELOG corpus counts.** 603 patterns vs. 42 loaded is a significant
  discrepancy that needs to be understood before any corpus count is published externally.

---

## Summary: the move

**Phase 1 of CSG using Juice Shop as the primary reference, not CVEfixes.**

1. Clone Juice Shop at pinned commit.
2. Get the OWASP vulnerability map (known intentional CVEs by file/route).
3. Run `frensense --json` on the full repository.
4. For each JSON finding, classify TP/FP against the vulnerability map.
5. Aggregate: recall + precision per pattern. Rank by F1.
6. Optionally run on one clean TS repository for FP rate on genuinely clean code.

This takes the 77 TP headline number and turns it into a per-pattern signal profile. It tells
you which 42 patterns are doing the work, which need more pairs, and what coverage the engine
actually has on a well-characterized vulnerable codebase.

Every downstream decision — SP4-SP8 authoring priority, M3 contextual featurization, M5
calibration, harvest pipeline scope — follows from what Phase 1 shows.

---

## Open questions before starting

1. **Pinned Juice Shop commit.** Which commit was used for the original 77 TP scan? Re-running
   at the same commit gives a diff that shows what the July 8 engine improvements changed.
   If unknown, use the current main as the reference commit going forward.

2. **Vulnerability map format.** OWASP Juice Shop documents its intentional vulnerabilities.
   Does the map go to file+function granularity, or only to route/category? File+function is
   needed for per-function TP/FP classification. Route/category requires a manual mapping step.

3. **sim_to_positive/sim_to_negative in JSON output.** Behind `--verbose-scores` flag only,
   or also in default output? Scope of the runner.rs change.
