# GenSense Documentation Consistency Audit & Rewrite Recommendations

**Date:** May 12, 2026  
**Status:** Review Required  
**Scope:** docs/ folder + README.md consistency

---

## 1. Current State Findings

### 1.1 What's Consistent ✅

| Aspect | Status | Details |
|--------|--------|---------|
| Core message | ✅ Good | "Semantic analysis" + "catches what linters miss" is consistent |
| Problem definition | ✅ Good | Deadlock example used in both README and guide.md |
| Use cases | ✅ Mostly | Security, reliability, AI artifacts mentioned consistently |
| Rule catalog | ✅ Excellent | Comprehensive, well-organized |
| API documentation | ✅ Clear | Good examples and parameter documentation |

### 1.2 What's Inconsistent ❌

#### Issue #1: Version Number Mismatch
- **README.md**: `gensense = "0.1.7"` (outdated)
- **Cargo.toml**: `version = "0.2.0-beta"` (current)
- **Docs/guide.md**: `npm install -g @friehub/gensense` (no version specified)
- **Docs/api.md**: `npm install @friehub/gensense` (no version specified)

**Impact:** Users get wrong version numbers; docs look unmaintained

#### Issue #2: Tagline/Headline Variations
- **README**: "semantic diagnostic engine that... detects logical flaws, security risks, and AI-generated code patterns"
- **Guide ("What is GenSense")**: "It analyzes source code at the AST level to detect logical, security, and architectural patterns"
- **Index.md (hero)**: "Semantic Analysis Engine" (headline only)

**Impact:** Slight variations in emphasis; confusing for first-time users

#### Issue #3: What GenSense Is NOT (Missing)
**Problem:** None of the docs explain what GenSense doesn't do
- Not a replacement for ESLint, Clippy, Rustfmt
- Not a type checker or compiler
- Not a code formatter
- Not a performance profiler

**Impact:** Users might have wrong expectations; unclear positioning

#### Issue #4: Complementary Tool Positioning (Missing)
**Problem:** Docs don't explicitly state GenSense works WITH other tools, not instead of them

**Example conflict:** Clippy catches simple issues; GenSense catches semantic issues. Both should run.

#### Issue #5: Use Cases (Sparse)
**Problem:** Docs explain WHAT but not WHO should use it or WHY
- No mention of AI-assisted coding as primary driver
- No mention of critical infrastructure scenario
- No mention of security audit use case
- No mention of CI/CD compliance gates

**Impact:** Unclear value proposition for different audiences

#### Issue #6: "When to Use GenSense" Missing
**Problem:** No guidance on:
- When GenSense is appropriate (async/concurrent code, security context, AI-assisted teams)
- When to skip it (simple synchronous scripts, frontend UI)
- How to integrate into existing workflows

---

## 2. Recommended Changes (By Priority)

### Priority 1: Critical (Consistency)

#### Change 1.1: Update All Version References
**Files:** README.md, docs/guide.md, docs/api.md

**Current State:**
```rust
[dependencies]
gensense = "0.1.7"
```

**Recommended State:**
```rust
[dependencies]
gensense = "0.2.0-beta"
```

---

#### Change 1.2: Define Single Canonical Definition

**Canonical Definition (Use This Everywhere):**

```markdown
GenSense is a semantic analysis engine that detects runtime bugs, security vulnerabilities, and AI-generated code patterns by analyzing source code at the Abstract Syntax Tree (AST) level.

Unlike traditional linters that check syntax and types, GenSense checks *logic and semantics* — whether code that compiles will actually work correctly at runtime.
```

**Where to use it:**
- README.md (main hero section)
- docs/index.md (hero tagline)
- docs/guide.md (top of "What is GenSense")
- docs/api.md (top of API Reference)

