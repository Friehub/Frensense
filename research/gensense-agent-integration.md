# GenSense in the Agent Era
## From Linter to Shared Ground Truth

---

## The Core Problem

When GenSense runs as a VSCode extension today, warnings appear as squiggles and Problems 
panel entries. The **human** sees them. The **LLM that generated the code does not** — 
unless the human copies the error back into the prompt manually.

With Copilot, Cursor, or any inline AI today:

```
LLM generates code → GenSense fires → Human sees warning → Human may or may not act
                                                         ↑
                                              LLM never knows it violated anything
```

The LLM that wrote the violation is already gone. The next generation starts fresh.
This breaks the feedback loop at the most critical point.

---

## What Changes With Multiple AI Agents

A developer running multiple agents — one writing code, one reviewing, one running tests, 
one managing the PR — has a fundamentally different architecture. GenSense in that world 
is not a linter the human reads. It is a **signal in the agent feedback loop**.

The agent writing code calls GenSense as a tool. The tool returns advisories. The agent 
either fixes the code or explains why the advisory does not apply. This is already how 
agent loops work with compiler errors and test output — they read structured output, 
they iterate. GenSense advisories in JSON and SARIF are already machine-readable. 
The infrastructure exists. What is missing is the contract that makes GenSense a 
first-class tool an agent can call, interpret, and act on.

---

## The Multi-Agent Topology

```
┌─────────────────────────────────────────────────────────────────┐
│                        Developer                                │
│                  (only sees requires_human)                     │
└──────────────────────────┬──────────────────────────────────────┘
                           │ architectural judgment only
                           │
          ┌────────────────▼────────────────┐
          │         Reviewer Agent          │
          │  reads SARIF, decides:          │
          │  fix / escalate / reject        │
          └────────┬───────────────┬────────┘
                   │               │
      ┌────────────▼───┐   ┌───────▼────────────┐
      │  Writer Agent  │   │    Fixer Agent      │
      │  generates     │   │  applies            │
      │  code          │   │  auto_fixable       │
      └────────┬───────┘   │  proposed_replace   │
               │           └────────────────────┘
               │
      ┌────────▼───────────────────────────────┐
      │              GenSense                  │
      │         Shared Ground Truth            │
      │                                        │
      │  • runs on every file save/tool call   │
      │  • returns structured advisories       │
      │  • enforces global call graph rules    │
      │  • flags what requires human judgment  │
      └────────────────────────────────────────┘
```

GenSense is the contract all agents agree on. The writer cannot ship code the reviewer 
will reject because they are both reading the same invariants. The fixer knows exactly 
what to change because the advisory contains the replacement. The human only appears 
for global invariants that require architectural judgment.

---

## What the Advisory Needs to Be Agent-Ready

The current Advisory struct is almost right. It has `rule_id`, `observation`, `impact`, 
`improvement`, `line`, `proposed_replacement`. An agent needs three more fields:

```rust
pub struct Advisory {
    // --- existing ---
    pub rule_id: String,
    pub file_path: String,
    pub severity: Severity,
    pub observation: String,
    pub impact: String,
    pub improvement: String,
    pub line: u32,
    pub proposed_replacement: Option<String>,

    // --- agent-ready additions ---
    pub confidence: f32,       // 0.0–1.0, derived from rule type
                               // taint findings: 0.9+, heuristic rules: 0.6–0.8
    pub auto_fixable: bool,    // safe for agent to apply proposed_replacement without review
    pub requires_human: bool,  // needs architectural judgment, surface to developer only
}
```

**`confidence`** tells the agent whether to fix immediately or reason first. 
A taint-reaches-sink finding at 0.95 confidence gets fixed. A heuristic similarity 
finding at 0.65 gets reasoned about.

**`auto_fixable`** tells the fixer agent whether it can apply `proposed_replacement` 
directly. Simple local fixes — replacing `.unwrap()` with `?`, adding an `await`, 
removing a dead result discard — are auto-fixable. Cross-file architectural violations 
are not.

**`requires_human`** is the escalation gate. Global invariant violations — a payment 
call path missing a validation node, a cross-file taint leak, a systemic contract 
surface failure across 20% of validators — get surfaced to the developer. Everything 
else stays in the agent loop.

---

## The Agent Loop

With these three fields, the agent workflow becomes deterministic:

