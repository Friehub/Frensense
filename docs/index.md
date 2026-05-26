---
layout: home

hero:
  name: "GenSense"
  text: "Semantic Analysis Engine"
  tagline: "Detect logical flaws, security risks, and AI-generated code patterns that conventional linters miss."
  actions:
    - theme: brand
      text: "Get Started"
      link: "/guide"
    - theme: alt
      text: "Rule Catalog"
      link: "/rules"
    - theme: alt
      text: "Write Custom Rules"
      link: "/extending"

features:
  - title: Semantic Program Graph
    details: SPG exposes a cross-file symbol graph with call edges, temporal event chains, and taint flow edges to every rule — enabling algebraic query composition without Datalog.
  - title: CSA Body Analysis
    details: Contextual Structural Analysis checks function bodies for required patterns (body_must_contain) and delegation suppression (body_may_delegate_via) — catches validation gaps, missing auth, and sanitizer passthrough.
  - title: Extensible Rule Engine
    details: Ship custom YAML rules without recompiling. Drop a .yml file in .gensense/rules/ and the engine picks it up automatically at startup.
  - title: Temporal & Taint Analysis
    details: Detects event-ordering violations (mutex locks held across await, connection leaks) and tracks data flow from sensitive sources to unsafe sinks across function boundaries.
  - title: Native Performance
    details: Built in Rust with optimized single-pass rule execution. Scans a 50-file project in under 5 seconds on standard hardware.
  - title: CI/CD Ready
    details: JSON, SARIF, strict-mode, diff-only, baseline comparison, and suite selection. Integrates with GitHub Actions, pre-commit hooks, and VS Code.
---

## Who Uses GenSense

### Organizations with AI-Assisted Development
Developers use GitHub Copilot, Claude, ChatGPT for code generation. GenSense catches AI-generated placeholder code, dead results, and tautological logic before they reach production.

### Critical Infrastructure Teams
Banking, healthcare, IoT systems cannot tolerate deadlocks and secret leaks. GenSense finds concurrency hazards and data leaks that traditional linters miss.

### Platform & DevOps Teams
Enforce architectural patterns across teams without source recompilation. Custom rules as YAML files, hot-loaded at startup.

### Security Auditors
Scan for hardcoded secrets and unsafe patterns. Generate SARIF compliance reports with GitHub integration.

---

## Quick Evaluation

Does this describe your scenario?

> "Our team uses Copilot for code generation, and we're worried about production issues."

✅ **GenSense is built for this**

> "We have strict typing but still get runtime deadlocks in async code."

✅ **GenSense will help**

> "We want to enforce that all database queries use prepared statements."

✅ **Write a custom YAML rule**

> "We have only synchronous Python code with no security risks."

❌ **GenSense won't add value** (also doesn't support Python yet)
