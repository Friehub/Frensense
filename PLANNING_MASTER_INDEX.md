# GenSense v0.3.0+ Planning — Master Index

**Created**: 2026-05-14 | **Status**: Active | **Baseline**: v0.2.1 (Complete)

This is the master index for GenSense future development, connecting all planning documents, research sources, and implementation guides.

---

## 🎯 Mission

Complete the v0.3.0 roadmap by implementing 6 enhancements across 3 tiers:
- **Tier 1** (Quick wins, v0.2.2): F5, F4
- **Tier 2** (Strategic, v0.3.0-alpha): F2a, F2, F1
- **Tier 3** (High-value, v0.3.0): F6, F3
- **Future** (v0.4.0+): CSA, MinHash, Datalog

---

## 📚 Documentation

### Core Planning Documents

| Document | Purpose | Read When | Owner |
|----------|---------|-----------|-------|
| **[FUTURE_ENHANCEMENTS_PLAN.md](FUTURE_ENHANCEMENTS_PLAN.md)** | Detailed technical plan with code examples | Before implementing any feature | @team |
| **[ROADMAP.md](ROADMAP.md)** | GitHub project board view for tracking | For progress updates, assignment | @team |
| **[IMPLEMENTATION_SUMMARY_v0.2.1.md](IMPLEMENTATION_SUMMARY_v0.2.1.md)** | What was fixed in v0.2.1 | New contributor onboarding | @team |
| **[QUICK_REFERENCE_v0.2.1.md](QUICK_REFERENCE_v0.2.1.md)** | 5-minute overview | Quick context refresh | @team |

### Research Sources (Inform v0.4.0+)

| Document | Topic | Key Insight | Impact |
|----------|-------|-------------|--------|
| **[research/gensense-agent-integration.md](research/gensense-agent-integration.md)** | Multi-agent systems | GenSense as shared ground truth; need confidence + auto_fixable fields | F2a (v0.3.0-alpha) |
| **[research/gensense-algorithmic-grounding.md](research/gensense-algorithmic-grounding.md)** | Math foundations | MinHash for similarity, Datalog for rules | v0.4.0 research |
| **[research/gensense-future-direction.md](research/gensense-future-direction.md)** | LLM failure patterns | Contract Surface Analysis (CSA) for semantic coherence | v0.4.0+ research |

### Original Documentation

| Document | Purpose |
|----------|---------|
| **[gensense-fixes-and-future.md](gensense-fixes-and-future.md)** | Problems W1-W3 (now fixed in v0.2.1), Future directions F1-F6 |

---

## 🗺️ Roadmap Overview

### Phase 1: Quick Wins (v0.2.2) — 1-2 days
```
F5 (15 min)     Fix original_content → Unblocks patcher/--fix
     ↓
Combined with F4 (4 hours)    SARIF Output → GitHub PR annotations
     ↓
RELEASE v0.2.2
```

### Phase 2: Strategic Foundations (v0.3.0-alpha) — 1 sprint
```
F2a (2 hours)               Agent-Ready Advisory
     ↓
F2 (1-2 days)       Incremental Analysis (BLOCKER for F1)
     ↓
F1 (2-3 days)       LSP Server (depends on F2)
     ↓
RELEASE v0.3.0-alpha
```

### Phase 3: Add-ons (v0.3.0) — 2-4 days
```
F6 (4 hours)       Duplicate Detection    ⎤
     ↓              (parallel)            ⎥
F3 (1 day)         Temporal Rules         ⎦
     ↓
RELEASE v0.3.0
```

### Phase 4: Future Research (v0.4.0+)
```
CSA (Contract Surface Analysis)    LLM semantic coherence
MinHash Similarity                  Structural duplicates
Datalog Semantics                   Declarative rules
```

---

## 🎯 Feature Summary

### Quick Reference Table

