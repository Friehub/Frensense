# Corpus Conventions

## Naming Convention

```
{language}_{category}_{descriptive_name}_{variant}.{ext}
```

| Part | Example | Notes |
|------|---------|-------|
| language | `ts`, `rust`, `js` | Matches `ext_to_language()` in parser.rs |
| category | `ssrf`, `cmdi`, `sqli` | Used by auto-filter for group analysis |
| descriptive_name | `fetch_direct`, `exec_shell` | kebab-case, describes the specific pattern |
| variant | `positive`, `negative`, `negative2`, `m1_async_positive` | Quality suffix |
| ext | `ts`, `rs` | Must match supported languages |

**Mutation naming**: `{base}_m{variant}_{descriptor}_{positive|negative}.ts`
- `ts_ssrf_fetch_direct_m1_async_positive.ts` — M1: async/await variant
- `ts_ssrf_fetch_direct_m2_varnames_positive.ts` — M2: different variable names
- `ts_ssrf_fetch_direct_m4_trycatch_positive.ts` — M4: error handling wrapper

## Directory Structure

```
corpus/targets/
├── by-vuln/            ← NEW: organized by vulnerability → API version
│   ├── ssrf/
│   │   ├── node-fetch/
│   │   ├── request/
│   │   └── axios/
│   ├── sqli/
│   │   ├── pg/
│   │   ├── mysql2/
│   │   └── sequelize/
│   └── cmdi/
│       ├── exec/
│       ├── execfile/
│       └── spawn/
├── route-handlers/     ← Patterns matching Express/Hono/Fastify route handlers
├── config/             ← Configuration/middleware patterns
├── test/               ← Test-specific patterns
├── utility/            ← Utility/helper patterns
├── mock/               ← Mock/stub patterns
└── *.ts, *.rs          ← Root is for unclassified / legacy patterns
```

Files in subdirectories are automatically found by `collect_corpus_files()` (recursive).
The subdirectory determines the file's `FileContext` environment.

## Required `[frensense]` Block Fields

Every positive file must have a `[frensense]` comment block with:

```typescript
// [frensense]
// observation: Human-readable description of the bug (what it looks like).
// impact: What an attacker can do (the consequence).
// improvement: How to fix it (actionable).
// cwe: CWE-918            ← required for Tier 1-2
// cvss: 8.8               ← required for Tier 1
// owasp: A10:2021          ← required for Tier 2
// severity: High           ← optional
// runtime_probe: ssrf      ← required for Tier 1
// tier: 1                  ← optional, inferred from category
```

## Tier Requirements

| Tier | Description | Positives | Negatives | CWE | CVSS | Runtime Probe |
|------|-------------|-----------|-----------|-----|------|---------------|
| 1 | Core Security | ≥7 (base + 4 mutations) | ≥4 | Required | Required | Required |
| 2 | Auth & Access | ≥5 | ≥3 | Required | — | — |
| 3 | Logic Bugs | ≥4 | ≥2 | — | — | — |
| 4 | Code Quality | ≥3 | ≥2 | — | — | — |
| 5 | Framework/LLM | ≥2 | ≥1 | — | — | — |

## Quality Scoring

Run `cargo run --bin corpus-quality -- corpus/targets/` to score every pattern 0-100.

**Scoring criteria:**
- +20: Full `[frensense]` block (observation + impact + improvement)
- +15: Has import statement
- +15: Has 2+ functions
- +15: Typed HTTP handler parameters
- +10: Explicit taint source (`req.body.X`, `c.Query("X")`)
- +10: CWE identifier
- +10: Negative uses same sink call safely
- −20: File under 10 lines
- −20: Placeholder names (foo, bar, test)
- −10: `req: any` throughout
- −10: Only one function

**Score targets:** ≥80 = good, 50-79 = needs work, <50 = rewrite candidate.

## Creating Multi-API Variants

To make the engine generalize across codebases, create the same pattern using
different APIs:

```bash
corpus/targets/by-vuln/ssrf/
├── node-fetch/ts_ssrf_fetch_positive.ts      # import fetch from "node-fetch"
├── request/ts_ssrf_request_positive.ts        # const request = require("request")
└── axios/ts_ssrf_axios_positive.ts            # import axios from "axios"
```

Each variant has:
- Same `cwe:` and `cvss:` (same vulnerability)
- Different imports and API calls
- Same vulnerability structure (user input → sink)

The auto-filter sees all variants and learns that NO single API is required —
it generalizes to "any HTTP client call leading to user-controlled URL".

## Negative File Requirements

- Must have `// SAFE: explanation` at the top
- Must use the SAME structure as the positive (same imports, functions, params)
- Must still call the same sink API — just safely (validated, parameterized)
- Must NOT simply delete the vulnerable call
- Should have 2+ negatives per pattern (primary fix + alternate approach)

## Bundle Building

```bash
# Build the bundle (takes 10-15 minutes with 4000+ files)
cargo run --bin build-corpus-bundle

# Rebuild binary with embedded bundle
touch src/bin/frensense.rs
cargo build --bin frensense
```

The bundle embeds all patterns, fingerprints, learned weights, and auto-filter stats.
Falls back to directory loading when bundle format is incompatible.
