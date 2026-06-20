# Frensense

Example-driven code analysis. Detects semantic bugs — code that compiles but doesn't do what it says it does — without YAML rules, regex patterns, or handwritten DSL.

```bash
cargo install frensense
frensense . --corpus corpus/targets/
```

[Full documentation →](FRENSENSE.md)

## How It Works

Detection is driven by example pairs in `corpus/targets/`. A pattern is two files — one showing the bug, one showing correct code. The engine fingerprints every function in your project, scores it against all patterns, and emits findings when multiple layers confirm:

1. **Corpus match** — function shape matches a known violation pattern
2. **Taint path** — tainted data actually flows from source to sink
3. **Taint entropy** — validator-named functions actually branch on their inputs
4. **Cross-function consistency** — no sibling function diverges on the same pattern

A finding only fires when all layers agree.

## What It Catches

- Credentials flowing to database writes, logs, or HTTP
- Untrusted input reaching shell exec or filesystem operations
- Mutex held across `.await` points (deadlock)
- `validate_*()` functions with no rejection path
- Hollow validators that pass input through unchanged
- LLM-hallucinated imports not in the lockfile
- Near-duplicate functions with divergent security behavior
- Hardcoded AWS keys, JWT, private keys in source

## Quick Start

```bash
# Basic scan
frensense .

# With corpus pattern detection
frensense . --corpus corpus/targets/ --threshold 0.65

# Only critical findings
frensense . --severity critical --strict

# JSON output
frensense . --json

# SARIF for GitHub Advanced Security
frensense . --sarif

# Diff-only (changed files since last commit)
frensense . --diff-only --strict

# Baseline suppression
frensense . --baseline baseline.json

# List loaded patterns
frensense --list-patterns --corpus corpus/targets/
```

## Languages

Rust, TypeScript, JavaScript, Python (opt-in), C (opt-in). Cross-language pattern matching via abstract kind taxonomy.

## Adding a Detection

Copy two files into `corpus/targets/`:

```bash
cp my_bug.rs    corpus/targets/rust_my_bug_positive.rs
cp fixed.rs     corpus/targets/rust_my_bug_negative.rs
```

No YAML. No compiler changes. No DSL.

### Other Detection Types

Frensense has 6 detection systems. See the [Developer Guide](docs/DEVELOPER_GUIDE.md) for:
- **Taint rules** — source-to-sink data flow (`taint_rules.toml`)
- **Temporal rules** — API ordering constraints (`temporal_rules.toml`)
- **Corpus patterns** — function shape matching (`corpus/targets/`)
- **Finding modules** — custom AST analysis (`src/engine/findings/`)
- **Composition** — cross-layer confidence adjustment
- **Secret scanning** — hardcoded credential detection

## Corpus Data Sources

The detection corpus is built from real-world vulnerability data:

- **CVEfixes** — 12,107 vulnerability-fixing commits across 4,249 open-source projects (11,873 CVEs, 272 CWE types). Before/after patches extracted as positive/negative function pairs.
- **Semgrep community rules** — 3,000+ rules with `_bad`/`_ok` test fixtures converted to corpus pairs.

### Citation

If you use CVEfixes data in your research or tooling, please cite:

> Moonen, L., Vidziunas, L., & Bhandari, G. P. (2024). *CVEfixes: Automated Collection of Vulnerabilities and Their Fixes from Open-Source Software* (v1.0.8). 17th International Conference on Predictive Models and Data Analytics in Software Engineering (PROMISE), Athens, Greece. Zenodo. https://doi.org/10.5281/zenodo.13138703

If you use the Semgrep community rules as a corpus source, please cite:

> Semgrep, Inc. (2024). *Semgrep Rules Repository*. GitHub. https://github.com/semgrep/semgrep-rules

## License

MIT
