# Backend & MCP Tools Audit — Blueprint API (`workers/api`)

**Scope:** `workers/api/src/**` (~12,200 lines of TypeScript), D1 migrations, `wrangler.toml` configs.
**Method:** full read-through of every source file, cross-referencing duplicated logic, tracing auth/credit code paths end-to-end, and verifying two regex-based bugs empirically with Node.
**Not in scope:** the `@friehub/blueprint` npm package itself (not included in this bundle — referenced as an external dependency in `converse/architecture.ts` and `tools/handlers/core.ts`).

Findings are ordered by severity. Each includes the exact file/line, why it matters, and a fix direction.

---

## Summary

| # | Severity | Finding |
|---|----------|---------|
| 1 | 🔴 Critical | No ownership check anywhere on `project_id` — full cross-account IDOR on files, env vars/secrets, plans, memory |
| 2 | 🔴 Critical | `/state/:id` has **zero auth** and shares a KV key with the agent-run engine — anyone can hijack/corrupt any run |
| 3 | 🔴 Critical | Credit deduction is a non-atomic read‑check‑write — double-spend race condition |
| 4 | 🔴 Critical | Session tokens are hardcoded to `tier: "free"` at login — paid web/session users never get paid features |
| 5 | 🔴 Critical | `/agent/run` has no tier gate, no credit deduction, no rate limit — free unlimited cloud-sandbox + LLM usage |
| 6 | 🟠 High | Two divergent auth-resolution implementations (`resolveTier` vs `resolveAuth`) — Bearer-token users denied paid tools; failures silently skip billing |
| 7 | 🟠 High | `parseGeneratedFiles()` silently drops any generated file whose name contains a hyphen (proven) |
| 8 | 🟠 High | "Financial module" keyword list is inconsistent across 5 call sites — `transfer_*` functions skip the bigint/idempotency/locking mandates in 2 of 5 paths |
| 9 | 🟠 High | `callReasoningAI()` never calls `pickReasoningModel()` despite its own doc comment |
| 10 | 🟠 High | `/admin/index-catalog` is reachable by *any* license key, including expired ones, with no tier check |
| 11 | 🟠 High | GitHub file reads decode base64 with `atob()` — corrupts any non-ASCII UTF‑8 content |
| 12 | 🟠 High | CodeSandbox task timeout hardcoded to 10s for `npm install && npm run build && npm test` |
| 13 | 🟡 Medium | `handleAgentRun` writes files to the sandbox twice (copy-paste duplication) |
| 14 | 🟡 Medium | GitHub Actions run-matching isn't paginated — false "verification timed out" on busy repos |
| 15 | 🟡 Medium | MCP tool credit deductions never appear in the billing ledger/usage history |
| 16 | 🟡 Medium | `makeCors()` can return `Allow-Origin: *` together with `Allow-Credentials: true` |
| 17 | 🟡 Medium | Session/auth helper functions duplicated 4–5×, already causing real drift |
| 18 | 🟡 Medium | Internal errors leak raw exception messages to API clients |
| 19 | 🟡 Medium | Cloudflare Workers `fetch()` handler never receives `ExecutionContext` — no `waitUntil`, so long agent runs are tied to the client connection |
| 20 | 🟡 Medium | Security scanner is single-line regex matching — trivially evaded, many false positives |
| 21 | 🟢 Low | `wrangler.toml` commits real KV/D1/R2 resource IDs; staging explicitly shares prod KV |
| 22 | 🟢 Low | Dead code: `streamAI()`, `streamFreeChat()` never called |
| 23 | 🟢 Low | Misc: duplicate object key, dead regex alternative, top-level `await import()` of a local file, 36 empty `catch` blocks, 469 untyped `any` |

---

## 🔴 Critical

### 1. No ownership check on `project_id` anywhere in the MCP tool / memory layer

`project_files`, `project_env_vars`, `agent_plans`, `decisions`, and `constraints` all carry a `project_id` column, and the `projects` table even has an `owner_id` (migration `0001_create-tables.sql`). But **every MCP tool handler that touches these tables trusts the client-supplied `project_id` verbatim**, with no check that the caller's authenticated `customerId` actually owns it:

- `tools/handlers/workspace.ts` — `handleWorkspaceReadFile`, `WriteFile`, `DeleteFile`, `ListFiles`, `SearchFiles`, `ReadMultipleFiles`, `RenameFile`, `Export`, `handleEnvVarSet/Get/List/Delete`, `handleUpdatePlan/GetPlan`, `handleRecallContext` — none of them query `owner_id`.
- `converse/tools.ts` `execChatTool()` (lines 20-25) — same pattern for the in-chat `workspace_read_file`/`workspace_list_files` tools.
- `pocket/converse.ts` `execPocketTool()` (lines 36-50) — same pattern for the "free chat" tool loop.
- `routes/agent.ts` `handleAgentRun()` — state keyed only by client-supplied `projectId`, no ownership check.

Compare this to `account/workspace.ts`, where the REST API for the same tables does it correctly:

```ts
// account/workspace.ts:122-127 — handleUploadFiles
const proj = await (env as any).blueprint_memory
  .prepare("SELECT owner_id FROM projects WHERE id = ?")
  .bind(projectId).first();
if (!proj) return apiError("Project not found", 404);
if ((proj as any).owner_id !== session.customerId) return apiError("Forbidden", 403);
```

**Impact:** any authenticated user (including free tier) can read, overwrite, or delete another customer's source files, decisions/constraints memory, and — worst of all — **`project_env_vars`, which stores plaintext secrets** (the `is_secret` column is just a display flag; it does not gate read access). They simply have to supply a different `project_id` in the tool call or in the `session.project` field of `/converse`.

