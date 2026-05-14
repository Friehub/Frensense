# GenSense Roadmap — GitHub Project Board

**Status**: Active Planning | **Created**: 2026-05-14 | **Last Updated**: 2026-05-14

This is a GitHub-style project board view of the future enhancements plan from [FUTURE_ENHANCEMENTS_PLAN.md](FUTURE_ENHANCEMENTS_PLAN.md).

---

## 📋 Board Overview

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│ BACKLOG                   │  TODO (v0.2.2)          │  IN PROGRESS  │  DONE     │
├─────────────────────────────────────────────────────────────────────────────────┤
│ F3: Temporal Rules        │ F5: Fix original_content │              │ W1,W2,W3  │
│ F6: Duplicates            │ F4: SARIF Output        │              │ v0.2.1    │
│ F2a: Agent-Ready Advisory │                         │              │           │
│ F2: Incremental Analysis  │                         │              │           │
│ F1: LSP Server            │                         │              │           │
│ CSA: Contract Analysis    │                         │              │           │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## 🔵 TIER 1: QUICK WINS (v0.2.2)

### Task: F5 — Fix `original_content` Gap
- **Status**:  BACKLOG →  TODO
- **Priority**:  CRITICAL
- **Effort**: 15 min
- **Assigned**: [Unassigned]
- **Labels**: `bug`, `quick-fix`, `patcher`
- **Related Issues**: Patcher broken on project rules
- **Checklist**:
  - [ ] Remove String::new() placeholder in src/rules/ir.rs:384
  - [ ] Test with correctness tests
  - [ ] Verify --fix mode works
  - [ ] Update CLI docs
  - [ ] PR reviewed & merged
- **PR**: [To be created]

### Task: F4 — SARIF Output Format
- **Status**:  BACKLOG →  TODO
- **Priority**:  HIGH
- **Effort**: 4 hours
- **Assigned**: [Unassigned]
- **Labels**: `feature`, `integration`, `github-actions`, `output-format`
- **Related Issues**: GitHub PR annotations
- **Breakdown**:
  - [ ] F4.1 Add SARIF dependencies (10 min)
  - [ ] F4.2 Add CLI flag (15 min)
  - [ ] F4.3 Implement converter (90 min)
  - [ ] F4.4 Update reporter (30 min)
  - [ ] F4.5 Add tests (30 min)
  - [ ] F4.6 GHA integration example (15 min)
- **Est. Start**: Week 1, Mon
- **Est. Complete**: Week 1, Tue
- **PR**: [To be created]

---

##  TIER 2: STRATEGIC FOUNDATIONS (v0.3.0-alpha)

### Task: F2a — Agent-Ready Advisory Struct
- **Status**:  BACKLOG
- **Priority**:  MEDIUM
- **Effort**: 2 hours
- **Assigned**: [Unassigned]
- **Labels**: `feature`, `api`, `agents`, `research`
- **Related Issues**: MultiAgent system coordination
- **Research Source**: `gensense-agent-integration.md`
- **Breakdown**:
  - [ ] F2a.1 Extend Advisory struct (15 min)
  - [ ] F2a.2 Implement confidence scoring (45 min)
  - [ ] F2a.3 Add auto_fixable flag (30 min)
  - [ ] F2a.4 JSON/SARIF serialization (30 min)
- **Depends On**: F4 (SARIF output framework)
- **Blocks**: F1 (LSP can use confidence scores)
- **Est. Start**: Week 3, after F1-F2 foundation
- **PR**: [To be created]

### Task: F2 — Incremental Analysis
- **Status**:  BACKLOG
- **Priority**:  HIGH (BLOCKER for F1)
- **Effort**: 1-2 days
- **Assigned**: [Unassigned]
- **Labels**: `feature`, `performance`, `infrastructure`, `lsp-prerequisite`
- **Related Issues**: LSP sub-100ms diagnostics
- **Breakdown**:
  - [ ] F2.1 Add content hash tracking (30 min)
  - [ ] F2.2 Build reverse-dependency index (1 hour)
  - [ ] F2.3 Implement cache layer (1.5 hours)
  - [ ] F2.4 Integrate into Engine (1 hour)
  - [ ] F2.5 Add benchmarks (30 min)
  - [ ] F2.6 Add tests (30 min)
- **Blocks**: F1 (LSP depends on this)
- **Est. Start**: Week 3, Mon
- **Est. Complete**: Week 3, Tue-Wed
- **Performance Target**: <100ms for 10k LOC project
- **PR**: [To be created]

