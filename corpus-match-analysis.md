# Full Corpus Match Analysis — NodeGoat & Juice Shop

## NodeGoat (163 findings)

### Score Distribution
```
Score   Count  Type                    Detection Mechanism     sem_mult
------  -----  ----------------------  ----------------------  --------
0.50    21     REAL (ground truth)     MISSING_CALL/CONFIG     0.30x penalty
0.55    74     NOISE (false positives) CORPUS (flow-based)     2.0x boost
0.90    12     DEPENDENCY CHECKS       VULN_NPM_*              Fixed 0.90
```

### Real findings at 0.50 — ALL MISSING_CALL/CONFIG
```
A5-HELMET_MISSING     | server.js:40     | CWE-1021 | tags=[]
A5-X_POWERED_BY       | server.js:41     | CWE-200  | tags=[]
A5-NOSNIFF            | server.js:64     | CWE-200  | tags=[]
A5-COOKIE_NAME        | server.js:89     | CWE-200  | tags=[]
A5-COOKIE_FLAGS       | server.js:84     | CWE-614  | tags=[]
A5-SAVE_UNINIT        | server.js:84     | CWE-384  | tags=[]
A5-HTTP               | server.js:145    | CWE-319  | tags=[]
A8-CSRF_MIDDLEWARE     | server.js:106    | CWE-352  | tags=[]
A3-SWIG_AUTOESCAPE    | server.js:137    | CWE-79   | tags=[]
A3-MARKED_VULN        | server.js:126    | CWE-79   | tags=[]
A7-NO_ADMIN_CHECK     | index.js:55      | CWE-862  | tags=[]
A10-SSRF              | research.js:16   | CWE-918  | tags=[]
A3-WRONG_ENCODING     | profile.js:28    | CWE-79   | tags=[]
REDOS                 | profile.js:59    | CWE-1333 | tags=[]
A4-IDOR_PARAM         | allocations.js:18| CWE-639  | tags=[]
A1-LOG_INJECTION      | session.js:64    | CWE-117  | tags=[]
A2-NO_SESSION_REGEN   | session.js:116   | CWE-384  | tags=[]
A2-USER_ENUM          | session.js:83    | CWE-204  | tags=[]
A2-USER_ENUM          | user-dao.js:92   | CWE-204  | tags=[]
A2-WEAK_PW            | session.js:144   | CWE-521  | tags=[]
A2-WEAK_PW            | user-dao.js:17   | CWE-521  | tags=[]
```

**Common trait:** tags=[], no match_evidence, MISSING_CALL/CONFIG detection

### Noise at 0.55 — ALL CORPUS flow-based
```
CORPUS_TS_NEXTJS_REDIRECT_EXTERNAL       | 4 findings | flow_sim=1.0 semantic_sim=0.25
CORPUS_TS_EXPRESS5_ASYNC_ERROR_UNHANDLED  | 4 findings | flow_sim=1.0 semantic_sim=0.0
CORPUS_TS_EXPRESS_ASYNC_HANDLER_MISSING   | 4 findings | flow_sim=1.0 semantic_sim=0.0
CORPUS_TS_CJS_ERROR_STACK_LEAK            | 4 findings | flow_sim=1.0 semantic_sim=0.0
CORPUS_TS_SSE_DATA_INJECTION              | 4 findings | flow_sim=1.0 semantic_sim=0.0
CORPUS_TS_WEBRTC_STUN_SERVER_USER         | 4 findings | flow_sim=1.0 semantic_sim=0.0
CORPUS_TS_CSP_DANGLING_MARKER             | 4 findings | flow_sim=1.0 semantic_sim=0.0
CORPUS_TS_TYPE_CONFUSION_STRING_NUMBER    | 4 findings | flow_sim=1.0 semantic_sim=0.0
CORPUS_TS_SSTI_MUSTACHE_RENDER            | 4 findings | flow_sim=1.0 semantic_sim=0.0
CORPUS_TS_CACHE_UNKEYED_HEADER            | 4 findings | flow_sim=1.0 semantic_sim=0.0
CORPUS_TS_EXPRESS_MISSING_ERROR_HANDLER   | 4 findings | flow_sim=1.0 semantic_sim=0.0
CORPUS_TS_CJS_XSS_REFLECTED              | 4 findings | flow_sim=1.0 semantic_sim=0.0
CORPUS_TS_CJS_ERROR_500_NO_HANDLER        | 4 findings | flow_sim=1.0 semantic_sim=0.0
CORPUS_TS_RCE_VM_CREATECONTEXT             | 4 findings | flow_sim=1.0 semantic_sim=0.0
CORPUS_TS_MASS_ASSIGNMENT_BUILD_BODY      | 4 findings | flow_sim=1.0 semantic_sim=0.0
CORPUS_TS_CJS_WEAK_ID                     | 2 findings | flow_sim=1.0 semantic_sim=0.0
CORPUS_TS_ORG_INVITE_REUSE                | 2 findings | flow_sim=1.0 semantic_sim=0.0
CORPUS_TS_PAGINATION_NO_MAX_LIMIT         | 2 findings | flow_sim=1.0 semantic_sim=0.0
CORPUS_TS_HEADER_INJECTION_LOCATION       | 2 findings | flow_sim=1.0 semantic_sim=0.0
CORPUS_TS_IDOR_BODY_USERID                | 2 findings | flow_sim=1.0 semantic_sim=0.0
CORPUS_TS_AUDIT_MISSING_READ_OPERATIONS   | 2 findings | flow_sim=1.0 semantic_sim=0.0
CORPUS_TS_NEXTJS_REDIRECT_EXTERNAL        | 2 findings | flow_sim=1.0 semantic_sim=0.25
```

