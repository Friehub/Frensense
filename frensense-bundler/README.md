# frensense-bundler: Corpus Compiler

`frensense-bundler` is responsible for compiling raw vulnerability examples, security fixes, and configuration definitions into a highly optimized, binary **Frensense Rule Corpus (`.frc`)**.

## Purpose

Unlike traditional SAST tools that rely on hand-written YAML or JSON rules, Frensense "learns" from real code snippets (the corpus). `frensense-bundler` acts as the compiler that prepares this corpus for the engine.

## What it does:

1. **AST Parsing & Fingerprinting:** Ingests raw `.js`, `.ts`, `.rs`, `.go`, etc., snippets from the corpus directories and runs them through the Frensense fingerprinting pipeline.
2. **Metadata Extraction:** Parses accompanying `.yml` or `.json` metadata files that define the vulnerability type, CWE, OWASP category, and required taint paths.
3. **Serialization & Compression:** Bundles the pre-computed fingerprints, structural hashes, and semantic markers into a compressed `.frc` binary archive.

## Usage in CI/CD
By shipping a pre-compiled `.frc` bundle, the `frensense` CLI can achieve sub-second startup times in CI environments without needing to re-parse and hash the raw training data on every scan.
