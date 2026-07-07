# Blueprint Pro — Agent Reasoning Audit

**Scope:** Why the agent reasons poorly / over-references the catalog instead of designing first.
**Method:** Direct code read of `workers/api/`, `packages/agent/`, `docs/adr/`.
**TL;DR:** The architecture is sound in the docs (ADR-002 already states the right philosophy). The implementation inverts it: keyword pre-filters run *before* reasoning and then hard-cage the model to their output, gaps are deliberately hidden, there's no real clarification flow, and the "design from description" logic is duplicated six times with six different bugs.

---

## Bug Index

| # | Severity | Bug | Files |
|---|----------|-----|-------|
| 1 | Critical | Keyword pre-filter gates and cages model reasoning | 6 files |
| 2 | Critical | Catalog gaps are hidden from user/agent | `design.ts`, `chat.ts` |
| 3 | High | No real clarification — scripted survey or silent guessing | `chat.ts`, `converse/index.ts` |
| 4 | High | Design is one-shot, not iterative | `converse/index.ts` |
| 5 | High | Six duplicate "research modules from description" implementations | 6 files |
| 6 | Medium | MCP tool sprawl — 3+ tools claim to do the same job | `tools/index.ts` |
| 7 | Medium | Inconsistent module caps across duplicate implementations | 6 files |
| 8 | Low | Architecture composition uses the fast/cheap model tier | `models.ts` |
| 9 | Low | "Vague prompt" substitution can silently change user intent | `converse/index.ts` |

---

## 1. Keyword pre-filter gates and cages model reasoning (Critical)

**What's happening:**
Before the model ever sees a project description, a substring/keyword matcher scores catalog modules and produces a short candidate list. The model is then *instructed* it may only use that list:

```
// design.ts:103
"Your output MUST use ONLY the module names listed above. Every module in
'modules[]' must be EXACTLY one of them."

// converse/index.ts:150
"Use ONLY these exact module names"
```

**Why it's the root cause of "dumb / keeps referencing blueprint":**
This isn't the model choosing to lean on the catalog — it's mechanically forbidden from doing anything else. The candidate list itself comes from `name.includes(word)` style matching (e.g. `design.ts` `researchBlueprint()`), which is brittle: plurals, synonyms, and domain language outside the matched terms are invisible to the model. The model can't reason about the architecture first and consult the catalog for patterns second, because the catalog selection happens *before* any model call, and the model call is then locked to it.

**Fix:**
- Let the model see the *full* candidate pool (or a much larger one, via semantic/embedding search instead of substring match) and reason about which modules fit — with the catalog framed as "available patterns," not a closed set.
- Change the instruction from "use ONLY these names" to: *"These are matching Blueprint patterns. Use them where they fit. Where the catalog doesn't cover something, use your own engineering judgment and say so explicitly."*
- Track which decisions came from the catalog vs. the model's own knowledge — both for transparency and for instrumenting where the catalog has real coverage gaps.

---

## 2. Catalog gaps are hidden from the user/agent (Critical)

**What's happening:**
`docs/adr/002-design-gate.md` states the intended behavior: *"Gaps in Blueprint catalog are surfaced before implementation."* The code does the opposite in two places:

```
// design.ts:189-198
// "Log gaps privately — never show to users"
if (architecture.gaps?.length > 0) {
  console.error(`[BLUEPRINT-GAP] Design gaps for "${body.description}": ...`);
}
const { gaps, ...safeArchitecture } = architecture;  // stripped from response

// chat.ts:126
// "Strip gaps before saving — they are internal, never user-facing"
const { gaps: _gaps, ...safeArch } = arch;
```

**Why it matters:**
This is the exact signal that should tell the agent (and the human) where it needs to think for itself instead of leaning on the catalog. Suppressing it into a server log means neither the user nor any downstream MCP-driving agent ever learns the catalog didn't cover something — they just silently get an architecture that may be missing pieces.

