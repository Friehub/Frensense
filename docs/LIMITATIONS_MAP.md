# Frensense Limitations Map
## Visual Guide to What Works, What Doesn't, and Why

---

## Detection Coverage Matrix

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         DETECTION COVERAGE                                  │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────────┐    ┌─────────────────────┐    ┌────────────────┐  │
│  │   CORPUS-BASED      │    │   HARDCODED          │    │   FINDING      │  │
│  │   (Example Pairs)   │    │   (Rust Code)        │    │   MODULES      │  │
│  ├─────────────────────┤    ├─────────────────────┤    ├────────────────┤  │
│  │ ✅ CSA patterns     │    │ ✅ TOCTOU (Prisma)  │    │ ✅ UNUSED_VAR  │  │
│  │ ✅ Hollow validators│    │ ✅ Temporal (lock)   │    │ ✅ DEAD_BRANCH │  │
│  │ ✅ SQL injection    │    │ ✅ Mutex across await│    │ ✅ HALLUC_IMPORT│  │
│  │ ✅ Cmd injection    │    │                      │    │ ✅ SECRET_*    │  │
│  │ ✅ Path traversal   │    │ ❌ TOCTOU (TypeORM)  │    │ ✅ STYLE       │  │
│  │ ✅ Open redirect    │    │ ❌ TOCTOU (Sequelize)│    │ ✅ NEAR_DUP    │  │
│  │ ✅ SSRF             │    │ ❌ TOCTOU (Knex)     │    │                │  │
│  │ ✅ Prototype pollution│   │ ❌ TOCTOU (raw SQL)  │    │                │  │
│  │ ✅ Hardcoded secrets│    │                      │    │                │  │
│  │ ✅ Timing attacks   │    │                      │    │                │  │
│  └─────────────────────┘    └─────────────────────┘    └────────────────┘  │
│                                                                             │
│  LIMITATION:                    LIMITATION:                 LIMITATION:     │
│  - Generic advisory text       - Prisma-only                - Per-file only │
│  - No pattern-specific fix     - Hardcoded patterns         - No cross-file │
│  - Requires example pairs      - Not extensible             - No taint aware│
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Taint Analysis Flow & Limitations

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         TAIT ANALYSIS PIPELINE                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  PHASE 1: SOURCE SEEDING (❌ REGEX-BASED)                                  │
│  ─────────────────────────────────────────                                  │
│                                                                             │
│    Variable Name ──→ Regex Match ──→ Tainted?                               │
│                                                                             │
│    "input"         matches "input"  → YES (correct)                        │
│    "userData"      matches "input"  → YES (correct)                        │
│    "url"           matches "url"    → YES (⚠️ FALSE POSITIVE)              │
│    "apiUrl"        matches "url"    → YES (⚠️ FALSE POSITIVE)              │
│    "internalUrl"   matches "url"    → YES (❌ FALSE POSITIVE)              │
│                                                                             │
│  PROBLEM: Name-based, not origin-based                                     │
│  FIX: AST entry point detection (T-FIX-1)                                  │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  PHASE 2: PROPAGATION                                                      │
│  ─────────────────────                                                      │
│                                                                             │
│    Tainted Var ──→ Assignment ──→ Tainted                                  │
│                ──→ Member Access ──→ Tainted                               │
│                ──→ Function Call ──→ Tainted (⚠️ NO SANITIZER CHECK)       │
│                ──→ Array/Object ──→ Tainted                                │
│                                                                             │
│  PROBLEM: Sanitizers not recognized                                        │
│  FIX: T-FIX-3 (read sanitizers from TOML)                                 │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  PHASE 3: SINK DETECTION                                                   │
│  ────────────────────────                                                   │
│                                                                             │
│    Tainted Data ──→ exec/system/shell ──→ TAINT_INPUT_TO_EXEC (Critical)   │
│                 ──→ db.insert/update   ──→ TAINT_CREDENTIAL_TO_DB (Critical)│
│                 ──→ log/console/print  ──→ TAINT_CREDENTIAL_TO_LOG (Warn)  │
│                 ──→ fetch/http/request ──→ TAINT_INPUT_TO_HTTP (Warn)       │
│                 ──→ write/open/remove  ──→ TAINT_INPUT_TO_FS (Warn)        │
│                                                                             │
│  WORKS, but high FP due to Phase 1 issues                                  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## TOCTOU Detection: What Works vs What Doesn't

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         TOCTOU DETECTION MATRIX                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ORM/LIBRARY        │ READ PATTERN           │ WRITE PATTERN  │ DETECTED?  │
│  ───────────────────┼────────────────────────┼────────────────┼────────────│
│  Prisma             │ prisma.user.findUnique │ prisma.user.   │ ✅ YES     │
│                     │ prisma.user.findFirst  │   update       │            │
│                     │ prisma.user.findMany   │ prisma.user.   │            │
│                     │                        │   create       │            │
│  ───────────────────┼────────────────────────┼────────────────┼────────────│
│  TypeORM            │ repo.findOne()         │ repo.update()  │ ❌ NO      │
│                     │ repo.find()            │ repo.save()    │            │
│                     │ repo.findOneBy()       │ repo.insert()  │            │
│  ───────────────────┼────────────────────────┼────────────────┼────────────│
│  Sequelize          │ User.findOne()         │ user.update()  │ ❌ NO      │
│                     │ User.findAll()         │ User.create()  │            │
│                     │ User.findByPk()        │ user.save()    │            │
│  ───────────────────┼────────────────────────┼────────────────┼────────────│
│  Knex               │ knex('t').first()      │ knex('t').     │ ❌ NO      │
│                     │ knex('t').where()      │   update()     │            │
│                     │                        │ knex('t').     │            │
│                     │                        │   insert()     │            │
│  ───────────────────┼────────────────────────┼────────────────┼────────────│
│  Drizzle            │ db.query.t.findFirst() │ db.update()    │ ❌ NO      │
│                     │ db.select().from()     │ db.insert()    │            │
│  ───────────────────┼────────────────────────┼────────────────┼────────────│
│  Raw SQL            │ SELECT ...             │ INSERT/UPDATE  │ ❌ NO      │
│                     │ db.query('SELECT...')  │ db.execute()   │            │
│  ───────────────────┼────────────────────────┼────────────────┼────────────│
│  MongoDB            │ collection.findOne()   │ collection.    │ ❌ NO      │
│                     │ collection.find()      │   updateOne()  │            │
│                     │                        │ collection.    │            │
│                     │                        │   insertOne()  │            │
│  ───────────────────┼────────────────────────┼────────────────┼────────────│
│                                                                             │
│  WHY: Hardcoded in helpers.rs lines 83-113                                  │
│  FIX: Add corpus patterns for each ORM                                      │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Corpus Pattern Limitations

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                      CORPUS PATTERN LIMITATIONS                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  WHAT WORKS:                                                               │
│  ───────────                                                               │
│  ✅ Function shape matching (7-dimensional fingerprinting)                 │
│  ✅ Contrastive scoring (positive vs negative)                             │
│  ✅ MinHash LSH for fast retrieval                                         │
│  ✅ Cross-lingual transfer (25% penalty)                                   │
│  ✅ IDF weighting (rare tokens score higher)                               │
│                                                                             │
│  WHAT DOESN'T WORK:                                                        │
│  ───────────────────                                                        │
│  ❌ Generic advisory text (no pattern-specific explanation)                │
│  ❌ No positive_similarity/negative_similarity in output                   │
│  ❌ Requires example pairs (can't detect novel bugs)                       │
│  ❌ Limited to function-level (no cross-function patterns)                 │
│  ❌ No semantic understanding (just shape matching)                        │
│                                                                             │
│  EXAMPLE OF PROBLEM:                                                       │
│  ────────────────────                                                       │
│  Finding: CORPUS_TS_CSA_VALIDATE_UNCONDITIONAL                             │
│  Observation: "Function shape matches a known violation pattern."          │
│  Impact: "Function shape matches a known violation pattern."               │
│  Improvement: "Review against corpus example."                             │
│                                                                             │
│  Developer reaction: "What pattern? What violation? How do I fix it?"      │
│                                                                             │
│  SHOULD BE:                                                                │
│  Observation: "Function 'validateUser' always returns true regardless      │
│                of input. A validator must have a rejection path."           │
│  Impact: "Invalid input passes validation, enabling auth bypass."          │
│  Improvement: "Add conditional branches that return false/error for        │
│                invalid input. See ts_csa_validate_unconditional_negative.ts"│
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## How to Extend: Decision Tree

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    HOW TO ADD NEW DETECTION                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Can you write a positive example (the bug) and negative (the fix)?        │
│                                                                             │
│          │                                                                  │
│          ▼                                                                  │
│    ┌─────────┐                                                              │
│    │   YES   │                                                              │
│    └────┬────┘                                                              │
│         │                                                                   │
│         ▼                                                                   │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │  USE CORPUS PATTERN                                                │    │
│  │                                                                     │    │
│  │  1. Create {lang}_{name}_positive.{ext}                           │    │
│  │  2. Create {lang}_{name}_negative.{ext}                           │    │
│  │  3. Create {lang}_{name}.toml with advisory text                  │    │
│  │  4. Rebuild bundle: cargo run --bin build-corpus-bundle           │    │
│  │                                                                     │    │
│  │  Pros: No code changes, easy to add, example-based                │    │
│  │  Cons: Generic advisory text (until Item 1 is fixed)              │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│                                                                             │
│          │                                                                  │
│          ▼                                                                  │
│    ┌─────────┐                                                              │
│    │   NO    │  (pattern requires structural analysis, not shape matching)  │
│    └────┬────┘                                                              │
│         │                                                                   │
│         ▼                                                                   │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │  USE HARDCODED DETECTOR                                            │    │
│  │                                                                     │    │
│  │  1. Create src/semantic_patterns/my_detector.rs                   │    │
│  │  2. Implement SemanticPattern trait                                │    │
│  │  3. Register in semantic_patterns/mod.rs                          │    │
│  │                                                                     │    │
│  │  Pros: Full control, can use AST analysis                         │    │
│  │  Cons: Requires Rust code, harder to maintain                     │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│                                                                             │
│          │                                                                  │
│          ▼                                                                  │
│    ┌─────────┐                                                              │
│    │  N/A    │  (pattern is per-file only, no cross-file reasoning)        │
│    └────┬────┘                                                              │
│         │                                                                   │
│         ▼                                                                   │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │  USE FINDING MODULE                                                │    │
│  │                                                                     │    │
│  │  1. Create src/engine/findings/my_finding.rs                      │    │
│  │  2. Implement FindingModule trait                                  │    │
│  │  3. Register in findings/mod.rs                                   │    │
│  │                                                                     │    │
│  │  Pros: Access to full file snapshot, symbol registry              │    │
│  │  Cons: Per-file only, no cross-function analysis                  │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Features That Reduce False Positives

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                   FALSE POSITIVE REDUCTION FEATURES                         │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  1. SEMANTIC FILTERS (corpus/semantic_filters.toml)                        │
│  ──────────────────────────────────────────────────                         │
│  Before scoring, patterns check AST-level constraints:                      │
│  - contains_call_to: must have these calls                                 │
│  - must_not_contain_call_to: must NOT have these calls                     │
│  - function_name_regex: function name must match                           │
│  - contains_node_type: must have these AST nodes                           │
│  - must_not_contain_node_type: must NOT have these nodes                   │
│                                                                             │
│  Example: Promise catch pattern only matches functions with .then()         │
│           but without .catch() or .finally()                               │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  2. TAINT VERIFICATION (runner.rs:175-215)                                 │
│  ──────────────────────────────────────────                                 │
│  Corpus findings are verified against taint flow:                          │
│  - If taint verified: confidence +20%, tagged "taint-verified"            │
│  - If not verified: use raw corpus score                                   │
│                                                                             │
│  This reduces FPs on patterns that look like bugs but don't                │
│  actually have dangerous data flow.                                        │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  3. PER-CATEGORY CALIBRATION (runner.rs:166-173)                           │
│  ────────────────────────────────────────────                               │
│  Confidence can be adjusted per pattern category:                          │
│  - sec: security patterns                                                  │
│  - csa: contract surface analysis                                          │
│  - llm: LLM anti-patterns                                                 │
│  - arch: architecture patterns                                             │
│  - async: concurrency patterns                                             │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  4. IDF WEIGHTING (fingerprint.rs:34-49)                                   │
│  ────────────────────────────────────────                                   │
│  Rare tokens score higher than common ones:                                │
│  - "db::execute" appears in 2 patterns → high weight                      │
│  - "let x" appears in 80 patterns → low weight                            │
│                                                                             │
│  This reduces FPs on functions that happen to share common tokens          │
│  with patterns but are structurally different.                             │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Patcher Limitations

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         PATCHER LIMITATIONS                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  IMPORT INJECTION: TypeScript-only                                          │
│  ────────────────────────────────────                                       │
│  Uses regex: ^import\s+.*                                                   │
│  ✅ import { foo } from 'bar'                                              │
│  ✅ import React from 'react'                                              │
│  ❌ use std::collections::HashMap;  (Rust)                                 │
│  ❌ from typing import List  (Python)                                      │
│                                                                             │
│  ATOMIC PATCHING: No rollback                                               │
│  ─────────────────────────────────                                          │
│  Writes to .patch_tmp, then atomic rename                                   │
│  If rename fails → .patch_tmp left behind                                   │
│  No multi-file atomic patches                                               │
│                                                                             │
│  CONTEXT MISMATCH: Fatal                                                    │
│  ───────────────────────────                                                │
│  If byte content doesn't match → entire patch fails                         │
│  No partial application                                                     │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Secret Scanner Limitations

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                      SECRET SCANNER LIMITATIONS                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  DETECTED:                                                                  │
│  ✅ AWS Access Key (AKIA...)                                               │
│  ✅ AWS Secret Key                                                         │
│  ✅ GitHub Token (ghp_...)                                                 │
│  ✅ Generic API Key (with quotes)                                          │
│  ✅ JWT Token (eyJ...)                                                     │
│  ✅ Private Keys (RSA/DSA/EC/PGP)                                         │
│  ✅ Connection Strings (mongodb/postgresql/redis://...)                    │
│  ✅ Slack Token (xox...)                                                   │
│  ✅ Google API Key (AIza...)                                               │
│                                                                             │
│  NOT DETECTED:                                                              │
│  ❌ Azure/Azure DevOps tokens                                              │
│  ❌ GitLab tokens                                                          │
│  ❌ npm tokens                                                             │
│  ❌ PyPI tokens                                                            │
│  ❌ Base64-encoded secrets                                                 │
│  ❌ Unquoted assignments (apiKey = abc123)                                 │
│  ❌ Secrets in comments                                                    │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Temporal Rules Limitations

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                     TEMPORAL RULES LIMITATIONS                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  BUILT-IN RULES (5):                                                        │
│  1. lock → unlock (error)                                                   │
│  2. acquire → release (error)                                               │
│  3. open → close (warning)                                                  │
│  4. connect → disconnect (warning)                                          │
│  5. lock → sleep (error)                                                    │
│                                                                             │
│  LIMITATIONS:                                                               │
│  ❌ Line-number-based ordering only (ignores control flow)                  │
│  ❌ No scope awareness (events from different functions mixed)              │
│  ❌ RAII patterns handled but limited                                       │
│                                                                             │
│  EXAMPLE OF PROBLEM:                                                        │
│  ────────────────────                                                       │
│  fn foo() {                                                                │
│      if condition {                                                         │
│          lock();  // line 10                                                │
│      }                                                                     │
│      // unlock();  // line 15 — different scope!                           │
│  }                                                                         │
│  This is VALID but may be flagged incorrectly.                             │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Config Limitations

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         CONFIG LIMITATIONS                                  │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  CURRENT OPTIONS (YAML):                                                    │
│  ──────────────────────                                                     │
│  rules_dir: custom rules directory                                          │
│  disabled_rules: rules to disable                                           │
│  severity_override: per-rule severity overrides                            │
│                                                                             │
│  CLI-ONLY (not in config file):                                             │
│  ─────────────────────────────                                              │
│  --threshold, --corpus, --extra-taint-rules                                │
│  --severity, --diff-only, --fix                                            │
│  --json, --sarif, --strict                                                 │
│                                                                             │
│  LIMITATIONS:                                                               │
│  ❌ Only 3 config options                                                   │
│  ❌ No validation of config file                                            │
│  ❌ Silent fallback to defaults on parse error                              │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Finding Module Limitations

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    FINDING MODULE LIMITATIONS                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  CROSS-FILE TAINT: NO-OP                                                    │
│  ─────────────────────────                                                  │
│  File: src/engine/findings/cross_file_taint.rs                             │
│  Returns empty Vec! Cross-file taint is handled by corpus layer instead.   │
│  This means finding modules have NO cross-file taint capability.           │
│                                                                             │
│  DEAD BRANCH: Simple Reachability                                          │
│  ──────────────────────────────────                                         │
│  Uses ReachabilityChecker from semantics/reachability.rs                   │
│  Detects: if false { ... }, if true { ... }                               │
│  Does NOT detect: complex condition analysis, provably dead code           │
│                                                                             │
│  UNUSED VARIABLE: CFG-Based                                               │
│  ────────────────────────────                                               │
│  Uses def-use chains from cfg/def_use.rs                                   │
│  Excludes: function parameters, variables prefixed with _                  │
│  Does NOT detect: variables used only in dead branches                     │
│                                                                             │
│  TEMPORAL VIOLATION: Line-Number Based                                     │
│  ──────────────────────────────────────                                     │
│  Loads rules from temporal_rules.toml (5 built-in rules)                   │
│  Validates: lock→unlock, acquire→release, open→close, etc.                │
│  Does NOT detect: scope violations, cross-function temporal issues         │
│                                                                             │
│  ATOMIC SECTION: C-Focused                                                 │
│  ──────────────────────────                                                 │
│  Originally designed for C lock/unlock pairing                             │
│  Detects: incomplete lock sections, TOCTOU with cond_wait                  │
│  Does NOT detect: other TOCTOU patterns (check-then-act without locks)    │
│                                                                             │
│  SEMANTIC PATTERNS: Prisma-Only                                            │
│  ───────────────────────────────                                            │
│  Only CHECK_THEN_ACT_TOCTOU is registered                                  │
│  Only recognizes Prisma ORM patterns                                       │
│  Does NOT detect: TypeORM, Sequelize, Knex, raw SQL TOCTOU                │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Taint Rules: Embedded, Not External

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                       TAIT RULES ARCHITECTURE                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  NOTE: There is NO taint_rules.toml file!                                  │
│                                                                             │
│  Taint rules are EMBEDDED in the engine code:                              │
│  - Source patterns: regex in resolve.rs                                    │
│  - Sink patterns: regex in resolve.rs                                      │
│  - Severity: hardcoded per rule                                            │
│                                                                             │
│  The --extra-taint-rules flag loads user-provided TOML files that          │
│  EXTEND (not replace) built-in rules.                                      │
│                                                                             │
│  BUILT-IN RULES (6):                                                       │
│  1. TAINT_CREDENTIAL_TO_DB — password/secret → db.insert/update            │
│  2. TAINT_INPUT_TO_EXEC — input → exec/system/shell/eval                  │
│  3. TAINT_CREDENTIAL_TO_LOG — password/secret → log/console/print         │
│  4. TAINT_INPUT_TO_FS — input → write/open/remove                         │
│  5. TAINT_INPUT_TO_HTTP — input → fetch/http/request                      │
│  6. TAINT_CREDENTIAL_TO_HTTP — password/secret → fetch/http               │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Priority Fixes (From ROADMAP.md)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                      PRIORITY FIXES                                         │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  🔴 CRITICAL (Blocks meaningful use)                                       │
│  ────────────────────────────────────                                       │
│  T-FIX-1: AST Entry-Point Based Source Seeding                             │
│           File: src/semantics/taint_entry_points.rs (new)                  │
│           File: src/semantics/data_flow/resolve.rs (modify)                │
│           Impact: TAINT_INPUT_TO_HTTP FP rate: 100% → <20%                │
│                                                                             │
│  🟡 HIGH (Improves accuracy)                                               │
│  ─────────────────────────────                                              │
│  T-FIX-3: Sanitizer Recognition                                            │
│           File: src/semantics/data_flow/resolve.rs                         │
│           Impact: Tainted data through sanitizers不再reach sinks           │
│                                                                             │
│  Item 1: Advisory TOML Sidecars                                            │
│          Files: frensense-engine/src/corpus/bundle.rs                      │
│                 src/engine/project/runner.rs:590-596                       │
│          Impact: Corpus findings show pattern-specific text                │
│                                                                             │
│  🟢 MEDIUM (Extends coverage)                                              │
│  ─────────────────────────────                                              │
│  Add corpus patterns for TypeORM, Sequelize, Knex, Drizzle, MongoDB        │
│  Impact: TOCTOU detection for all major ORMs                               │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```
