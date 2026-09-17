---
layout: home

hero:
  name: "Frensense"
  text: "Deterministic Security Engine."
  tagline: "Static analysis without the rules."
  actions:
    - theme: brand
      text: Read the Paper
      link: /frensense-paper/00_OVERVIEW
    - theme: alt
      text: Read the Blog
      link: /blog/
---

<div class="showcase-container">
  
  <div class="about-section">
    <h2 class="showcase-title">About Frensense</h2>
    <p>
      Frensense is a high-performance static analysis system that is <strong>provider and language agnostic</strong>. It detects security vulnerabilities and architectural violations without relying on hand-written rules, DSLs, or regular expressions. 
    </p>
    <p>
      Instead of writing complex Abstract Syntax Tree (AST) queries, Frensense introduces a new paradigm: <strong>example-driven detection</strong>. By fingerprinting concrete pairs of vulnerable and fixed code, the engine inherently captures structural shape, control flow, API usage, and data flow simultaneously. 
    </p>
    <p>
      Its strict "AND-gate" architecture ensures that findings only trigger when exact structural similarity and precise, field-sensitive Program Dependence Graph (PDG) data-flow completely agree - drastically reducing false positives and generalizing across framework dialects natively.
    </p>
  </div>

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
/* Compress the default VitePress Hero padding */
:deep(.VPHero) {
  padding-top: 2rem !important;
  padding-bottom: 0rem !important;
  margin-top: 0 !important;
}
:deep(.VPHero .name) {
  font-size: 3rem !important;
}
:deep(.VPHome) {
  padding-bottom: 0 !important;
}

.showcase-container {
  max-width: 900px;
  margin: 2rem auto 4rem auto;
  padding: 0 1.5rem;
}
.about-section {
  margin-bottom: 4rem;
}
.about-section p {
  color: var(--vp-c-text-2);
  font-size: 1.15rem;
  line-height: 1.7;
  margin-bottom: 1.5rem;
  text-align: left;
}
.about-section strong {
  color: var(--vp-c-text-1);
}
.showcase-title {
  font-size: 2.2rem;
  font-weight: 700;
  text-align: left;
  margin-bottom: 1.5rem;
  letter-spacing: -0.02em;
}
.showcase-subtitle {
  text-align: left;
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
  text-align: left;
}
</style>
