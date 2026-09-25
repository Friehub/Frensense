<div align="left">
  <h1>Frensense</h1>
  <p><strong>A deterministic, dataflow-driven security scanner for TypeScript, JavaScript, Python, Go, and Rust.</strong></p>
</div>

<br />

Frensense detects security vulnerabilities by combining compiler-grade dataflow
analysis with a learned knowledge base. No brittle YAML rule packs, no regex
heuristics, the engine builds an AST, def-use chains, and a program dependence
graph for your code, then verifies whether untrusted data can actually reach a
dangerous sink.

## How It Works

1. **IR & Graphs**: Every file is lowered to a lightweight IR; the engine builds
   def-use chains, a call graph, heap/points-to summaries, and an SVFG.
2. **Taint Analysis**: Taint paths are tracked from sources (`req.body`,
   query params) to sinks (`exec`, `db.query`, `fetch`) with path-sensitive
   pruning through the taint engine.
3. **Knowledge Merge**: A `.frc` bundle's learned facts (framework-aware
   sources, sinks, sanitizers, and modelled APIs) are merged into the config,
   teaching the engine framework-specific semantics.
4. **Composition**: A finding is emitted only when the dataflow result and the
   structural evidence agree, keeping the false-positive rate low.

## Quick Start

```bash
# Install
cargo install frensense

# Scan a project
frensense .

# Teach the engine with a .frc knowledge bundle
frensense . --corpus-bundle frensense-corpus.frc

# JSON / SARIF output
frensense . --json
frensense . --sarif

# Diff-only (changed files since last commit)
frensense . --diff-only --strict

# Baseline workflow: save current findings, then fail on new ones
frensense . --emit-baseline baseline.json
frensense . --compare-baseline baseline.json --strict
```

The engine auto-discovers a `frensense-corpus.frc` in the project root, or you
can point `--corpus-bundle` at any bundle file.

## What It Catches

- **Injection**: SQL/NoSQL injection, command injection, XPath/XXE.
- **Broken Access Control**: IDOR, missing auth guards, mass assignment.
- **Cryptographic Weaknesses**: weak hashes, ECB mode, hardcoded keys.
- **Server-Side Issues**: SSRF, path traversal, XXE, unsafe deserialization.
- **Client-Side**: Reflected/DOM XSS, prototype pollution, open redirects.
- **Misconfiguration**: Missing security headers, session mismanagement,
  insecure cookies, TLS verification disabled.

## Languages

| Language | Parser | Compiler Mode |
|----------|--------|---------------|
| TypeScript / JavaScript / TSX | Tree-sitter | Oxc (`--use-compiler`) |
| Python | Tree-sitter |, |
| Go | Tree-sitter |, |
| Rust | Tree-sitter | rust-analyzer HIR (`--use-compiler`) |

## MCP Integration (AI Agents)

Frensense ships an MCP server so AI coding agents can scan their own output
before committing:

```bash
frensense-mcp
```

Agents can request scans, taint-path resolutions, and validate generated code
against the engine's dataflow analysis.

## Knowledge Bundles (.frc)

Detection knowledge ships as `.frc` bundles, compiled, checksummed archives
produced by `frensense-bundler`. A bundle carries two kinds of content:

- **Patterns**: fingerprinted positive/negative code pairs the engine matches
  against your functions.
- **Learned facts**: replay-verified sources, sinks, sanitizers, and modelled
  framework APIs.

At scan time the engine loads a bundle in this priority order:

1. `--corpus-bundle <file>` CLI flag
2. Auto-discovered `frensense-corpus.frc` in the project root
3. Embedded bytes via the library API (`set_corpus_bundle`)

The envelope is validated (`FRC1` magic, version, checksum) and the learned
facts are merged into the engine's fact table on top of the built-in language
specs, the bundle teaches the engine at runtime, without touching engine code.

## Contributing

Contributions are welcome under GPL-3.0. Read [CONTRIBUTING.md](CONTRIBUTING.md)
and sign the lightweight [CLA](CLA.md), one click via the CLA bot on your first
pull request.

## License

GPL-3.0, see [LICENSE](LICENSE). Commercial licensing available at
https://friehub.com/licensing.
