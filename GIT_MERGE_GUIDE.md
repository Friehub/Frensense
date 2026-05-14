# GenSense Planning Docs — Git Organization

**Status**: Ready to Merge | **Created**: 2026-05-14

This document clarifies which planning documents should be committed to git, which should stay ignored, and the merge workflow.

---

## 📋 Quick Summary

**COMMIT to git** (stay in repo):
- ✅ All `.md` planning documents in root
- ✅ `scripts/validate-v0.2.1-fixes.sh`
- ✅ Modified `gensense-fixes-and-future.md`

**IGNORE (do not commit)**:
- ❌ `target/` (build artifacts)
- ❌ `node_modules/` (npm packages)
- ❌ `.DS_Store`, `*.swp` (OS/editor files)
- ❌ `*.log` (build logs)
- ❌ Local test outputs

---

## 📂 New Files to Commit

All in **root** directory:

```
/
├── PLANNING_SUMMARY.md                    ← COMMIT (navigation hub)
├── PLANNING_MASTER_INDEX.md               ← COMMIT (master index)
├── FUTURE_ENHANCEMENTS_PLAN.md            ← COMMIT (detailed roadmap)
├── ROADMAP.md                             ← COMMIT (github board)
├── IMPLEMENTATION_SUMMARY_v0.2.1.md       ← COMMIT (v0.2.1 summary)
├── QUICK_REFERENCE_v0.2.1.md              ← COMMIT (5-min overview)
├── gensense-fixes-and-future.md           ← COMMIT (updated with status)
│
└── scripts/
    └── validate-v0.2.1-fixes.sh           ← COMMIT (validation script)
```

**Existing directories** (no changes):
```
├── research/
│   ├── gensense-agent-integration.md          (unchanged, for reference)
│   ├── gensense-algorithmic-grounding.md      (unchanged, for reference)
│   └── gensense-future-direction.md           (unchanged, for reference)
│
├── docs/
│   ├── api.md                                 (unchanged)
│   ├── editor.md                              (unchanged)
│   └── ...
│
├── src/                                       (unchanged)
├── tests/                                     (unchanged, fixes already there)
└── (other folders)                           (unchanged)
```

---

## 🔒 Current .gitignore (Verify)

Your `.gensense/.gitignore` or root `.gitignore` should have:

```gitignore
# Rust build artifacts
/target/
/Cargo.lock
*.rlib
*.rmeta

# Node/npm
node_modules/
package-lock.json
*.tgz
dist/

# IDE & Editor
.vscode/
.idea/
*.swp
*.swo
*~
.DS_Store
Thumbs.db

# Logs
*.log
npm-debug.log*

# OS
.DS_Store
.env
.env.local

# Test artifacts (if any)
output.sarif
*.json (if generated)
```

**No changes needed to `.gitignore`** — planning docs don't conflict with it.

---

## ✅ Git Workflow for Merge

### Step 1: Create Feature Branch
```bash
git checkout -b docs/planning-complete-v0.2.1
```

### Step 2: Add New Files
```bash
# Add all new planning docs
git add PLANNING_SUMMARY.md
git add PLANNING_MASTER_INDEX.md
git add FUTURE_ENHANCEMENTS_PLAN.md
git add ROADMAP.md
git add IMPLEMENTATION_SUMMARY_v0.2.1.md
git add QUICK_REFERENCE_v0.2.1.md
git add scripts/validate-v0.2.1-fixes.sh

# Add updated file
git add gensense-fixes-and-future.md
```

Or use wildcard (clean):
```bash
git add *.md PLANNING_*.md scripts/validate-v0.2.1-fixes.sh gensense-fixes-and-future.md
```

### Step 3: Verify What's Staged
```bash
git status

# Should show (no build artifacts, no node_modules):
On branch docs/planning-complete-v0.2.1

Changes to be committed:
  new file:   PLANNING_SUMMARY.md
  new file:   PLANNING_MASTER_INDEX.md
  new file:   FUTURE_ENHANCEMENTS_PLAN.md
  new file:   ROADMAP.md
  new file:   IMPLEMENTATION_SUMMARY_v0.2.1.md
  new file:   QUICK_REFERENCE_v0.2.1.md
  new file:   scripts/validate-v0.2.1-fixes.sh
  modified:   gensense-fixes-and-future.md
```

