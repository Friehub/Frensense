# The Frensense Corpus (.frc)

Starting in `v0.5.0`, Frensense completely abolished the concept of handwritten YAML rules and regex patterns. All analysis is now powered by the **Frensense Rule Corpus (.frc)**.

## What is the `.frc` File?

The `.frc` (Frensense Rule Corpus) bundle is a highly optimized, pre-compiled binary file. It contains the mathematical representations of hundreds of real-world vulnerabilities. 

Instead of an engineer writing a brittle rule like:
*If a function calls `db.delete()` and doesn't contain `if (user.isAdmin)`...*

Frensense is fed raw examples of what a vulnerable function looks like (Positive Target), and what a safe function looks like (Negative Target). 

When you run `cargo run --bin build-corpus-bundle`, Frensense parses all these snippets, extracts their **AST (Abstract Syntax Tree) n-gram fingerprints**, and serializes them using `bincode` and `blake3` into a single, lightning-fast binary blob: `frensense-corpus.frc`.

At runtime, Frensense loads this binary bundle into memory in milliseconds. It never has to re-parse the training data.

### Corpus Targets

Each vulnerability pattern in the corpus is defined by three files in the `corpus/targets/` directory before being compiled into the `.frc` bundle:
1. `pattern_name.toml` - Metadata about the finding (Severity, Impact, Tags).
2. `pattern_name_positive.ts` - An exact code snippet containing the vulnerability.
3. `pattern_name_negative.ts` - The remediated, safe version of the snippet.

When you run Frensense, the AST of your project is compared against the positive and negative fingerprints embedded in the `.frc` bundle.

## Semantic Taint Composition

If a structural match is found (meaning your code is "shaped" identically to a known vulnerability), Frensense does not immediately flag it.

First, it passes the node to Layer 2: **The DataFlowEngine**. 
The engine dynamically verifies the Taint Path. It checks if the tainted data flows from a Source (e.g. `req.body`) into a vulnerable Sink (e.g. `db.query()`), without being sanitized.

If your code *looks* like a SQL injection, but the data is safely hardcoded or sanitized, the DataFlowEngine kills the finding instantly. This multi-layered composition guarantees a significant reduction in false positives.

## Adding to the Corpus

If you want Frensense to detect a novel vulnerability specific to your architecture, you simply drop a positive and negative code snippet into the `corpus/targets/` directory.

Run the builder to compile your new custom `.frc` bundle:
```bash
cargo run --bin build-corpus-bundle
```
Frensense will automatically learn the AST patterns, pack them into the `.frc` bundle, and apply its taint composition pipeline to your custom structures!