**Common trait:** tags=['corpus', 'pattern'], flow_sim=1.0, match_evidence present

### Dependency checks at 0.90
```
VULN_NPM_BCRYPT_NODEJS      | package.json:0
VULN_NPM_MARKED             | package.json:0
VULN_NPM_NEEDLE             | package.json:0
VULN_NPM_NODE_ESAPI         | package.json:0
VULN_NPM_SWIG               | package.json:0
VULN_NPM_HELMET             | package.json:0
VULN_NPM_EXPRESS_SESSION    | package.json:0
VULN_NPM_FOREVER            | package.json:0
VULN_NPM_GRUNT              | package.json:0
VULN_NPM_MOCHA              | package.json:0
VULN_NPM_NODEMON            | package.json:0
VULN_NPM_SELENIUM_WEBDRIVER | package.json:0
```

**Common trait:** tags=['vulnerable-dependency', 'npm'], separate detection mechanism

---

## Juice Shop (1696 findings)

### Score Distribution
```
Score     Real  Noise  Total  Notes
--------  ----  -----  -----  -----
0.35      1     0      1      SSRF_DNS_REBINDING_VECTOR
0.36      1     1      2      SSRF_DNS_REBINDING_VECTOR
0.37      1     0      1      IDOR_FINDBYPK_BODY_PARAM
0.38      1     1      2      SSRF_FETCH
0.39      1     2      3      SSRF_DNS_REBINDING_VECTOR, SSRF_FETCH
0.40      1     0      1      SSRF_DNS_REBINDING_VECTOR
0.41      1     2      3      SSRF_FETCH
0.42      1     1      2      SSRF_DNS_REBINDING_VECTOR
0.43      1     0      1      SSRF_FETCH
0.44      1     0      1      SSRF_FETCH
0.45      1     1      2      IDOR_FINDBYPK_BODY_PARAM
0.46      1     2      3      IDOR_FINDBYPK_BODY_PARAM
0.49      1     5      6      SSRF_DNS_REBINDING_VECTOR, SSRF_FETCH
0.50      1     1      2      SSRF_FETCH
0.51      0     1      1      SSRF_FETCH
0.52      1     2      3      SSRF_FETCH, SSRF_DNS_REBINDING_VECTOR
0.53      1     2      3      IDOR_FINDBYPK_BODY_PARAM, SSRF_DNS_REBINDING_VECTOR
0.54      1     2      3      SSRF_FETCH, IDOR_FINDBYPK_BODY_PARAM
0.55      702   942    1644   MASSIVE CLUSTER — indistinguishable
0.90      2     0      2      VULN_NPM_HELMET, VULN_NPM_GRUNT
```

### Key observation at 0.55
At score 0.55, real and noise findings are mixed with NO way to separate them:
```
Real findings at 0.55 (702):
  CORPUS_TS_SEQUELIZE_UPDATE_NO_OWNERSHIP: 22
  CORPUS_TS_SVELTEKIT_ENDPOINT_NO_AUTH: 20
  CORPUS_TS_SQLI_CONCAT_DIRECT: 20
  CORPUS_TS_CJS_ERROR_500_NO_HANDLER: 15
  CORPUS_TS_CJS_ERROR_STACK_LEAK: 15
  ...

Noise findings at 0.55 (942):
  CORPUS_TS_CJS_ERROR_STACK_LEAK: 34
  CORPUS_TS_TYPE_CONFUSION_STRING_NUMBER: 34
  CORPUS_TS_MASS_ASSIGNMENT_BUILD_BODY: 34
  CORPUS_TS_CJS_ERROR_500_NO_HANDLER: 34
  CORPUS_TS_IDOR_BODY_USERID: 34
  ...
```