### Step 4: Commit
```bash
git commit -m "docs: Add comprehensive v0.3.0 planning suite

- PLANNING_SUMMARY.md: Navigation hub for all planning docs
- PLANNING_MASTER_INDEX.md: Master index with feature table
- FUTURE_ENHANCEMENTS_PLAN.md: Detailed roadmap (6 features, 3 tiers)
- ROADMAP.md: GitHub project board view with tracking
- IMPLEMENTATION_SUMMARY_v0.2.1.md: What was fixed in v0.2.1
- QUICK_REFERENCE_v0.2.1.md: 5-minute overview
- scripts/validate-v0.2.1-fixes.sh: Validation script
- gensense-fixes-and-future.md: Updated with implementation status

Integrated research from:
- gensense-agent-integration.md → F2a enhancements
- gensense-algorithmic-grounding.md → v0.4.0 research
- gensense-future-direction.md → CSA direction

All features have acceptance criteria, task breakdowns, code previews, and test plans.
Ready for implementation starting Week 1."
```

### Step 5: Push & Create PR
```bash
git push origin docs/planning-complete-v0.2.1

# Then create PR on GitHub with description:
# "Complete Planning Suite for v0.3.0"
# (Use template below)
```

---

## 📝 PR Description Template

Use this for your GitHub PR:

```markdown
## Complete Planning Suite for GenSense v0.3.0

### Overview
Comprehensive planning suite for future enhancements, incorporating research 
insights and detailed implementation roadmap.

### What's Included

#### Planning Documents (8 files)
- **PLANNING_SUMMARY.md** - Navigation hub; start here
- **PLANNING_MASTER_INDEX.md** - Master index with feature table
- **FUTURE_ENHANCEMENTS_PLAN.md** - Detailed breakdown (850+ lines)
- **ROADMAP.md** - GitHub project board style tracking
- **IMPLEMENTATION_SUMMARY_v0.2.1.md** - Current release summary
- **QUICK_REFERENCE_v0.2.1.md** - 5-minute overview
- **scripts/validate-v0.2.1-fixes.sh** - Validation script
- **gensense-fixes-and-future.md** - Updated status (v0.2.1 complete)

#### Features (6)
- **Tier 1 (Quick wins, v0.2.2)**: F5 (15 min), F4 (4 hrs)
- **Tier 2 (Strategic, v0.3.0-alpha)**: F2a (2 hrs), F2 (1-2 d), F1 (2-3 d)
- **Tier 3 (Add-ons, v0.3.0)**: F6 (4 hrs), F3 (1 day)

#### Research Integration
- gensense-agent-integration.md → F2a (agent-ready advisory)
- gensense-algorithmic-grounding.md → v0.4.0 (MinHash, Datalog)
- gensense-future-direction.md → v0.4.0+ (CSA, semantic analysis)

### Ready to Execute
- [x] All features have acceptance criteria
- [x] All features have task breakdowns with time estimates
- [x] All features have code previews/skeletons
- [x] All features have test plans
- [x] Validation script working (v0.2.1)
- [x] Dependencies mapped and documented
- [x] Timeline and release schedule included

### How to Use
1. Start: Read PLANNING_SUMMARY.md (5 min)
2. Implement: Pick a feature from FUTURE_ENHANCEMENTS_PLAN.md
3. Track: Update status in ROADMAP.md
4. Release: Merge to main and tag v0.3.0 after all tiers complete

### Files Changed
- 8 new markdown files
- 1 new shell script (validation)
- 1 existing file updated (gensense-fixes-and-future.md)
- 0 source code files changed

### Validation
```bash
bash scripts/validate-v0.2.1-fixes.sh
# Output: ✅ All v0.2.1 fixes validated!
```

### Reviewers
- @team-leads: Verify scope alignment
- @tech-leads: Review roadmap timeline
- @anyone: Can start using PLANNING_SUMMARY.md immediately
```

---

## 📊 File Size Reference

**Know what's being committed:**