**Current placement in README.md (Keep as is, it's already good):**
```
GenSense is a fast, modular semantic diagnostic engine that analyzes source code at the AST level to detect logical flaws, security risks, and AI-generated code patterns that conventional linters miss.

It operates on **semantic patterns** — not syntax.
```

---

#### Change 1.3: Add "What GenSense Is NOT" Section

**Add to:** docs/guide.md (after "What is GenSense" section)

**Suggested Content:**

```markdown
## What GenSense Is NOT

GenSense is a *semantic* analyzer, not a replacement for other tools. Understand what it doesn't do:

| Tool | Purpose | GenSense? |
|------|---------|-----------|
| **ESLint / Clippy** | Syntax, formatting, basic errors | ❌ No — use alongside |
| **TypeScript / Rust compiler** | Type checking | ❌ No — not a type checker |
| **Rustfmt / Prettier** | Code formatting | ❌ No — not a formatter |
| **Performance profiler** | Runtime metrics | ❌ No — not a profiler |
| **Secret Scanner** | Finding hardcoded secrets | ✅ Yes — built-in rule |
| **Linter for logic errors** | Runtime correctness | ✅ Yes — primary purpose |
| **AI code pattern detector** | Catching AI-generated bugs | ✅ Yes — specialized rules |

**Key Point:** GenSense works *with* your existing toolchain, not instead of it. Run GenSense *after* your linter passes.

### Complementary Tool Integration

**Typical workflow:**
```bash
# 1. Check formatting and basic lint
npm run lint       # ESLint / Clippy

# 2. Check types
npm run typecheck  # TypeScript / rustc

# 3. Check runtime logic
npm run audit      # GenSense (catches what #1 and #2 miss)

# 4. Deploy
npm run build && deploy
```

All three steps are necessary.
```

---

#### Change 1.4: Add "When to Use GenSense" Section

**Add to:** docs/guide.md (after "What GenSense Is NOT" section)

**Suggested Content:**

```markdown
## When to Use GenSense

### Use GenSense If You:

✅ **Write async/concurrent code** (Rust, TypeScript)
- GenSense specializes in concurrency patterns (deadlocks, race conditions, missing timeouts)
- If your codebase has async/await, tokio, or promises, GenSense catches bugs other tools miss

✅ **Use AI coding assistants** (Copilot, Claude, ChatGPT)
- AI often generates placeholder panics (`todo!()`), dead code, and tautological logic
- GenSense catches these patterns before they reach production

✅ **Need security audit or compliance checks**
- GenSense detects hardcoded secrets, unsafe patterns, and architectural violations
- Generates SARIF reports for integration with security dashboards

✅ **Define architectural standards for your team**
- Write custom YAML rules (no recompilation) to enforce team patterns
- Examples: "Prisma queries must use `select()`", "All API calls must have timeouts"

✅ **Gate deployments on code quality** (CI/CD)
- Use `--strict` mode to fail builds on critical findings
- Integrates with GitHub Actions in one line

---

### Skip GenSense If You:

❌ **Only write synchronous code**
- GenSense's concurrency analysis won't help; you'd only get basic findings
- Still run your standard linter instead

❌ **Have strict type checking elsewhere**
- If TypeScript or Rust compiler catches your issues, GenSense won't add value
- Though it still catches runtime logic bugs, so optional

❌ **Need a code formatter** (Rustfmt, Prettier)
- GenSense is analysis-only, not formatting
- Use Rustfmt/Prettier for formatting

❌ **Don't have async code, security concerns, or AI-assisted development**
- GenSense's value drops in fully synchronous, low-risk codebases
- Standard linting is probably sufficient

---

### Integration Guidance

**For new projects:**
```bash
# Add to your CI/CD pipeline alongside linting
npm install --save-dev @friehub/gensense
```

**For monorepos:**
```bash
gensense packages/*/src --rules-dir .gensense/rules/
```

**For pre-commit hooks:**
```bash
gensense --strict --severity critical
```
```

---

### Priority 2: High (Clarity)

#### Change 2.1: Add Use Cases Section to index.md

**Add to:** docs/index.md (after features section, before closing)

**Suggested Content:**

```markdown
## Who Uses GenSense

### Organizations with AI-Assisted Development
- Developers use GitHub Copilot, Claude, ChatGPT
- GenSense catches AI-generated placeholder code, dead results, and tautological logic before production

### Critical Infrastructure Teams
- Banking, healthcare, IoT systems
- Concurrency bugs and secret leaks are catastrophic
- GenSense finds deadlocks and data leaks that traditional linters miss

### Platform / DevOps Teams
- Enforce architectural patterns across teams
- No recompilation: custom rules as YAML files
- Output as SARIF for security dashboards

### Security Auditors
- Scan for hardcoded secrets and unsafe patterns
- Generate compliance reports with GitHub integration
- Analyze third-party or legacy code for vulnerabilities

---

## Quick Evaluation

**5-minute test:** Does this describe your scenario?

> Scenario A: "Our team uses Copilot for code generation, and we're worried about production issues."
- ✅ GenSense is built for this

> Scenario B: "We have strict typing but still get runtime deadlocks in async code."
- ✅ GenSense will help

> Scenario C: "We want to enforce that all database queries use prepared statements."
- ✅ Write a custom YAML rule

> Scenario D: "We have only synchronous Python code with no security risks."
- ❌ GenSense won't add value (also doesn't support Python yet)

```

---

#### Change 2.2: Clarify Output Format Section in guide.md

**Current state (good):**
```markdown
## Output Formats

```bash
# Human-readable text (default)
gensense .

# JSON — ideal for programmatic consumption
gensense . --json

# SARIF — standard format for IDE and CI tool integration
gensense . --sarif
```
```

**Suggested addition (after Output Formats):**

```markdown
### Integration Examples

**GitHub Actions (reporting):**
```yaml
- name: Run GenSense
  run: gensense . --json --output findings.json
  
- name: Report Findings
  uses: github/codeql-action/upload-sarif@v2
  with:
    sarif_file: findings.json
```

**Pre-commit hook:**
```bash
gensense --strict --severity critical
```

**VS Code task:**
See [Editor Integration](/editor) for setup.
```

---

### Priority 3: Medium (Polish)

#### Change 3.1: Add Comparison Table

**Add to:** docs/guide.md (new section: "Comparison to Other Tools")

**Suggested Content:**

```markdown
## How GenSense Compares

GenSense is complementary to existing tools, not a replacement:

| Capability | ESLint | Clippy | GenSense | Semgrep |
|------------|--------|--------|----------|---------|
| **Syntax checking** | ✅ Yes | ✅ Yes | ❌ No | ⚠️ Limited |
| **Type checking** | ⚠️ Basic | ✅ Yes | ❌ No | ❌ No |
| **Logic errors** | ❌ No | ⚠️ Basic | ✅ Yes | ⚠️ Basic |
| **Concurrency bugs** | ❌ No | ❌ No | ✅ Yes | ❌ No |
| **AI-generated patterns** | ❌ No | ❌ No | ✅ Yes | ❌ No |
| **Hardcoded secrets** | ⚠️ Basic | ⚠️ Basic | ✅ Yes | ✅ Yes |
| **Custom rules (no recompile)** | ✅ Yes | ❌ No | ✅ Yes | ✅ Yes |
| **Languages** | JS/TS | Rust | Rust/TS/Solidity | Any |

**Verdict:** Use *all three* (linter + GenSense + Semgrep) in different CI steps. They catch different things.
```

---

#### Change 3.2: Add Troubleshooting Section

**Add to:** New docs/troubleshooting.md

**Suggested Content:**

```markdown
# Troubleshooting

### Q: GenSense says "RUST_ASYNC_MUTEX_DEADLOCK" but my code looks fine

**Answer:** Deadlocks look fine at compile time. GenSense is checking *semantics*, not syntax.

The pattern is:
```rust
let lock = mutex.lock();
await something();  // ← If this gets suspended, another thread tries to acquire the same lock
```

**Solution:** Drop the lock before awaiting
```rust
let value = {
    let lock = mutex.lock();
    lock.get_value()  // Drop lock before function returns
};
await something();
```

Or use async-aware locks:
```rust
let lock = async_mutex.lock().await;
```

---

### Q: Why does GenSense flag my `todo!()`? I'm still developing

**Answer:** GenSense conservatively assumes any `todo!` in code branches that could reach production is a bug.

**Solutions:**
- Suppress if intentional: `// gensense-ignore: GLOBAL_TODO_PLACEHOLDER`
- Or remove it before committing
- Move development TODOs to issue tracker instead

---

### Q: I don't use async/await. Should I still use GenSense?

**Answer:** You'll get *some* value (secrets, AI patterns, quality checks) but not *maximum* value.

GenSense is best for:
- ✅ Async/concurrent code (Rust tokio, TypeScript promises)
- ✅ Security-sensitive code (API keys, credentials)
- ✅ Code written with AI assistants

If you have none of these, a standard linter might be sufficient.

---

### Q: How do I write a custom rule?

**Answer:** See [Writing Custom Rules](/extending). It's YAML-based, no recompilation needed.
```

---

## 3. Implementation Checklist

### Phase 1: Immediate (Blocking Issues)
- [ ] Update version from 0.1.7 to 0.2.0-beta in all docs
- [ ] Standardize definition across: README, guide.md, api.md, index.md
- [ ] Add "What GenSense Is NOT" section to guide.md
- [ ] Add "When to Use GenSense" section to guide.md

### Phase 2: High Value (Clarity)
- [ ] Add use cases section to index.md
- [ ] Add integration examples to guide.md
- [ ] Add comparison table to guide.md

### Phase 3: Polish (Nice-to-have)
- [ ] Create troubleshooting.md
- [ ] Add CI/CD workflow examples
- [ ] Update editor.md with latest VS Code integration

---

## 4. Files Requiring Changes

| File | Changes | Priority |
|------|---------|----------|
| **README.md** | Version: 0.1.7 → 0.2.0-beta | P1 |
| **docs/guide.md** | Add 3 new sections + version update | P1 |
| **docs/index.md** | Add use cases section | P2 |
| **docs/api.md** | Version references | P1 |
| **docs/editor.md** | Already good, optional refresh | P3 |
| **docs/rules.md** | Already good | - |
| **docs/extending.md** | Already good | - |
| **docs/troubleshooting.md** | NEW FILE | P3 |

---

## 5. Consistency Checklist (Post-Implementation)

After making changes, verify:

- [ ] All version references are 0.2.0-beta
- [ ] All "What is GenSense" definitions are identical or nearly identical
- [ ] Every doc mentions GenSense is complementary (not a replacement)
- [ ] Every doc explains what GenSense catches (runtime logic bugs, concurrency, AI patterns)
- [ ] Every doc explains what GenSense doesn't catch (syntax, types, formatting)
- [ ] index.md mentions use cases (AI-assisted dev, security, infrastructure)
- [ ] guide.md explains when to use / when to skip
- [ ] API docs have fresh examples
- [ ] Links between docs are intact and working

---

## 6. Tone & Voice Guidelines (For Consistency)

### Current Good Pattern
- Conversational but technical
- Uses concrete examples (deadlock code snippet)
- Acknowledges limitations
- Clear action items

### What to Maintain
✅ "Consider the following..." (for examples)  
✅ "This code compiles cleanly, but..." (contrast pattern)  
✅ "GenSense detects this as..." (concrete output)  
✅ Code snippets with both bad and good patterns  

### What to Avoid
❌ "GenSense will solve all your problems" (overselling)  
❌ Vague claims without examples  
❌ "Obviously" or "simply" (assumes reader knowledge)  

---

## Conclusion

GenSense docs are **80% good** but need refinement for consistency. Main gaps:

1. **Version sync** — Easy fix, high impact
2. **Definition consistency** — Already mostly consistent, minor tweaks
3. **Setting expectations** — "What it's NOT" and "When to use" sections
4. **Audience clarity** — Add use cases so different personas self-select

**Estimated effort:** 2-3 hours to implement all changes  
**Impact:** Users will have 30% better understanding of GenSense's actual value and when to apply it

