# Pocket Workspace — Codebase Audit

Scope: `workers/pocket-workspace-api` (backend) and `sites/pocket-workspace` (frontend).
Goal: find why the agent can't continue conversations, why Run tabs / WebContainer terminal misbehave, and clean up deprecated Blueprint module code.

---

## Severity legend
- 🔴 Critical — breaks the feature outright
- 🟠 High — feature silently fails or degrades
- 🟡 Medium — cleanup / correctness, not user-blocking today
- ⚪ Dead code — unreachable, safe to delete (after dependency check)

---

## 1. 🔴 Broken AI Gateway URL — root cause of "can't continue conversation"

**File:** `workers/pocket-workspace-api/src/workspace/domain/models.ts`

```js
export const AI_API = "https://gateway.ai.cloudflare.com/v1/{ACCOUNT_ID}/{GATEWAY_NAME}/opencode-ai/v1/chat/completions";
```

`{ACCOUNT_ID}` and `{GATEWAY_NAME}` are never substituted anywhere in the repo (verified via grep — this is the only place those tokens appear). This file is the "no Blueprint dependency" replacement for model selection, and it's used by:

- `workspace/domain/ai-engine.ts` → `runUnifiedToolLoop()` (lines 41 and 152) — the engine behind **every** chat turn and the agent run workflow.

Every `fetch(api, ...)` call hits this literal templated string and 404s. `runUnifiedToolLoop` then returns `"Error: AI API returned 404 - ..."` as the assistant's reply on every single turn, including continuations.

It's also **incomplete**: it never exports `pickReasoningModel()`, which `chat-shared.ts` needs — that's *why* `chat-shared.ts` still imports model selection from the deprecated `shared/blueprint` module (see §6).

### Fix
- Replace the templated URL with a real, working endpoint. Either:
  - Hardcode the actual account/gateway IDs (from your Cloudflare dashboard), or
  - Read them from `env` (e.g. `env.CF_ACCOUNT_ID`, `env.CF_GATEWAY_NAME`) and build the URL at request time, or
  - Simplest: point `AI_API` directly at the working OpenCode Zen endpoint already used elsewhere in the codebase: `https://opencode.ai/zen/go/v1/chat/completions` (this is what the old Blueprint `models.ts` used and is confirmed reachable).
- Add `pickReasoningModel()` to this file so `chat-shared.ts` can stop depending on `shared/blueprint`.

---

## 2. 🟠 Frontend calls backend routes that don't exist

Checked every `fetch()` call in `runs.js` / `chat.js` against the router in `src/index.ts`. These routes are called by the client but have **no matching handler**, so they 404 (caught silently by try/catch):

| Frontend call | File:line | Status |
|---|---|---|
| `POST /agent/run/approve` | `runs.js:289` | Not routed |
| `POST /agent/run/reject` | `runs.js:289` | Not routed |
| `POST /agent/run/request_changes` | `runs.js:321` | Not routed |
| `POST /agent/run/rpc_result` | `runs.js:90` | Not routed |
| `GET /agent/stream` | `chat.js:622` | Not routed |

Effect: the entire plan/code review UI (Approve / Reject / Request changes buttons) and the "resume an in-progress run after backgrounding the app" feature are dead on arrival.

### Fix
Add handlers + router entries in `src/index.ts` for all five paths. They need to:
- `/agent/run/rpc_result` — write the posted result into whatever storage `verifyWebContainerModule` polls (`rpc_result_${commandId}`), keyed by project/run.
- `/agent/run/approve` / `reject` / `request_changes` — advance the run's state (likely stored per-project, e.g. in `ConversationDO` or D1) and emit a corresponding event so the SSE/WS stream picks it up.
- `/agent/stream` — return an SSE stream of buffered events for a project since a given sequence number (the frontend already sends `?projectId=...&since=...`).

Note: none of these matter until §1 is fixed and the run workflow actually reaches a review/RPC state — see §4.

---

## 3. 🟠 Run log text is literally `\n`, not real newlines

**File:** `sites/pocket-workspace/modules/chat.js` (13 occurrences)

Example (line 575):
```js
genStreamBuffer += "\\n" + ICON.thought + " " + event.content;
```