```bash
# Run this to see final sizes:
wc -l *.md scripts/*.sh | tail -1

# Expected output (~3500 lines total):
    850 FUTURE_ENHANCEMENTS_PLAN.md
    300 ROADMAP.md
    300 PLANNING_MASTER_INDEX.md
    300 PLANNING_SUMMARY.md
    220 IMPLEMENTATION_SUMMARY_v0.2.1.md
    120 QUICK_REFERENCE_v0.2.1.md
     60 gensense-fixes-and-future.md (additions)
     50 scripts/validate-v0.2.1-fixes.sh
   ----
   3500 total (docs only, no binary size)
```

**Not huge, very readable, all text.**

---

## ✨ After Merge Checklist

Once merged to `main`:

- [ ] Tag commit: `git tag -a v0.3.0-planning -m "Complete planning suite"`
- [ ] Update README.md to link to PLANNING_SUMMARY.md
- [ ] Announce in team: Planning docs are ready, implementation can start Week 1
- [ ] Create GitHub Project Board from ROADMAP.md tasks
- [ ] Pin PLANNING_MASTER_INDEX.md in repo root discussion
- [ ] Verify validation script in CI/CD

---

## 🚀 What Gets Ignored (Stays Local)

```
/target/
node_modules/
dist/
build/
*.log
.DS_Store
.vscode/
.idea/
*.swp
.env
output.sarif
test-output.json
```

**These never touch git** — they're build artifacts, editor configs, OS files.

---

## 📋 Merge Decision Checklist

Before merging to main, verify:

- [ ] All 8 planning documents created ✅
- [ ] Validation script working ✅
- [ ] No build artifacts staged (not in target/, node_modules/) ✅
- [ ] No .gitignore changes needed ✅
- [ ] Branch name clear: `docs/planning-complete-v0.2.1`
- [ ] PR description uses template above ✅
- [ ] At least 1 team lead approval
- [ ] CI/CD passes (should be green for docs)
- [ ] Ready to start Week 1 implementation

**If all ✅, merge to main.**

---

## 🎯 Post-Merge Steps

1. **Archive planning session**: Store these docs in shared drive for reference
2. **Create implementation tasks**: Use ROADMAP.md to file GitHub Issues
3. **Assign Tier 1 (F5, F4)**: Assign to team members Week 1
4. **Schedule kickoff**: Team sync to review PLANNING_SUMMARY.md
5. **Start F5**: 15-minute fix (first week)

---

## 📞 Questions Before Merge?

**Q: Are we committing too much documentation?**  
A: No. These are strategic docs that stay in the repo forever. They guide future work. Standard practice in mature projects.

**Q: Will .gitignore need changes?**  
A: No. These are plain `.md` files (text), not build artifacts. .gitignore already covers everything that should be ignored.

**Q: Should we compress these into one document?**  
A: No. Multiple docs allow different audiences to start at different places (5-min summary vs detailed plan).

**Q: Can we commit research/ docs too?**  
A: They're already in the repo. We just referenced them in the planning docs.

---

## ✅ Final Checklist

```bash
# Step 1: Verify new files exist
ls -1 PLANNING_*.md
ls -1 ROADMAP.md
ls -1 IMPLEMENTATION_SUMMARY_v0.2.1.md
ls -1 QUICK_REFERENCE_v0.2.1.md
ls -1 scripts/validate-v0.2.1-fixes.sh

# Step 2: Verify no build artifacts
ls target/ 2>/dev/null && echo "❌ STOP: target/ exists" || echo "✅ No target/"
ls node_modules/ 2>/dev/null && echo "❌ STOP: node_modules exists" || echo "✅ No node_modules/"

# Step 3: Verify git status
git status

# Step 4: Run validation
bash scripts/validate-v0.2.1-fixes.sh

# Step 5: Stage and commit
git add PLANNING_*.md IMPLEMENTATION_SUMMARY_v0.2.1.md QUICK_REFERENCE_v0.2.1.md ROADMAP.md scripts/validate-v0.2.1-fixes.sh gensense-fixes-and-future.md
git commit -m "docs: Add comprehensive v0.3.0 planning suite"
git push origin docs/planning-complete-v0.2.1

# Step 6: Go to GitHub, create PR
# Use template above, request review
```

---

**Ready to commit?** Run the checklist above and create the PR!

**Document Version**: 1.0 | **Created**: 2026-05-14
