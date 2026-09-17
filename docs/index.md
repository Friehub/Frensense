---
layout: home

hero:
  name: "Frensense"
  text: "Deterministic Security Engine."
  tagline: "Static analysis without the rules. Frensense detects vulnerabilities using concrete code pairs rather than YAML or DSLs—encoding structure, control flow, and data flow simultaneously for exact similarity search."
  actions:
    - theme: brand
      text: Read the Paper
      link: /frensense-paper/00_OVERVIEW
    - theme: alt
      text: Read the Blog
      link: /blog/

features:
  - title: Example-Driven, Not Rule-Driven
    details: Skip writing complex Semgrep rules or regex. Provide a vulnerable function and its fix, and the engine automatically fingerprints the difference.
  - title: Multi-Dimensional Similarity
    details: Fingerprints corpus pairs at build time for fast, exact similarity search at runtime across Rust, TypeScript, and JavaScript.
  - title: Precise Data-Flow
    details: Uses field-sensitive Program Dependence Graphs (PDG) for exact control flow reachability tracking to drastically reduce false positives.
---

<div class="showcase-container">
  <h2 class="showcase-title">How Frensense Works</h2>
  <p class="showcase-subtitle">Stop writing abstract syntax trees. Teach the engine by providing a simple before-and-after example of a vulnerability.</p>

  <div class="step">
    <div class="step-label">1. Define the Corpus Pair</div>
    <p>Provide a vulnerable (positive) snippet and its safe (negative) counterpart. The engine learns the structural difference.</p>

::: code-group

```typescript [ts_cmdi_positive.ts (Vulnerable)]
// [frensense]
// observation: User input passed to exec() via shell interpolation.
// improvement: Use execFile() with an arguments array.
import { exec } from "child_process";

export function ping(req, res) {
    const ip = req.body.target;
    exec(`ping -c 4 ${ip}`, (err, stdout) => {
        res.send(stdout);
    });
}
```

```typescript [ts_cmdi_negative.ts (Fixed)]
// SAFE: Replaced exec with execFile to prevent shell injection.
import { execFile } from "child_process";

export function ping(req, res) {
    const ip = req.body.target;
    // The engine sees this structure and marks it as safe.
    execFile("ping", ["-c", "4", ip], (err, stdout) => {
        res.send(stdout);
    });
}
```

:::
  </div>

  <div class="step">
    <div class="step-label">2. Compile the Bundle</div>
    <p>Frensense compiles your examples into a highly-optimized, multi-dimensional binary bundle.</p>
    
```bash
$ frensense corpus/targets --build-bundle frensense-corpus.frc

[INFO] Analyzing 1 corpus pair...
[INFO] Fingerprinting control flow and PDG reachability...
[INFO] Indexed 1 pattern (2 AST models) in 14ms.
[INFO] Emitted bundle: frensense-corpus.frc
```
  </div>

  <div class="step">
    <div class="step-label">3. Scan and Detect</div>
    <p>Run the engine against your codebase. It performs exact similarity search, tracking tainted data flow to catch variants of the vulnerability.</p>

```bash
$ frensense src/ --use-compiler

[!] Command Injection Detected (Similarity: 98.4%)
  --> src/api/network.ts:42
   |
42 |   exec(`ping -c 4 ${req.query.host}`, (err, output) => {
   |   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = observation: User input passed to exec() via shell interpolation.
   = improvement: Use execFile() with an arguments array.

[INFO] Scan completed in 420ms. Found 1 issue.
```
  </div>
</div>

<style>
.showcase-container {
  max-width: 900px;
  margin: 4rem auto;
  padding: 0 1.5rem;
}
.showcase-title {
  font-size: 2.2rem;
  font-weight: 700;
  text-align: center;
  margin-bottom: 0.5rem;
  letter-spacing: -0.02em;
}
.showcase-subtitle {
  text-align: center;
  color: var(--vp-c-text-2);
  margin-bottom: 3rem;
  font-size: 1.1rem;
}
.step {
  margin-bottom: 3.5rem;
}
.step-label {
  font-weight: 600;
  font-size: 1.2rem;
  color: var(--vp-c-brand-1);
  margin-bottom: 0.5rem;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}
.step p {
  color: var(--vp-c-text-2);
  margin-bottom: 1rem;
}
</style>
