# GenSense Stabilization Manifesto

**Version:** v1
**Phase:** Core Engine Stabilization
**Status:** ACTIVE

---

# Purpose

This document exists to prevent GenSense from becoming:

* an endlessly rewritten architecture project,
* an overgeneralized analysis framework,
* a research system without operational reliability,
* or a feature-heavy engine with weak correctness guarantees.

The current priority is no longer rapid feature expansion.

The current priority is:

> building a stable, measurable, trustworthy semantic analysis engine.

---

# Current Engineering Phase

GenSense is currently in:

## Stability Phase

This means:

* reliability matters more than novelty,
* determinism matters more than abstraction,
* tests matter more than features,
* benchmarks matter more than architecture discussions,
* operational confidence matters more than conceptual ambition.

---

# Primary Objective

The objective of this phase is:

> prove the core engine is reliable under real-world pressure.

This includes:

* semantic correctness,
* graph consistency,
* stable traversal behavior,
* parser resilience,
* deterministic output,
* low-noise analysis,
* measurable performance,
* predictable behavior on messy repositories.

---

# Priority Order (Strict)

The following order is mandatory.

## PRIORITY 1 — Engine Stability

Allowed work:

* bug fixing,
* graph consistency,
* traversal correctness,
* parser resilience,
* deterministic behavior,
* semantic validation,
* memory/performance profiling,
* reducing complexity,
* simplification.

---

## PRIORITY 2 — Testing Infrastructure

Required:

* unit tests,
* semantic graph tests,
* temporal/event ordering tests,
* parser edge-case tests,
* integration tests,
* regression tests,
* snapshot/baseline tests,
* invariant validation tests,
* crash reproduction tests.

Every discovered bug must create:

1. a reproducible case,
2. a regression test,
3. a documented root cause.

---

## PRIORITY 3 — Benchmarking

GenSense must be benchmarked against:

* real repositories,
* messy repositories,
* intentionally vulnerable repositories,
* AI-generated repositories,
* async-heavy systems,
* large codebases,
* malformed edge-case samples.

Track:

* runtime,
* memory usage,
* false positives,
* false negatives,
* parser failures,
* unsupported syntax,
* traversal bottlenecks,
* graph scaling behavior.

No optimization work should happen before benchmarks exist.

---

## PRIORITY 4 — Operational Confidence

The engine must explicitly know:

* what it supports,
* what it does not support,
* where semantics are unstable,
* where results are noisy,
* where assumptions exist,
* where analysis is incomplete.

A trusted tool understands its own limitations.

---

# Feature Freeze Rules

Until stabilization goals are met:

## DO NOT:

* redesign the architecture,
* rewrite core systems,
* add major semantic layers,
* invent new query systems,
* create new engines,
* generalize abstractions prematurely,
* add features without benchmarks,
* optimize unmeasured code,
* pursue hypothetical scalability.

---

# Feature Admission Rules

A new feature is ONLY allowed if all questions are answered.

## Mandatory Questions

1. What measurable limitation does this solve?
2. Is the limitation benchmarked?
3. Is the limitation user-visible?
4. Is the current engine stable enough?
5. Can this be solved with less complexity?
6. What tests will validate this feature?
7. What invariants could this break?
8. Does this increase maintenance cost?
9. Does this improve trustworthiness?
10. Does this introduce architectural drift?

If these questions cannot be answered clearly:

> the feature is postponed.

---

# Mandatory Engineering Habits

## Rule 1 — Every Bug Creates a Test

Never fix a bug without:

* a reproducible case,
* a regression test,
* verification that the bug cannot silently reappear.

---

## Rule 2 — Measure Before Rewriting

Never rewrite systems based on intuition alone.

Required first:

* benchmarks,
* profiling,
* failure evidence,
* scaling data,
* reproducible bottlenecks.

---

## Rule 3 — Simplify Aggressively

For every abstraction ask:

* can this be smaller?
* can this be removed?
* can this become data-driven?
* can this become deterministic?
* can this become easier to test?

Complexity must justify itself.

---

## Rule 4 — Stability Before Capability

A stable limited engine is more valuable than:

* an unstable advanced engine.

Trust is more important than feature count.

---

## Rule 5 — Avoid Architectural Curiosity Drift

Do not pursue:

* C rewrites,
* C++ rewrites,
* custom runtimes,
* custom IR systems,
* universal query engines,
* distributed analysis systems,
* infrastructure expansion,

unless benchmarks prove necessity.

Architectural curiosity is not engineering necessity.

---

# Known Weaknesses Log (Required)

Maintain a continuously updated document containing:

* parser limitations,
* semantic ambiguity,
* unsupported syntax,
* graph assumptions,
* unstable event ordering,
* performance collapse points,
* noisy detections,
* false positive patterns,
* incomplete analysis paths.

This document is mandatory.

---

# Weekly Discipline Cycle

## Monday–Thursday

Allowed:

* stabilization,
* tests,
* benchmarks,
* profiling,
* bug fixing,
* edge-case handling.

---

## Friday

Review:

* benchmark changes,
* regression failures,
* performance trends,
* false positives,
* crash reports,
* architectural drift.

---

## Saturday

Allowed:

* controlled experimentation,
* research,
* future ideas,
* architecture exploration.

No production rewrites.

---

## Sunday

Document:

* lessons learned,
* discovered limitations,
* unstable assumptions,
* future risks,
* simplifications.

---

# Engineering Philosophy

GenSense should not attempt to become:

* a universal programming platform,
* a research operating system,
* or an infinitely generalized analysis framework.

GenSense should become:

> a reliable semantic analysis engine with strong operational trust.

---

# Final Constraint

Before any major engineering decision ask:

> “Is this solving a measured problem or satisfying architectural curiosity?”

If the answer is architectural curiosity:

STOP.

Return to:

* tests,
* benchmarks,
* profiling,
* stabilization,
* simplification,
* operational confidence.

---

# End State Goal

The goal is not:

* maximum complexity,
* maximum abstraction,
* maximum architecture.

The goal is:

> a stable engine that developers trust.
