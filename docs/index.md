---
layout: home

hero:
  name: "Frensense"
  text: "Compositional Taint Analysis Engine"
  tagline: "Detect complex structural vulnerabilities and logic flaws without writing regex or YAML rules."
  actions:
    - theme: brand
      text: "Read the Guide"
      link: "/guide"
    - theme: alt
      text: "The Corpus (.frc)"
      link: "/corpus"
    - theme: alt
      text: "MCP Integration"
      link: "/mcp"

features:
  - title: 100% Corpus-Driven
    details: Frensense does not use hardcoded rules. It learns the shape of vulnerabilities from hundreds of positive and negative AST pairs compiled into a single .frc binary bundle.
  - title: Multi-Layered Composition
    details: Layer 1 runs a fast structural pass using multi-scale AST n-grams. Layer 2 executes a semantic verification pass, evaluating data-flow taint, entropy, and cross-file invariants.
  - title: Reduced False Positives
    details: If a structural match lacks a provable taint path from an untrusted Source to a vulnerable Sink, the finding is dynamically culled. It significantly reduces false positives, though edge cases remain in highly dynamic codebases.
  - title: Native Performance
    details: Built in Rust. Single-pass AST parsing with tree-sitter allows for rapid scanning of codebases.
  - title: Continuous Harvesting
    details: The corpus is updated by harvesting real-world CVEs and PR fixes from open-source repositories using NVD streaming harvesters.
  - title: AI Agent Native
    details: Exposes the full engine to AI agents (like Claude or GPT-4) via the Model Context Protocol (MCP) frensense_audit tool.
---

## Why Frensense?

### The Problem with Linters
Traditional linters rely on regex or handwritten AST rules (YAML/DSL). They are tedious to write, brittle to code formatting, and struggle to reason about multi-file data flow. This often leads to CI pipelines flooded with false positives.

### The Problem with SAST
Standard Static Application Security Testing (SAST) tools trace data flow blindly. They can be computationally expensive, taking minutes or hours to run, and they often lack context about what makes a specific structural pattern vulnerable.

### The Frensense Approach
Frensense fuses the two methodologies. It uses AST Fingerprinting to spot code that looks like a vulnerability (Layer 1), and then uses Deterministic Taint Verification to prove the vulnerability exists (Layer 2).

---

## Quick Evaluation

Does this describe your scenario?

> "Our team uses Copilot for code generation, and we're worried about subtle logic bugs making it to production."

✅ **Frensense is built for this**

> "We want to catch Unauthenticated DB Writes across our microservices."

✅ **Frensense catches this natively without configuration**

> "We want a tool to format our code and check for missing semicolons."

❌ **Use Prettier or ESLint instead**