### Task: F1 — LSP Server
- **Status**:  BACKLOG
- **Priority**:  HIGH
- **Effort**: 2-3 days
- **Assigned**: [Unassigned]
- **Labels**: `feature`, `editor-integration`, `lsp`, `vs-code`
- **Related Issues**: Editor integration (VS Code, Neovim, Helix)
- **Blocking Issues**: None (standalone after F2)
- **Depends On**: F2 (Incremental Analysis)
- **Breakdown**:
  - [ ] F1.1 Add dependencies (5 min)
  - [ ] F1.2 Create LSP service struct (1 hour)
  - [ ] F1.3 Implement diagnostics handler (1 hour)
  - [ ] F1.4 Implement code action handler (1 hour)
  - [ ] F1.5 Create LSP entrypoint (30 min)
  - [ ] F1.6 VS Code extension scaffold (1.5 hours)
  - [ ] F1.7 Integration tests (1 hour)
  - [ ] F1.8 Documentation (30 min)
- **Est. Start**: Week 4, Mon (after F2 complete)
- **Est. Complete**: Week 4, Thu
- **PR**: [To be created]

---

##  TIER 3: HIGH-VALUE ADD-ONS (v0.3.0+)

### Task: F6 — Fingerprint Duplicates
- **Status**:  BACKLOG
- **Priority**:  MEDIUM
- **Effort**: 4 hours
- **Assigned**: [Unassigned]
- **Labels**: `feature`, `code-quality`, `copy-paste-detection`
- **Related Issues**: Copy-paste code detection
- **Breakdown**:
  - [ ] F6.1 Add CLI flag (15 min)
  - [ ] F6.2 Extend Engine method (1 hour)
  - [ ] F6.3 Output formatting (1 hour)
  - [ ] F6.4 Integration (1 hour)
  - [ ] F6.5 Tests & docs (1.5 hours)
- **Est. Start**: Week 4
- **Par allel**: Can run alongside F3
- **PR**: [To be created]

### Task: F3 — Richer Temporal Rules
- **Status**:  BACKLOG
- **Priority**:  MEDIUM
- **Effort**: 1 day
- **Assigned**: [Unassigned]
- **Labels**: `feature`, `temporal-analysis`, `async-safety`
- **Related Issues**: Async deadlock detection, lock safety
- **Breakdown**:
  - [ ] F3.1 Extend temporal rule DSL (1 hour)
  - [ ] F3.2 Implement scope analysis (2 hours)
  - [ ] F3.3 Add test rules (30 min)
  - [ ] F3.4 Test cases (1.5 hours)
  - [ ] F3.5 Documentation (30 min)
- **Est. Start**: Week 4
- **Parallel**: Can run alongside F6
- **Example Rules**: RUST_LOCK_ACROSS_AWAIT
- **PR**: [To be created]

---

## 🔮 FUTURE RESEARCH (v0.4.0+)

### Initiative: Contract Surface Analysis (CSA)
- **Status**:  RESEARCH
- **Priority**:  MEDIUM (future strategic)
- **Target Version**: v0.4.0
- **Effort**: 2-3 days (research + impl)
- **Source**: `gensense-future-direction.md`
- **Problem**: LLM-generated code has systematic failures (incoherence between name and implementation)
- **Examples**:
  - `validate_*()` returns true unconditionally
  - `find_*()` never returns None
  - `sanitize_*()` returns input unmodified
- **Approach**: Name-based contract checking on AST
- **Est. Target**: Q3 2026

### Initiative: Algorithmic Enhancements
- **Status**:  RESEARCH
- **Priority**:  MEDIUM (future scope)
- **Target Version**: v0.4.0+
- **Source**: `gensense-algorithmic-grounding.md`

#### F6+ — MinHash Similarity Scoring
- **Problem**: Current fingerprints detect exact duplicates, not near-duplicates
- **Solution**: Locality-Sensitive Hashing for Jaccard similarity
- **Use Case**: Detect LLM-generated variants (70-85% similar)
- **Prerequisite**: F6 (Duplicate detection foundation)
- **Est. Target**: Q4 2026

#### F7 — Datalog Semantics for Project Rules
- **Problem**: Hand-written BFS in ProjectRule is fragile and hard to extend
- **Solution**: Declare rules in Datalog-like logic, compute fixed points
- **Example**: "Variable x flows from S to T through ≤3 hops"
- **Prerequisite**: F2 (Incremental analysis for perf)
- **Est. Target**: Q4 2026+

---

## 📊 Timeline & Milestones