**Same rules, same scores, can't tell real from noise.**

---

## Traced Evidence

### Noise finding: CORPUS_TS_NEXTJS_REDIRECT_EXTERNAL (0.55)
```json
{
  "rule_id": "CORPUS_TS_NEXTJS_REDIRECT_EXTERNAL",
  "confidence": 0.55,
  "tags": ["corpus", "pattern"],
  "match_evidence": {
    "ngram_sim": 0.0,
    "ast_sim": 0.57,
    "signature_sim": 0.0,
    "api_sim": 0.2,
    "semantic_sim": 0.25,
    "flow_sim": 1.0,
    "motif_sim": 1.0,
    "has_taint_path": true
  }
}
```
**Gate:** `flow_sim=1.0 > 0.8` → `sem_mult = SEMANTIC_MATCH_BOOST (2.0x)`

### Real finding: A5-HELMET_MISSING (0.50)
```json
{
  "rule_id": "A5-HELMET_MISSING",
  "confidence": 0.5,
  "tags": [],
  "match_evidence": {}
}
```
**Gate:** No match_evidence → `sem_mult = SEMANTIC_ZERO_PENALTY (0.30x)`

---

## The Scoring Flow

```
Candidate code
    ↓
Extract fingerprints (ngram, ast, api, semantic, flow, motif, ...)
    ↓
Compute dimensions (best_dim vs worst_neg)
    ↓
Weighted score = sum(dim_i * weight_i) * identity_gate
    ↓
sem_mult applied:
  - CORPUS + semantic_sim > 0 → 2.0x boost
  - CORPUS + no semantic_sim → 0.30x penalty
  - MISSING_CALL/CONFIG → 0.30x penalty (no match_evidence)
    ↓
Final score = weighted_score * sem_mult * context_multiplier
    ↓
Noise gate (max_signal > 0.4 OR moderate_count >= 3)
    ↓
Output as confidence
```

**The problem:** sem_mult is applied uniformly, but MISSING_CALL findings can't produce semantic_sim > 0.

---

## Root Cause

### sem_mult logic (scorer.rs line 453-459)
```rust
let sem_mult = if positive.semantic_markers.is_empty() {
    1.0
} else if dim.semantic_sim == 0.0 {
    SEMANTIC_ZERO_PENALTY   // 0.30x ← REAL FINDINGS HIT THIS
} else {
    SEMANTIC_MATCH_BOOST    // 2.0x  ← NOISE HIT THIS
};
```

### Why this is wrong

1. MISSING_CALL findings don't have match_evidence
2. They can't produce `semantic_sim > 0` — it's not how they work
3. They get penalized for a metric they can't produce
4. CORPUS findings have match_evidence and get boosted
5. Result: real findings score LOWER than noise

### The fundamental mismatch

Two different detection mechanisms are being scored the same way:
- **Flow-based** (CORPUS): traces data from source → sink, computes similarity
- **Missing-call** (MISSING_CALL/CONFIG): checks if a function is NOT called

Missing-call findings don't have match_evidence (ngram_sim, semantic_sim, etc.)
They get penalized by sem_mult because they can't produce semantic_sim > 0.

---

## Impact on Benchmarks

### NodeGoat
- Real findings: 0.50 (penalized)
- Noise: 0.55 (boosted)
- Gap: 10% — noise wins

### Juice Shop
- Real findings: 0.55 (same as noise)
- Noise: 0.55 (indistinguishable)
- Gap: 0% — can't separate at all

### Benchmark distortion
- Benchmarks run with `--min-confidence 0.0` (show everything)
- Real users run with default or `--confidence medium` (0.60+)
- At 0.60+: only 2 findings (both dependency checks)
- **No usable middle ground** — can't separate real from noise

---

## Recommended Fix

Skip sem_mult for non-corpus findings (MISSING_CALL/CONFIG types):

```rust
let sem_mult = if positive.semantic_markers.is_empty() {
    1.0
} else if !positive.tags.contains(&"corpus".to_string()) {
    // MISSING_CALL / CONFIG findings — different detection mechanism
    1.0
} else if has_flow_match || dim.semantic_sim > 0.0 {
    SEMANTIC_MATCH_BOOST
} else {
    SEMANTIC_ZERO_PENALTY
};
```

---

## Files Involved

- `frensense-engine/src/pattern/scorer.rs` — sem_mult logic (line 453-459)
- `frensense-engine/src/pattern/similarity.rs` — dimension computation
- `corpus/targets/legacy/nodegoat/ts_ns_*.js` — NodeGoat patterns
- `scripts/benchmark_nodegoat.py` — benchmark script (uses --min-confidence 0.0)