**Fix:** add a single `assertProjectOwnership(env, projectId, customerId)` helper (mirroring `account/workspace.ts`'s pattern) and call it at the top of every handler in `tools/handlers/workspace.ts`, `converse/tools.ts`, `pocket/converse.ts`, and `routes/agent.ts` before touching D1/KV.

### 2. `handleStateSync` has no auth, and shares a KV key with the agent-run engine

`routes/agent.ts`:

```ts
// handleStateSync — no resolveAuth() call at all
export async function handleStateSync(request: Request, env: Env): Promise<Response> {
  if (request.method === "POST") {
    const body = JSON.parse(await request.text()) as { id: string; state: any };
    await env.LICENSES_KV.put(`project:${body.id}:state`, JSON.stringify(body.state), { expirationTtl: 60 * 60 * 24 * 30 });
    ...
```

This is wired up at `/state/:id` in `index.ts` with **zero call to `resolveAuth`**. Anyone on the internet can `POST` arbitrary JSON to `/state/<any-id>` or `GET` it back.

It gets worse: `handleAgentRun` uses the *exact same key* for its own internal sandbox/build state:

```ts
const stateKey = `project:${projectId}:state`;   // routes/agent.ts:578
```

These are two unrelated features (`/agent/run`'s live build loop vs. the documented "Track 11 — for team sessions, multi-device" sync feature) silently sharing one KV namespace. Combined with #1 (client-controlled `projectId`), this means:

- Anyone can read another customer's in-progress build state (sandbox ID, preview URL, generated code, GitHub commit SHAs).
- Anyone can **overwrite** that same key via an unauthenticated `POST /state/<projectId>` mid-run, corrupting or hijacking an in-progress agent run for any project whose ID they know or guess.

**Fix:** require `resolveAuth` + ownership check on `handleStateSync`, and put it on a *different* key prefix (`sync:${projectId}:state`) than the agent engine's internal state.

### 3. Credit deduction is a non-atomic read → check → write (double-spend race)

`auth/credits.ts`:

```ts
export async function deductCredits(env: Env, customerId: string, amount: number, idempotencyKey?: string): Promise<boolean> {
  ...
  const raw = await env.LICENSES_KV.get(key);
  const balance = raw ? parseInt(raw, 10) : 0;
  if (balance < amount) return false;
  ...
  await env.LICENSES_KV.put(key, String(balance - amount));
```

This get→check→put sequence is not atomic and Workers KV is eventually-consistent. Two concurrent requests (very plausible in a "multi-device pocket workspace" product, or even just a chat client firing several tool calls in parallel) both read the same starting balance, both pass the check, both deduct — the customer can spend more credits than they have. The same unguarded pattern is reused independently in `deductToolCost()` (`tools/index.ts:119-126`), which is a *third*, separate hand-rolled re-implementation that doesn't even share code with `deductCredits()`.

**Mitigating factor:** `routes/generate.ts` (the paid `/generate` endpoint) adds an external soft lock (`reserve:${customerId}` key, lines 73-86) specifically around its own call — credit due, that path is reasonably protected. But the MCP tool-call path (`deductToolCost`, used by every Pro/Premium MCP tool) has **no such lock**, so it's exposed to the same race the `/generate` endpoint was clearly designed to avoid.

**Fix:** route every credit deduction through one function, and either (a) take the same `reserve:` lock pattern already proven in `generate.ts`, or (b) move balances to D1 and use a single `UPDATE ... SET balance = balance - ? WHERE balance >= ?` with `changes` checked, which D1's SQLite engine can do atomically per-statement.

### 4. Session tokens are hardcoded to `tier: "free"` at login

`account/auth.ts`, both `handleLogin` (line 45) and `handleGoogleCallback` (line 136):

```ts
await env.LICENSES_KV.put(`tok:${token}`, JSON.stringify({ customerId: acct.customerId, tier: "free" }), { expirationTtl: 86400 });
```

The customer's *real* tier (pro/team/premium/enterprise) only lives in a separate `key:<hash>` license record, looked up correctly in `account/billing.ts`'s `handleBlueprintOverview` — but **never** when a session is minted. Every downstream consumer of session-token auth reads tier straight off this record:

- `auth/sessions.ts` `getSession()` → `tier: data.tier ?? "free"`
- `tools/index.ts` `resolveTier()` → same `tok:` lookup
- `converse/index.ts` → `const hasBlueprint = auth.tier !== "free"` gates the entire Blueprint-grounded chat experience

**Impact:** a customer who logs in via email/password or "Sign in with Google" — i.e. the normal dashboard/web flow — is permanently treated as free tier for every session-token-gated feature (`/converse` Blueprint grounding, MCP tool tier gates, `/generate` tier limits), no matter what they're actually paying for. The only way to unlock paid features is to copy-paste the raw `fhp_...` API key into `X-License-Key` directly, bypassing session auth entirely. If Pocket Workspace's mobile UI authenticates primarily via session token (it does — see frontend audit), **paying mobile users may never see their paid tier reflected in the app.**

**Fix:** at login, look up the customer's current license tier (the same lookup `handleBlueprintOverview` does) and store the real tier in the `tok:` record; refresh it on a TTL or on each billing-state-changing webhook.

### 5. `/agent/run` has no tier gate, no credit deduction, and no rate limit

`routes/agent.ts` `handleAgentRun()` only calls `resolveAuth` (any authenticated user, any tier, passes) before:

- spinning up a real CodeSandbox Devbox VM,
- calling `handleGenerateImpl()` / `handleRepairImpl()` — paid LLM calls — up to 3 planning-loop iterations, each potentially 1-2 model calls,
- polling GitHub Actions for up to 60 seconds per module.

There is no `deductCredits`/`deductToolCost` call anywhere in this function, and no `limit:` key check (contrast with the much cheaper `handleAgent()` 30 lines above it, which *does* enforce `AGENT_DAY_LIMIT = 10/day`). A free-tier customer can call `/agent/run` in a loop and get unlimited cloud compute + LLM generations for free.

**Fix:** add the same tier check used elsewhere (`PRO_TIERS.includes(tier)`) and a per-loop-iteration credit deduction before `handleGenerateImpl`/`handleRepairImpl` are invoked; add a daily/concurrent-run limit analogous to `AGENT_DAY_LIMIT`.

---

## 🟠 High

### 6. Two divergent, hand-rolled auth resolvers

`tools/index.ts` `resolveTier()` only recognizes `X-Session-Token` (and only the `tok:` KV prefix, not the `sess:` hashed fallback) and `X-License-Key`:

```ts
async function resolveTier(request: Request, env: Env): Promise<string> {
  const sessionToken = request.headers.get("X-Session-Token");
  const licenseKey = request.headers.get("X-License-Key");
  ...
```

Meanwhile the canonical `resolveAuth()` in `auth/sessions.ts` also accepts `Authorization: Bearer <token>` and `X-Api-Key`, and checks both `tok:` and `sess:` KV prefixes. Since the MCP tool dispatcher (`handleMcp` in `tools/index.ts`) uses `resolveTier()` to gate tier-restricted tools but then calls `resolveAuth()` *separately* a few lines later just to find a `customerId` for billing:

```ts
let customerId = "";
try { const a = await resolveAuth(request, env); if (a) customerId = a.customerId; } catch {}
if (customerId && toolDef.name) {
  const allowed = await deductToolCost(customerId, toolDef.name, env);
  ...
```

two concrete bugs fall out:
1. A client authenticated only via `Authorization: Bearer` is correctly tier-gated as paid by nothing (`resolveTier` doesn't look at that header) — they get `403 tier_required` even though they're a paying customer.
2. If `resolveAuth()` throws or returns `null` for any reason while `resolveTier()` succeeded (different lookup paths, KV inconsistency, etc.), `customerId` stays empty and the `if (customerId && ...)` guard **skips billing entirely while the tool still runs** — a free execution.

**Fix:** call `resolveAuth()` once, derive both `tier` and `customerId` from the same result, and delete `resolveTier()`.

### 7. `parseGeneratedFiles()` drops any filename containing a hyphen — verified

`tools/handlers/core.ts:385`:

```ts
const fileRegex = /---\s*([^\s-]+)\s*---([\s\S]*?)(?=(?:---\s*[^\s-]+\s*---|$))/g;
```

The filename character class `[^\s-]+` excludes the hyphen. Verified with a standalone repro:

```js
const code = "--- error-codes.ts ---\nexport const A = 1;\n--- types.ts ---\nexport type B = string;\n";
// parseGeneratedFiles(code) => [{ path: "types.ts", content: "export type B = string;" }]
// "error-codes.ts" and its entire content are silently dropped — no error, no warning.
```

Since hyphenated filenames (`rate-limiter.ts`, `error-handling.ts`, `feature-flags.ts`, …) are an extremely common convention, this means generated multi-file output regularly loses entire files with no indication to the agent or the user — they just never get written to the workspace/sandbox/repo.

**Fix:** change the capture group to `([^\s]+?)` (or explicitly allow `-` and `_`): `/---\s*([^\s]+)\s*---/`.

### 8. "Financial module" detection drifts between 5 near-identical copies

The same financial-keyword check is duplicated five times across the codebase. Three include `"transfer"`, two don't:

| Location | Keywords |
|---|---|
| `tools/handlers/core.ts:54` (`handleGenerateImpl`, fallback path) | `debit, credit, pay, refund, charge, transfer` |
| `tools/handlers/core.ts:282` (`handleValidateProject`) | `debit, credit, pay, refund, charge, transfer` |
| `routes/generate.ts:153` | `debit, credit, pay, refund, charge, transfer` |
| `tools/handlers/core.ts:374` (`generateFromContext` — the **architecture-driven** generation path) | `debit, credit, pay, refund, charge` ❌ no `transfer` |
| `catalog/resolver.ts:240` (`resolveArchitecture`) | `debit, credit, pay, refund, charge` ❌ no `transfer` |

`generateFromContext` is the code path used whenever an `architecture` object (from `resolve_architecture`) is passed into `generate_implementation` — i.e. the *recommended*, deterministic-resolver-driven flow described in the tool's own docstring ("STEP 4... Pass architecture from resolve_architecture for full context"). A module with a function like `transfer_funds()` will get the hard financial mandates (bigint amounts, mandatory idempotency keys, `SELECT ... FOR UPDATE`, outbox events) when generated through the legacy fallback path, but **silently skip all of them** when generated through the recommended architecture-driven path, purely because of a missing array entry.

**Fix:** extract one `FINANCIAL_KEYWORDS` constant and one `isFinancialModule(functions)` helper; use it everywhere instead of five inline copies.

### 9. `callReasoningAI()` never actually calls the reasoning model

`tools/helpers.ts:53-56`, with its own doc comment two lines above:

```ts
/**
 * Uses the stronger reasoning-tier model for architecture design calls.
 * These are infrequent but reasoning-heavy — worth the extra cost.
 */
export async function callReasoningAI(env: Env, system: string, prompt: string, opts: ... = {}): Promise<any> {
  const { api, model } = pickModel("pro");   // <-- should be pickReasoningModel()
```

`pickModel("pro")` returns the same flash/chat model (`deepseek-v4-flash`) used for ordinary conversation everywhere else in the codebase; `pickReasoningModel()` (which correctly returns `deepseek-v4-pro` via the `/zen/go/` endpoint) is defined in `ai/models.ts` and is correctly used in `converse/shared.ts`'s `callAI({ reasoning: true })`, but the `tools/helpers.ts` version of essentially the same utility — used by the *MCP* architecture/ADR/runbook tools — never wires it up. Every "reasoning-heavy" Pro/Premium tool that calls `callReasoningAI` (check call sites in `tools/handlers/premium.ts`, `style.ts`, etc.) is silently getting the cheaper model.

**Fix:** `const { api, model } = pickReasoningModel();`

### 10. `/admin/index-catalog` is gated only by "any non-expired-looking license key"

`routes/admin.ts:63-72`:

```ts
const licenseKey = request.headers.get("X-License-Key");
if (!licenseKey) return ...401...;
const keyHash = createHash("sha256").update(licenseKey).digest("hex").slice(0, 16);
const lic = await env.LICENSES_KV.get(`key:${keyHash}`);
if (!lic) return ...401...;
// no tier check, no status check — proceeds straight to re-indexing the whole catalog
```

This is yet another hand-rolled license lookup (the 5th+ in the codebase) and it's missing the `status === "expired" | "grace"` filter that the *canonical* `resolveApiKey()` in `auth/sessions.ts` applies. It also doesn't check tier at all — any customer (not an admin) with any valid key, including an expired-but-not-yet-purged one, can trigger a full Workers-AI-embedding + Vectorize re-index of the entire catalog, an operation that should be admin/CI-only.

**Fix:** gate this on a dedicated admin secret/header (e.g. compare against an `ADMIN_TOKEN` env binding), not a customer license key.

### 11. GitHub file reads corrupt non-ASCII content via `atob()`

`tools/github.ts:166-168`, `readFile()`:

```ts
if (data.encoding === "base64") {
  data.content = atob(data.content);
}
```

`atob()` decodes base64 into a binary string where each JS character is one byte (Latin‑1), not a proper UTF‑8 decode. Any file containing multi-byte UTF‑8 characters — emoji in comments, non-English text, smart quotes, accented characters — comes back mangled. The correct pattern is:

```ts
data.content = new TextDecoder().decode(Uint8Array.from(atob(data.content), c => c.charCodeAt(0)));
```

### 12. CodeSandbox task timeout hardcoded to 10 seconds

`tools/codesandbox.ts` `runTask()`:

```ts
body: JSON.stringify({ command, timeout: 10000 }),
```

This same client is used in `routes/agent.ts` to run `npm install <new deps>` and then `npm run build && npm test` in sequence. `npm install` alone routinely exceeds 10 seconds for any real dependency tree. The agent loop will systematically misclassify slow-but-correct builds as failures, triggering unnecessary (and costly) repair cycles via `handleRepairImpl`.

**Fix:** make timeout configurable per command (e.g. 120s+ for install/build, shorter for quick checks), or have `CodeSandboxClient.runTask` accept an explicit timeout parameter from the caller instead of a hardcoded constant.

---

## 🟡 Medium

### 13. `handleAgentRun` writes files to the sandbox twice

`routes/agent.ts`, inside the per-module generation block (lines ~740-779): for the `codesandbox` backend, files are written via `csb.writeFiles(...)`, then build/test is run — and then immediately afterward, a second, separate `if (backend === "codesandbox" && csb)` block re-emits the "Writing file ... to Sandbox" events and calls `csb.writeFiles(state.sandboxId, files)` again with the *same* `files` array. This looks like a leftover from refactoring (the github-backend `else if` branch was probably added later without removing the now-duplicated codesandbox block above it). It doubles the CodeSandbox API calls and duplicates SSE events sent to the client for no benefit.

### 14. GitHub Actions run lookup is unpaginated

`tools/github.ts` `getLatestWorkflowRun()` calls `/repos/${repo}/actions/runs?branch=${branch}` and does a client-side `.find(r => r.head_sha === commitSha)` with no `per_page`/pagination handling. On any repo with more than ~30 recent runs on that branch, a just-created run can be missed, causing `routes/agent.ts`'s poll loop to spin for the full `maxPolls` (60s) and report `"GitHub Actions verification timed out or could not be verified"` even though the run actually succeeded.

### 15. MCP tool credit deductions never show up in billing history

`auth/credits.ts`'s `deductCredits()` writes a `gen:${customerId}:${timestamp}` ledger entry that `getLedger()`/`handleBillingHistory()` read back for the customer-facing usage page. `deductToolCost()` in `tools/index.ts` (used for every paid MCP tool call) bypasses `deductCredits()` entirely and never writes a ledger entry. Customers will see their credit balance go down with zero corresponding line items in their transaction history.

### 16. `makeCors()` can emit an invalid/contradictory CORS response

`types.ts`:

```ts
export function makeCors(origin: string | null): Record<string, string> {
  const allow = origin && ALLOWED_ORIGINS.includes(origin) ? origin : "*";
  return {
    "Access-Control-Allow-Origin": allow,
    ...
    "Access-Control-Allow-Credentials": "true",
  };
}
```

For any request from an origin *not* in `ALLOWED_ORIGINS`, this returns `Allow-Origin: *` together with `Allow-Credentials: true` — a combination browsers reject per the Fetch spec (credentialed requests can't use a wildcard origin). The practical effect is that the request silently fails client-side with an opaque CORS error for any origin outside the allowlist, rather than a clean 403. Low security impact (the browser blocks it), but a confusing failure mode worth fixing: only set `Allow-Credentials` when `allow !== "*"`.

### 17. Session/auth helpers duplicated 4–5 times, already causing drift

Near-identical `extractSession()` / `getSession()` / `hashKey()` trios exist independently in `auth/sessions.ts`, `account/workspace.ts`, `account/auth.ts`, `account/billing.ts`, and (partially) `account/github.ts`. This isn't just a style nit — it's the direct root cause of finding #6 (`resolveTier` missing the `sess:` fallback that every other copy has) and a contributing factor to #10 (admin route re-implementing key lookup without the status check). Any future auth change (e.g. adding a new token type, fixing an expiry bug) has to be applied in 4–5 places by hand, and history shows that doesn't happen reliably.

**Fix:** consolidate to the single `resolveAuth`/`getSession` pair in `auth/sessions.ts` and have every other file import it.

### 18. Internal exceptions leak to API responses

`index.ts`'s top-level catch:

```ts
} catch (e) {
  const msg = e instanceof Error ? e.message : String(e);
  console.error("Unhandled error:", msg);
  res = new Response(JSON.stringify({ error: "internal_error", message: msg }), { status: 500, ... });
}
```

Any uncaught exception's raw `.message` is returned verbatim to the client. Several other handlers do the same (`routes/agent.ts`'s `handleStep` catch, etc.). Depending on what throws, this can leak internal details (D1/KV error text, stack-adjacent info, library error strings). Return a generic message to the client and log the detailed one server-side only.

### 19. No `ExecutionContext` / `waitUntil` anywhere — long agent runs are tied to the live client connection

The Worker's entry point is declared as:

```ts
export default {
  async fetch(request: Request, env: Env): Promise<Response> { ... }
};
```

with no third `ctx: ExecutionContext` parameter, and it's never threaded through to any handler. `handleAgentRun`'s entire multi-cycle planning loop (CodeSandbox spin-up, file writes, builds, GitHub Actions polling for up to 60s per module) runs inside a single `ReadableStream.start()` callback tied to that one request/response. Cloudflare Workers do not guarantee continued CPU time once a client disconnects from a streamed response unless the work is wrapped in `ctx.waitUntil()` or moved to a Durable Object / Queue / Workflow. On mobile — where backgrounding a browser tab, locking the screen, or a momentary network drop is the *normal* case, not the exception — this architecture risks silently aborting in-progress cloud builds. This is a design-level issue, not a one-line fix; see the companion features document for a recommended approach (Durable Objects-backed run state + push notification on completion, decoupled from the request lifecycle).

### 20. The security scanner is single-line regex matching, not real static analysis

`security/checks/*.ts` (sql-injection, command-injection, secrets, crypto) all operate line-by-line with regexes like:

```ts
{ re: /(?:pg\.query|client\.query|pool\.query)\s*\(\s*[`"']/i, severity: "medium", msg: "Direct SQL query without ORM" }
```

This flags **any** parameterized query written with a template literal (`db.query(\`SELECT * FROM x WHERE id = $1\`, [id])` is perfectly safe but still matches), so it will produce false "critical" findings on safe code (ironically, several `?`-bound prepared statements elsewhere in this very codebase, e.g. `tools/handlers/workspace.ts`, would trip the "Raw SQL query without parameterization" rule if scanned, despite being correctly parameterized). It also can't catch anything that spans multiple lines, and is trivially evaded by reformatting. Given `scanSecurity()` is used as a hard gate in `routes/agent.ts` (`blocked.length > 0` aborts the write), false positives will block legitimate generated code, and false negatives (multi-line injection, string-built queries split across lines) will pass code that shouldn't.

**Fix:** treat this as a fast pre-filter, not a verdict — pair it with a real parser-based check (e.g. AST-based taint tracking) before treating "critical" findings as a hard block, or at minimum surface findings as warnings the agent/user can review rather than an automatic abort.

---

## 🟢 Low / Cleanup

- **`wrangler.toml` commits live resource IDs** (KV namespace IDs, D1 database ID, R2 bucket name) directly into the repo, and `wrangler.staging.toml` has a comment admitting *"Use production KV — staging shares data for now."* Staging traffic can pollute/corrupt production license, idempotency, and catalog-cache data. Split staging onto its own KV/D1 resources.
- **Dead code:** `converse/shared.ts`'s `streamAI()` and `pocket/converse.ts`'s `streamFreeChat()` are fully implemented but never imported/called anywhere in the codebase (confirmed via repo-wide grep). Either wire them in or remove them.
- `tools/handlers/workspace.ts`'s `EXT_TO_LANG` map defines the `ts` key twice (lines 6 and 9) — harmless but a lint signal.
- `tools/handlers/workspace.ts`'s `summarizeToolOutput()` regex has a duplicated identical alternative: `/pass|ok|success|✓|✓/i` — almost certainly meant to be two different glyphs (e.g. `✓|✔`), currently a no-op duplicate.
- `tools/helpers.ts` opens with `const { pickModel, pickReasoningModel } = await import("../ai/models.js");` — a **dynamic, top-level-awaited import of a local relative file**. There's no reason this can't be a normal static `import`; as written it adds avoidable latency to every cold start, and `pickReasoningModel` ends up unused in this file (see #9).
- 36 `catch {}` / `catch (e) {}` blocks across the codebase swallow errors with no logging — several are appropriate ("best effort" KV writes), but a handful (e.g. in `account/billing.ts`'s license-scan fallback, `converse/index.ts`'s memory persistence) hide failures that would be useful to know about in production.
- 469 occurrences of `: any` / `as any` across the codebase. The project's own generated-code linter (`tools/helpers.ts` `LANG_DIAGNOSTICS.typescript.type`) flags `:\s*any\b` as a warning ("Implicit `any` type — use specific type annotation") for code it generates for *customers* — the backend doesn't hold itself to the same bar.
- `converse/architecture.ts`'s `composeArchitecture()` calls `detResolve(...)` (imported from the external `@friehub/blueprint` package) without `await`, while the local same-named function in `catalog/resolver.ts` is `async`. The external package's source isn't included in this bundle so this can't be confirmed either way — but it's worth explicitly verifying whether the published `resolveArchitecture` export is synchronous. If it isn't, `detResult` would be a `Promise` object, `detResult?.modules?.length` would always be `undefined`, and the deterministic resolver path would silently never fire, falling through to the slower/costlier LLM-based fallback on every call.
