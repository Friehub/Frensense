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

```
cp my_bug.rs    corpus/targets/rust_my_bug_positive.rs
cp fixed.rs     corpus/targets/rust_my_bug_negative.rs
```

No YAML. No compiler changes. No DSL.

## License

MIT