```
1. Agent generates code
      ↓
2. GenSense runs (on save or explicit tool call)
      ↓
3. For each advisory where auto_fixable = true:
      → Fixer agent applies proposed_replacement
      → Re-run GenSense to confirm resolved
      ↓
4. For each advisory where auto_fixable = false AND requires_human = false:
      → Reviewer agent reasons about the finding
      → Writer agent attempts a fix
      → Re-run GenSense
      → Repeat up to N attempts
      ↓
5. For each advisory where requires_human = true:
      → Surface to developer with full context
      → Block merge until resolved
      ↓
6. When GenSense returns empty → code is considered correct
```

The writer cannot ignore a tool return value the way it ignores a squiggle 
a human forgot to mention. The loop does not exit until the advisories are resolved.

---

## The Interface That Makes This Real

GenSense today outputs JSON to stdout. That is correct for CI. For agents it needs 
to be callable as a tool with a defined schema — either as an MCP server or a 
structured tool definition.

**As an MCP tool:**

```json
{
  "name": "gensense_audit",
  "description": "Run semantic analysis on a path. Returns advisories the agent must resolve before the code is considered correct. An empty result means the code satisfies all invariants.",
  "parameters": {
    "path": {
      "type": "string",
      "description": "File or directory to audit"
    },
    "fix_auto": {
      "type": "boolean", 
      "description": "Automatically apply proposed_replacement for auto_fixable advisories"
    },
    "severity_threshold": {
      "type": "string",
      "enum": ["critical", "warning", "info"]
    }
  },
  "returns": {
    "advisories": "Advisory[]",
    "auto_fixed": "number",
    "requires_human": "Advisory[]",
    "clean": "boolean"
  }
}
```

This single interface change — from CLI to callable tool — is what makes GenSense 
visible to every LLM writing code in any editor or agent framework. The model 
does not see a squiggle. It sees a tool return value. And it cannot move forward 
while `clean: false`.

---

## Why Global Invariants Cannot Be Bypassed by Agents

Individual rules can be satisfied locally. An LLM generating a `validate_user_input` 
function can produce a body that passes taint entropy checks, has a branch, and touches 
its inputs. The local rule fires clean.

What cannot be satisfied locally is a **global invariant over the project call graph**. 
An agent writing one function at a time cannot reason about whether its output, composed 
with fifty other agent-generated functions across twelve files, violates a cross-file 
path property.

A rule that says:

> "Every call path from `process_payment` to `db.execute` must pass through a node 
> with taint-branch-ratio above 0.6"

...cannot be satisfied by generating one convincing function. It requires the entire 
call graph to be coherent. No agent generating code file-by-file can guarantee that 
without running GenSense after every change and iterating until the global invariant holds.

This is why `requires_human: true` is reserved specifically for these. The Datalog 
fixed-point layer, the cross-file taint checks, the path guards — these are the 
findings that require a human architect to define the invariant once. After that, 
agents enforce it forever.

---

## The Role Split

| Who | What They See | What They Do |
|---|---|---|
| **Writer Agent** | GenSense tool return on each generation | Iterates until clean or escalates |
| **Fixer Agent** | `auto_fixable` advisories with `proposed_replacement` | Applies fixes, re-runs GenSense |
| **Reviewer Agent** | Full advisory set with confidence scores | Routes: fix / attempt / escalate |
| **Developer** | Only `requires_human` advisories | Defines global invariants, reviews architectural violations |
| **CI/CD** | SARIF output, `clean` boolean | Blocks merge if not clean |

---

## What This Means for Code Quality at Scale

The feedback loop closes at the right place. Today:

```
LLM generates → human reviews → human fixes → merge
              (GenSense warns but LLM never iterates)
```

With GenSense as a first-class agent tool:

```
Agent generates → GenSense returns → agent iterates → GenSense confirms → merge
                  (LLM sees every violation, cannot exit loop until resolved)
```

The cost of generating code that satisfies GenSense's global invariants becomes equal 
to the cost of writing correct code. The bypass and the solution converge. That is the 
point.

---

## Summary

GenSense's path from linter to ground truth is one interface change: MCP tool with 
three new advisory fields. The human does not disappear — they define the global 
invariants once, as architectural rules, and then step back. Every agent in the loop 
reads the same contract. The writer cannot ship what the reviewer will reject. The 
fixer knows exactly what to change. The developer only sees what genuinely requires 
human judgment.

In a world where LLMs write most code, the bottleneck is not generation — it is 
correctness verification. GenSense becomes the verification layer that every agent 
in the system must satisfy before code moves forward.