| # | Feature | Tier | Ver | Effort | Impact | Status | Owner |
|---|---------|------|-----|--------|--------|--------|-------|
| **W1** | BFS visited set fix | ✅ Done | 0.2.1 | 30 min | CRITICAL | ✅ Complete | — |
| **W2** | JS API project rules | ✅ Done | 0.2.1 | 1 hr | CRITICAL | ✅ Complete | — |
| **W3** | E2E project tests | ✅ Done | 0.2.1 | 2 hrs | CRITICAL | ✅ Complete | — |
| **F5** | original_content fix | 1 | 0.2.2 | 15 min | HIGH | 📝 TODO | [Assign] |
| **F4** | SARIF output | 1 | 0.2.2 | 4 hrs | HIGH | 📝 TODO | [Assign] |
| **F2a** | Agent-Ready Advisory | 2 | 0.3.0a | 2 hrs | MEDIUM | 📝 BACKLOG | [Assign] |
| **F2** | Incremental Analysis | 2 | 0.3.0a | 1-2 d | **HIGH** | 📝 BACKLOG | [Assign] |
| **F1** | LSP Server | 2 | 0.3.0a | 2-3 d | **HIGH** | 📝 BACKLOG | [Assign] |
| **F6** | Duplicates | 3 | 0.3.0 | 4 hrs | MEDIUM | 📝 BACKLOG | [Assign] |
| **F3** | Temporal Rules | 3 | 0.3.0 | 1 day | MEDIUM | 📝 BACKLOG | [Assign] |

---

## 🔍 Implementation Guide

### For Contributors

**Step 1: Understand the Current State**
```bash
# Run validation to see v0.2.1 status
bash scripts/validate-v0.2.1-fixes.sh

# Read 5-min overview
cat QUICK_REFERENCE_v0.2.1.md
```

**Step 2: Pick a Feature**
- New? Start with **F5** (15 min) or **F4.1** (10 min)
- Want to learn more? Read [FUTURE_ENHANCEMENTS_PLAN.md](FUTURE_ENHANCEMENTS_PLAN.md)

**Step 3: Implement**
1. Create branch: `git checkout -b feature/f5-original-content`
2. Follow task breakdown in [FUTURE_ENHANCEMENTS_PLAN.md](FUTURE_ENHANCEMENTS_PLAN.md)
3. Write tests first (TDD)
4. Verify: `cargo test --all && npm test`
5. Submit PR with checklist completed

**Step 4: Update Progress**
- Mark task status in [ROADMAP.md](ROADMAP.md): 📌→🔄→✅
- Comment on this board when done
- Request review

---

## 📋 Dependency Graph

```
                    ┌─────────────┐
                    │ F5 (15 min) │ ◄─── Do this first (immediate)
                    └─────────────┘

                    ┌─────────────────────┐
                    │ F4 (4 hrs) + GHA    │ ◄─── Parallel with F5
                    │ RELEASE v0.2.2      │
                    └─────────────────────┘

    ┌──────────────┐
    │ F2a (2 hrs)  │ ◄─── Parallel group, can do in any order
    ├──────────────┤
    │ F6 (4 hrs)   │
    ├──────────────┤
    │ F3 (1 day)   │
    └──────────────┘
            │
            ▼
    ┌──────────────────┐
    │ F2 (1-2 days)    │ ◄─--- BLOCKER for F1
    │ Req'd for F1     │       (50-100ms analysis)
    └──────────────────┘
            │
            ▼
    ┌──────────────────────┐
    │ F1 (2-3 days)        │ ◄─--- LSP Server
    │ RELEASE v0.3.0-alpha │       (depends on F2)
    └──────────────────────┘
            │
            ▼
    RELEASE v0.3.0 (add polish)
            │
            ▼
    ┌────────────────────────┐
    │ v0.4.0+ Research       │
    │ • CSA (LLM semantics)  │
    │ • MinHash (similar)    │
    │ • Datalog (rules)      │
    └────────────────────────┘
```

---

## 🧪 Testing Strategy