```
Week 1 (May 19-23)
  Mon-Tue: F5 (15 min)
  Mon-Tue: F4 (4 hours) ✨ RELEASE v0.2.2
  
Week 2-3 (May 26-Jun 8)
  Strategic planning & code review
  
Week 3-4 (Jun 2-13)
  Mon-Wed: F2 (Incremental Analysis) 1-2 days
  Thu-Fri: Begin F1 scaffolding
  
Week 4-5 (Jun 9-23)
  Mon-Thu: F1 (LSP Server) 2-3 days
  Fri: F1 testing & docs ✨ RELEASE v0.3.0-alpha
  
Week 5-6 (Jun 23-Jul 7)
  Mon: F6 (Duplicates) 4 hours
  Mon-Tue: F3 (Temporal Rules) 1 day
  Wed: Polish & documentation ✨ RELEASE v0.3.0

Q3-Q4 2026
  📚 CSA & Algorithmic research
  🔮 Future directions
```

---

## 🎯 Success Criteria by Release

### v0.2.2 (Late May)
- [ ] F5: original_content populated (100%)
- [ ] F4: SARIF output passes validator
- [ ] All existing tests pass (no regressions)
- [ ] GitHub Actions integration example in repo

### v0.3.0-alpha (Mid June)
- [ ] F2a: Advisory struct enhanced, agents can parse confidence
- [ ] F2: Incremental analysis <100ms on 10k LOC
- [ ] F1: LSP works in VS Code and Neovim
- [ ] Code actions apply fixes correctly

### v0.3.0 (End June)
- [ ] F6: Duplicate detection finds copy-paste code
- [ ] F3: Temporal rules with scope constraints
- [ ] Full test coverage on new features
- [ ] Comprehensive documentation & examples

---

## 🏃 How to Help

### New to GenSense? Start Here
1. Read [QUICK_REFERENCE_v0.2.1.md](QUICK_REFERENCE_v0.2.1.md) for context
2. Review [FUTURE_ENHANCEMENTS_PLAN.md](FUTURE_ENHANCEMENTS_PLAN.md) for details
3. Pick a Tier 1 task: F5 (15 min) or F4.1 (10 min)
4. Submit PR with reference to this board

### Want to Contribute?
1. Pick a task from TODO column
2. Comment here to claim: "Taking F5"
3. Create branch: `feature/f5-original-content`
4. Reference this board in PR description
5. Update task status as you work

### Want to Review?
1. Watch this board for PRs
2. Review against task checklist
3. Test locally with validation script
4. Approve & merge when ready

---

## 📖 Documentation Map

| Document | Purpose | Audience |
|----------|---------|----------|
| [IMPLEMENTATION_SUMMARY_v0.2.1.md](IMPLEMENTATION_SUMMARY_v0.2.1.md) | Current state (v0.2.1) | Everyone |
| [QUICK_REFERENCE_v0.2.1.md](QUICK_REFERENCE_v0.2.1.md) | 5-min overview of fixes | New contributors |
| [FUTURE_ENHANCEMENTS_PLAN.md](FUTURE_ENHANCEMENTS_PLAN.md) | Detailed roadmap with code examples | Implementers |
| [ROADMAP.md](ROADMAP.md) ← You are here | GitHub project board view | Project management |
| [gensense-fixes-and-future.md](gensense-fixes-and-future.md) | Original problems & context | Architecture review |
| [research/gensense-agent-integration.md](research/gensense-agent-integration.md) | Multi-agent systems | Research, v0.4+ |
| [research/gensense-algorithmic-grounding.md](research/gensense-algorithmic-grounding.md) | Mathematical foundations | Research, algorithms |
| [research/gensense-future-direction.md](research/gensense-future-direction.md) | LLM-generated code patterns (CSA) | Research, v0.4+ |

---

## 🔗 Quick Links

- **Main Plan**: [FUTURE_ENHANCEMENTS_PLAN.md](FUTURE_ENHANCEMENTS_PLAN.md)
- **Current Release**: [IMPLEMENTATION_SUMMARY_v0.2.1.md](IMPLEMENTATION_SUMMARY_v0.2.1.md)
- **Quick Ref**: [QUICK_REFERENCE_v0.2.1.md](QUICK_REFERENCE_v0.2.1.md)
- **Validation**: `bash scripts/validate-v0.2.1-fixes.sh`
- **Repository**: https://github.com/friehub/gensense
- **Issues**: [GitHub Issues](https://github.com/friehub/gensense/issues)

---

**Board Version**: 1.0 | **Updated**: 2026-05-14 | **Maintained By**: GenSense Team