**Fix:**
- Surface `gaps` as a first-class section of the design output (this is already one of the 8 mandatory sections in `DESIGN_TEMPLATE_SECTIONS` — it's being generated and then thrown away).
- Remove both stripping steps. If there's a legitimate reason to hide internal debug info, separate "internal diagnostic gaps" from "user-facing coverage gaps" instead of deleting the whole field.

---

## 3. No real clarification flow (High)

**What's happening:**
Two different, both-wrong patterns:

- **CLI (`packages/agent/src/cli/chat.ts:17-21`):** A fixed, scripted 3-question survey (scale / constraints / timeline) fires every time, regardless of how detailed the user's first message already was. It's also disabled entirely in the TUI client (`setQualification(false)` in `chatTUI`).
- **Production `/converse` endpoint (`converse/index.ts:203-212`):** On a short/vague prompt, the code explicitly does *not* ask anything. It pattern-matches words like `"crud"`, `"api"`, `"rbac"`, `"saas"`, `"auth"` and silently substitutes a fabricated, more-detailed description in their place. Comment in the code: `// Vague prompt — propose a reasonable default instead of asking`.

**Why it matters:**
This is the inverse of "give follow-ups, be free-flow, understand intent." Right now the system either interrogates with a canned script unrelated to what's actually missing, or guesses and silently runs with it — the user never sees that their intent was reinterpreted.

**Fix:**
- Replace the scripted survey with model-generated clarification: after the description comes in, ask the model *"what's genuinely ambiguous or missing here for a production design (scale, compliance, consistency needs, integrations, etc.)?"* and only surface follow-ups for what's actually unclear. Skip entirely if the prompt is already detailed.
- Never silently rewrite the user's stated intent. If the prompt is too thin to design from, ask — don't substitute.

---

## 4. Design is one-shot, not iterative (High)

**What's happening:**
`converse/index.ts:214`:
```
const wantsDesign = !hasArchitecture && !isQuestion && body.messages?.length === 1 && query.length > 8;
```
Architecture composition only fires on the very first message of a session. Everything after that — even if the user adds significant new detail — falls through to the generic "general conversation" branch, which just chats with a 3-module catalog snippet stapled into the system prompt. There's no path back into real design reasoning.

**Fix:**
- Allow re-composition whenever the user adds material new information (new constraints, new scale, new compliance requirement, explicit "redesign" intent) — not just on message #1.
- Track which parts of the architecture are still valid vs. need re-deriving, so a follow-up doesn't throw away prior decisions unnecessarily.

---

## 5. Six duplicate "research modules from description" implementations (High)

**What's happening:**
The same job — "given free text, score catalog modules by relevance" — is implemented independently six times, each with its own keyword dictionary, its own term-length cutoff, and its own bugs:

| Implementation | Location |
|---|---|
| `researchBlueprint()` | `workers/api/src/routes/design.ts` |
| keyword scorer | `workers/api/src/routes/discover.ts` |
| `researchBlueprint()` (different one, own `DOMAIN_MODULES` dict) | `workers/api/src/converse/index.ts` |
| `handleArchitect()` | `workers/api/src/tools/handlers/premium.ts` (`architect_system` MCP tool) |
| `handleSuggestModules()` | `workers/api/src/tools/handlers/cross_language.ts` |
| `handleDesignSystem()` | `workers/api/src/tools/handlers/cross_language.ts` |

**Why it matters:**
A fix or improvement to one (e.g. adding a new domain keyword) never propagates to the other five. Depending on which entry point a user or agent happens to hit, they get a different module set for the *same* description, with no consistency guarantee.

**Fix:**
- Extract one shared `researchCatalog(description, catalog)` function. Every route and every MCP tool calls it. One place to fix, one place to improve (e.g. swap substring matching for embeddings later).

---

## 6. MCP tool sprawl (Medium)

**What's happening:**
`workers/api/src/tools/index.ts` exposes at least three MCP tools that all claim to "design a system from a description":
- `architect_system` — *"Design a complete system architecture from a product description."*
- `design_system` — *"Design a system from description with polyglot awareness."*
- `suggest_modules` — *"Suggest modules from a description..."*

**Why it matters:**
Whatever agent is driving tool calls (opencode, Claude Code, etc.) has no principled way to choose between these, and each one (per Bug #5) returns a different module set for the same input. This produces exactly the symptom of an agent that looks erratic or over-reliant on the catalog — it may call more than one, get conflicting answers, and default to whichever the catalog handed back.

**Fix:**
- Pick one canonical tool for "design from description." Deprecate or merge the other two. If polyglot-specific output is genuinely a different need, make it a parameter on the one tool, not a separate tool with separate logic.

---

## 7. Inconsistent module caps across duplicate implementations (Medium)

**What's happening:** Because of Bug #5, the number of candidate modules shown to the model varies arbitrarily by entry point:

| Location | Cap |
|---|---|
| `design.ts` `composeArchitecture` | top 25 |
| `converse/index.ts` `composeArchitecture` | top **5** |
| `handleArchitect` (premium.ts) | top 20 |
| `handleSuggestModules` | top 10 |

**Why it matters:** The `/converse` path — the one the actual CLI uses by default (`packages/agent/src/cli/chat.ts` calls `/converse`, not `/design`) — gives the model the *smallest* candidate pool of all of them (5 modules), even though `researchBlueprint()` in that same file selects up to 15 before truncating. The model is structurally prevented from seeing most of what it resolved as relevant.

**Fix:** Resolved automatically once Bug #5 is fixed (single implementation, single configurable cap, tuned once rather than guessed six times).

---

## 8. Architecture composition uses the fast/cheap model tier (Low — needs verification)

**What's happening:** `workers/api/src/ai/models.ts` — both `pickModel("pro")` paths (used by `design.ts` and `converse/index.ts` for architecture composition) resolve to `"deepseek-v4-flash"`, a "flash"-class model. This is the single highest-reasoning-load step in the whole pipeline (SDLC mapping, capacity estimates, distributed-systems tradeoffs, ADRs) and it's running on the same tier as quick chat replies.

**Fix:** Worth A/B testing a stronger model specifically for `composeArchitecture` / `handleArchitect`, separate from the model used for conversational replies. Cost is higher per design call, but design calls are infrequent relative to chat turns.

---

## 9. Silent intent substitution on vague prompts (Low, related to #3)

**What's happening:** `converse/index.ts:206-211` — if the user's prompt is short and contains a trigger word, the actual `query` variable used for design is replaced with a hardcoded longer description (e.g. `"crud"` → `"a CRUD API with users, products, orders and PostgreSQL"`). The user never sees this substitution happen; the returned message just refers to "your project" as if it matched what they typed.

**Fix:** Once Bug #3 is fixed (ask instead of guess), this becomes moot. If a default-and-confirm pattern is kept for speed, the substituted description must be shown back to the user before designing from it.

---

## Suggested Fix Order

1. **Bug #5** (consolidate the six scorers into one function) — unblocks everything else; every other fix is easier once there's one implementation to change.
2. **Bug #1** (stop hard-caging the model to the pre-filtered list) — this is the direct fix for "agent keeps referencing the blueprint instead of reasoning."
3. **Bug #2** (stop hiding gaps) — small change, high value, already half-built (the field exists, it's just being deleted).
4. **Bug #3 + #4** (real clarification, iterative design) — the part that makes vague prompts feel like a conversation instead of a survey or a guess.
5. **Bug #6 / #7** — cleanup once #5 is done.
6. **Bug #8 / #9** — lower priority, verify with real usage data first.