### Validation Scripts
```bash
# Quick validation (5 min)
bash scripts/validate-v0.2.1-fixes.sh

# Full test suite (5 min)
cargo test --all

# Node.js integration
npm test

# After F2a (advisory struct)
cargo test --lib AdvisoryConfidence

# After F2 (incremental)
cargo bench --bench incremental

# After F1 (LSP)
cargo build --bin gensense-lsp --release
# Manual test with ncvim or VS Code
```

### Test Coverage by Feature
- **F5**: existing `correctness_tests.rs` should pass with populated original_content
- **F4**: new `tests/sarif_output.rs` with SARIF schema validation
- **F2a**: verify advisory JSON/SARIF include confidence and auto_fixable
- **F2**: benchmark shows optimal cache hits (5ms for no-change, 50ms for 1 change)
- **F1**: integration tests with mock LSP client
- **F6**: `test_finds_copy_pasted_functions` with real duplicates
- **F3**: temporal scope tests on async code samples

---

## 📊 Metrics & KPIs

| Metric | Target | How to Measure |
|--------|--------|---|
| LSP diagnostics latency | <250ms | `time engine.run(project/)` after F2 |
| Incremental speedup | 10x | `bench_no_change / bench_all_changes` |
| SARIF output validity | 100% | `sarif-validator output.sarif` |
| Test coverage | >90% | `cargo tarpaulin --out Html` |
| Agent readiness | 100% | Agents can parse `confidence` and `auto_fixable` |
| Copy-paste detection | 100% recall | Find all intentional duplicates in test corpus |

---

## 🚀 Release Schedule

| Version | Date | Features | Branch |
|---------|------|----------|--------|
| **v0.2.1** | 2026-05-14 | W1,W2,W3 fixes | `main` ✅ |
| **v0.2.2** | 2026-05-23 | F5, F4 | `release/0.2.2` |
| **v0.3.0-alpha** | 2026-06-13 | F2a, F2, F1 | `release/0.3.0-alpha` |
| **v0.3.0** | 2026-06-27 | F6, F3 | `release/0.3.0` |
| **v0.4.0** | Q3-Q4 2026 | CSA, MinHash, Datalog | TBD |

---

## 🤝 Getting Started Checklist

- [ ] Read [QUICK_REFERENCE_v0.2.1.md](QUICK_REFERENCE_v0.2.1.md) (5 min)
- [ ] Run `bash scripts/validate-v0.2.1-fixes.sh` to verify v0.2.1
- [ ] Read [FUTURE_ENHANCEMENTS_PLAN.md](FUTURE_ENHANCEMENTS_PLAN.md) intro (10 min)
- [ ] Pick a feature from Tier 1 (F5 or F4)
- [ ] Review code in FUTURE_ENHANCEMENTS_PLAN.md for your feature (15 min)
- [ ] Clone repo and create feature branch
- [ ] Begin implementation following task breakdown
- [ ] Submit PR with checklist
- [ ] Update this board when complete

---

## 📞 Questions?

### Technical
- **Implementation questions?** See [FUTURE_ENHANCEMENTS_PLAN.md](FUTURE_ENHANCEMENTS_PLAN.md) section for your feature
- **Code examples?** Each feature has "Code Preview" section with Rust stubs
- **Testing?** Check tests section in that feature

### Process
- **How to contribute?** See "Implementation Guide" above
- **How to track progress?** Update [ROADMAP.md](ROADMAP.md)
- **How to request features?** File issue with reference to v0.4.0 research

### Research
- **LLM failure patterns?** See `gensense-future-direction.md` (CSA)
- **Multi-agent coordination?** See `gensense-agent-integration.md`
- **Algorithms?** See `gensense-algorithmic-grounding.md`

---

## 📝 Document Change Log

| Date | Change | Author |
|------|--------|--------|
| 2026-05-14 | Initial master index + all planning docs | @team |
| TBD | Add progress notes after F5 starts | — |
| TBD | Update after v0.2.2 release | — |

---

**Master Index v1.0** | Last Updated: 2026-05-14  
**Maintained**: GenSense Team  
**Next Review**: Before F5 implementation starts