`"\\n"` in JS source is an escaped backslash followed by `n` — i.e. the two literal characters `\` and `n`, **not** a newline control character. `updateGenStream()` later does `genStreamBuffer.replace(/\n/g, "<br>")`, which matches real newlines only, so these never get converted to line breaks. Result: the Run tab's live log shows visible `\n` text instead of line breaks.

Affected lines: 474, 544, 559, 560, 575, 578, 581 (×2 — `"\\n```\\n"` and `"\\n```"`), 586, 590, 594, 626, 650.

### Fix
Replace every `"\\n"` in this block with an actual newline (`"\n"` — one backslash in the source, or a template literal). Also fix line 638 (see §5).

---

## 4. 🟠 Resume-run feature is broken twice over

**File:** `sites/pocket-workspace/modules/chat.js:638`

```js
const parts = buffer.split("\\n\\n");
```

Same bug as §3: this should be `buffer.split("\n\n")` (real double-newline, which is the SSE event delimiter). As written, it searches for the literal 4-character sequence `\n\n` and never matches real SSE frames, so `parts` is always `[buffer]`, `parts.pop()` removes the only entry, and the `for (const part of parts)` loop never runs. The resume/reconnect-to-active-run code silently processes zero events even if the (currently missing — see §2) `/agent/stream` endpoint existed and worked.

### Fix
Change to `buffer.split("\n\n")`. Combine with adding the `/agent/stream` route from §2.

---

## 5. 🟠 WebContainer terminal repeatedly tears down and reboots

**File:** `sites/pocket-workspace/modules/runs.js:175` and `modules/webcontainer.js`

```js
// runs.js
if (!window.xtermInstance) { ... mountTerminal(...) ... }
```

`webcontainer.js` declares `let xtermInstance = null;` at **module/script scope**. These are loaded as classic `<script>` tags (no `type="module"`), and a top-level `let`/`const` never attaches to `window` (unlike `var` or `function` declarations). So `window.xtermInstance` is **always `undefined`**, and this guard is always true.

Compounding bug: the companion guard `run._wcBooted` is set on a `run` object that comes from `loadRuns()` → `JSON.parse(localStorage.getItem(...))` — a brand-new object parsed fresh from storage on every render — and is **never persisted** back via `saveRuns()`. So even the "have we already booted this run's container" flag is lost on the very next render.

Effect: `renderRuns()` is called after every single SSE/status event. Each time a run card is expanded and in `"done"`/`"Verify"` phase, the code re-mounts the terminal *and* re-runs `wc.mount(fileTree)` → `npm install` → `npm run dev` from scratch — terminal thrashing, duplicate dev-server processes, and an unusable terminal during an active run.

### Fix
- Expose the real instance, e.g. in `webcontainer.js`: `window.xtermInstance = xtermInstance;` whenever it's (re)assigned in `mountTerminal()`, or better, replace the global-variable pattern with an explicit getter function (`function getXtermInstance() { return xtermInstance; }`) that `runs.js` calls instead of touching `window` directly.
- Persist `_wcBooted` properly: store booted-run-ids in a separate, explicitly-saved structure (e.g. a `Set` in `state`, or a dedicated `localStorage` key), not as a mutation on a throwaway object from `loadRuns()`.

---

## 6. 🟠 Backend agent run never actually drives the WebContainer

**Files:** `workspace/domain/agent-run-workflow.ts`, `workspace/providers/webcontainer-provider.ts`

`webcontainer-provider.ts` exports `verifyWebContainerModule()`, which is designed to emit an `rpc_call` SSE event (telling the browser to run `npm install && npm run build && npm test` in its WebContainer) and then poll Durable Object storage for up to 120s waiting for the result. **This function is never called anywhere** — confirmed via grep, it has zero callers. `AgentRunWorkflow.run()` only calls `runUnifiedToolLoop()` with simple read/write/list-file tools; it never touches the WebContainer RPC path, never emits `status` events with a `module` field (the ones it does emit via `onStatusUpdate` have `content`/`status` fields but no `module`, so `updateRun()`'s `event.type === "status" && event.module` check silently drops them), never reaches a `"review"` or `"plan_review"` phase, and never sets `previewUrl`.

Net effect: today's Run tab can only ever show "thought"/"action" log lines and a final "Done" — no module-by-module progress, no plan/code review step, no preview link, and the terminal/WebContainer is *only* ever driven by the (broken, see §5) client-side auto-boot logic in `runs.js`, completely disconnected from the backend agent.

### Fix
This is a design decision, not a one-line fix. Either:
- Wire `verifyWebContainerModule()` into `AgentRunWorkflow` if you want backend-orchestrated browser verification (then also fix §2's missing `rpc_result` route so the 120s poll can actually succeed), or
- Remove the dead `verifyWebContainerModule` / RPC-call / review-phase UI entirely and simplify `runs.js` to match what `AgentRunWorkflow` actually does (file writes + a final done/error state), which is a much smaller surface to keep correct.

Given the rest of this audit, simplifying is probably the pragmatic choice — the richer review/RPC flow appears to be inherited UI from the Blueprint product that the current workflow was never rebuilt to support.

---

## 7. 🟡 Legacy Blueprint module — what's dead vs. still wired in

`grep -ril blueprint` turns up references throughout `src/shared/`, `src/workspace/`, and `wrangler.toml`. Breaking it down:

### Fully dead (safe to delete)
- **`src/shared/index.ts`** and everything under **`src/shared/blueprint/`** (364 KB) — this is a separate `fetch` handler (billing, licensing, waitlist, `/generate`, `/discover`, `/catalog`, the whole Blueprint SaaS surface). `wrangler.toml` sets `main = "src/index.ts"`, and `src/index.ts` never imports `shared/index.ts`. It looks like a leftover monolith from before Blueprint was split into its own deployed service (`wrangler.toml` already has a `BLUEPRINT_API` service binding for the real, separate worker). **Unreachable in this worker.**
- **`workspace/domain/architecture.ts`** (`composeArchitecture`) — imported by nothing in the live `workspace/` tree. Only ever referenced by the also-dead `shared/blueprint/routes/design.ts`.
- **`handleAgent`, `handleInterfaces`, `handleClassify`, `handleStep`** in `workspace/routes/agent-chat-routes.ts` (≈90% of that file), routed at `/agent`, `/agent/interfaces`, `/agent/classify`, `/agent/step` in `src/index.ts`. Verified the frontend never calls any of these three paths — only `/agent/run` is used. Prompts inside them still say "You are Blueprint's planning agent" / reference "Blueprint modules". Pure legacy surface.

### Still wired into the live app (cannot delete yet)
- **`workspace/domain/chat-shared.ts`** imports `pickModel`, `pickReasoningModel` from `shared/blueprint/index.ts`.
- **`workspace/domain/architecture.ts`** imports `hybridSearch` from `shared/blueprint/index.ts` (moot once §7's dead-code removal happens, but technically a live import today).
- **`workspace/routes/agent-chat-routes.ts`** imports `loadCatalog` from `shared/blueprint/data/catalog.js` (line 5) and dynamically imports `shared/blueprint/domain/models.js` directly (line 271, inside the dead `handleStep` — once `handleStep` is deleted this import goes away too).

### Fix order
1. Fix `workspace/domain/models.ts` (§1) and add `pickReasoningModel()` to it.
2. Update `chat-shared.ts` to import `pickModel`/`pickReasoningModel` from `workspace/domain/models.ts` instead of `shared/blueprint`.
3. Delete `handleAgent`, `handleInterfaces`, `handleClassify`, `handleStep` from `agent-chat-routes.ts` and their router entries in `src/index.ts` (§2/§6 decision permitting).
4. Delete `workspace/domain/architecture.ts` (unused) and its `hybridSearch` import.
5. Once nothing under `workspace/` imports anything from `shared/blueprint/` or `shared/index.ts`, delete `src/shared/index.ts` and `src/shared/blueprint/` wholesale (~364 KB).

---

## 8. 🟡 Minor / housekeeping

- `workspace/domain/converse.ts` imports `pickModel` from `./models.js` but never calls it — dead import, harmless, but worth removing while touching this file.
- `src/shared/tools.ts` is a 2-line re-export shim duplicating `src/shared/tools/index.ts` — consolidate to avoid ambiguity (not currently imported by anything live, but confusing to find both).

---

## Suggested fix order

1. **§1** — fix the AI Gateway URL. This alone should restore basic chat/continuation.
2. **§3 + §4** — fix the `"\\n"` string bugs in `chat.js` (quick, isolated, high-value for Run tab readability and resume).
3. **§5** — fix the `window.xtermInstance` / `_wcBooted` terminal-remount bug.
4. **§6** — decide: wire up real backend↔WebContainer RPC, or simplify the Run tab UI to match what the workflow actually does. Do this before §2, since it determines which routes you actually need.
5. **§2** — add only the routes still needed after the §6 decision.
6. **§7** — Blueprint cleanup, in the listed order, last (it's not breaking anything live today, just bloat and confusion).
