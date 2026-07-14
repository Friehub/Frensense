# Pocket Workspace Agent — Audit Report & Solidification Guide

**Scope:** Agent run workflow · LLM gateway · Tool executor · Sandbox layer · OpenRouter migration
**Auditor:** Claude (Anthropic) — July 2026
**Goal:** Make the cloud coding agent production-solid before any new features ship.

---

## Severity Key

| Level | Meaning |
|---|---|
| **P0** | Production security breach or data loss happening right now |
| **P1** | Broken feature or missing table — users experience silent failures |
| **P2** | Stability / reliability risk |
| **P3** | Architecture smell that will cause pain at scale |

---

## P0 — Stop. Fix These Now.

---

### AGENT-001 · `sandbox-client.ts` sends `VERCEL_ACCESS_TOKEN` to a third-party URL

**File:** `src/core/sandbox-client.ts`

```ts
const SANDBOX_BRIDGE_URL = "https://vercel-sandbox-bridge.vercel.app/api/sandbox";
// ...
if (token) headers["Authorization"] = "Bearer " + token; // ← YOUR token, their server
const res = await fetch(SANDBOX_BRIDGE_URL, { ... body: JSON.stringify({ action: "exec", cmd }) });
```

Every `sandboxExec`, `sandboxWriteFile`, `sandboxReadFile` call in the **legacy agent path** sends:
- Your `VERCEL_ACCESS_TOKEN` in the `Authorization` header
- The full shell command (including any user-supplied content) to `vercel.app`

If `vercel-sandbox-bridge.vercel.app` is not a domain you own and control, this is an active credential leak and a full remote code execution vector — whoever owns that app can run arbitrary commands as your Vercel account.

**Fix:**
1. Rotate the `VERCEL_ACCESS_TOKEN` immediately.
2. If this is your own app, move `SANDBOX_BRIDGE_URL` to an env var: `SANDBOX_BRIDGE_URL` in wrangler secrets.
3. Audit which code paths still use the legacy `sandboxExec` from `sandbox-client.ts` vs the new `SandboxBridge` class (daemon path). The daemon path in `agent-run-workflow.ts` does NOT use `sandbox-client.ts` — it uses `SandboxBridge` directly. The legacy executor in `agent-tool-executor.ts` DOES use it.

---

### AGENT-002 · `workspace_deploy` exposes `VERCEL_ACCESS_TOKEN` as a CLI argument

**File:** `src/workspace/agent/agent-tool-executor.ts`

```ts
result = await sandboxExec(env,
  `cd /workspace && npx --yes vercel deploy --yes \
   --token="${(env as any).VERCEL_ACCESS_TOKEN || ""}" 2>&1 | tail -1`
);
```

The token appears in:
- **The process list** — any process inside the sandbox can run `ps aux` and read it
- **Shell history** — `~/.bash_history` in the sandbox
- **The result string** — the output is saved to `agent_runs.result_data` in D1
- **The agent's memory** — the LLM may echo it back in its next message

**Fix:** Use a Vercel deployment token scoped to only this project, passed via an environment variable inside the sandbox rather than as a CLI argument:

```bash
# Inject as env var inside sandbox; CLI picks it up automatically
export VERCEL_TOKEN="..." && npx vercel deploy --yes
```

Or better: call the Vercel REST API directly from the Worker (not the sandbox) after collecting the files to deploy.

---

### AGENT-003 · `/agent/run` Hono route bypasses quota check entirely

**File:** `src/workspace/routes/agent-router.ts`

```ts
agentRouter.post('/run', async (c) => {
  // Creates workflow directly — no checkAndConsumeQuota call
  await c.env.AGENT_RUN_WORKFLOW.create({ id: runId, params: { ... } });
  return c.json({ data: { runId } }, 201);
});
```

The `handleAgentRun` function in `agent-run-routes.ts` calls `checkAndConsumeQuota` and checks entitlement limits. The Hono router's `/agent/run` bypasses all of this. Any authenticated user can call `POST /agent/run` unlimited times at no credit cost.

**Fix:** Move all agent run dispatch behind `handleAgentRun`. The router should call the handler, not duplicate the logic:

```ts
agentRouter.post('/run', (c) => handleAgentRun(c.req.raw, c.env));
```

---

### AGENT-004 · `search_index` and `agent_context` tables referenced but never created

**Files:** `src/workspace/agent/context.ts`

`injectFileContext` queries:
```ts
const { results } = await db.prepare(
  "SELECT file_path FROM search_index WHERE project_id = ? AND file_path LIKE ? LIMIT 3"
).bind(projectId, ...).all();
```

`saveContext` and `loadContext` query:
```ts
db.prepare(`INSERT INTO agent_context (project_id, summary, ...) VALUES (...)`)
db.prepare("SELECT summary, files_touched, decisions FROM agent_context WHERE project_id = ?")
```

**Neither `search_index` nor `agent_context` exist in any migration file.** Every call silently swallows the D1 error (caught by `try/catch`). The agent has no context memory between runs — every run starts from scratch even when "resuming" a project.

**Fix:** Create the migrations:

```sql
-- Migration 0015_agent_context.sql
CREATE TABLE IF NOT EXISTS agent_context (
  project_id TEXT PRIMARY KEY,
  summary    TEXT,
  files_touched TEXT,
  decisions  TEXT,
  updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS search_index (
  project_id TEXT NOT NULL,
  file_path  TEXT NOT NULL,
  content    TEXT,
  language   TEXT,
  indexed_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (project_id, file_path)
);
CREATE INDEX idx_search_index_project ON search_index(project_id);
```

---

### AGENT-005 · `agent_preferences` column named `name` in schema, `key` in all queries

**Migration:** `0003_complete_schema.sql`:
```sql
CREATE TABLE IF NOT EXISTS agent_preferences (
  project_id TEXT NOT NULL,
  name TEXT NOT NULL,   ← column is 'name'
```

**Code:** `src/workspace/data/workspace.ts`:
```ts
await db(env!, projectId)
  .prepare("INSERT OR REPLACE INTO agent_preferences (project_id, key, value, ...) VALUES (?, ?, ?, ...)")
  // ← 'key' doesn't exist → D1 error → silent catch
```

Every agent preference save (model choice, write mode, etc.) silently fails. Users can't customize their agent — every run uses defaults.

**Fix:** Create a migration to rename the column:

```sql
-- Migration 0016_fix_agent_preferences_column.sql
ALTER TABLE agent_preferences RENAME COLUMN name TO key;
```

---

### AGENT-006 · Cron hard-deletes `project_files` rows but NOT the R2 objects

**File:** `src/workspace/agent/cron-handler.ts`

```ts
await db.prepare("DELETE FROM project_files WHERE deleted_at < datetime('now', '-30 days')").run();
// ← R2 objects at {projectId}/files/{path} are never deleted
```

Files soft-deleted 30+ days ago get their D1 rows removed but their R2 content stays forever. After a year, this becomes a significant storage bill and a GDPR violation (user-deleted files still stored).

**Fix:**
```ts
// Before the DELETE, fetch and delete R2 objects
const { results: expiredFiles } = await db.prepare(
  "SELECT project_id, path, content FROM project_files WHERE deleted_at < datetime('now', '-30 days') AND content LIKE 'r2://%'"
).all<{ project_id: string; path: string; content: string }>();

for (const f of expiredFiles) {
  const key = f.content.replace("r2://", "");
  await env.WORKSPACE_FILES.delete(key).catch(() => {});
}

await db.prepare("DELETE FROM project_files WHERE deleted_at < datetime('now', '-30 days')").run();
```

---

## P1 — Broken Behavior Users Can See

---

### AGENT-007 · `runLegacy` ignores the user's chosen model — hardcoded to `"zen-3-pro"`

**File:** `src/workspace/agent/agent-run-workflow.ts`

```ts
const loopPromise = runUnifiedToolLoop(this.env, projectId, systemPrompt, ..., {
  model: "zen-3-pro",   // ← hardcoded, ignores event.payload.model
```

Users selecting a different model get `zen-3-pro` regardless. The daemon path correctly passes `event.payload.model`.

**Fix:**
```ts
model: event.payload.model || "deepseek-v4-flash",
```

---

### AGENT-008 · `/agent/stop` calls `pause` instead of stopping the workflow

**File:** `src/workspace/routes/agent-router.ts`

```ts
agentRouter.post('/stop', async (c) => {
  await workflowAction(c, 'pause');  // ← 'pause', not 'terminate' or 'cancel'
```

Clicking "Stop run" pauses it, not stops it. The user's next interaction may resume a paused run unexpectedly.

**Fix:** Use the Workflow `terminate` method if available, or use the `abort` signal through the queue. At minimum rename to match the actual behavior:
```ts
agentRouter.post('/stop', async (c) => {
  await workflowAction(c, 'terminate'); // or 'abort'
```

---

### AGENT-009 · `workflowAction` doesn't verify the run belongs to the authenticated user

**File:** `src/workspace/routes/agent-router.ts`

```ts
async function workflowAction(c: any, action: 'pause' | 'resume') {
  const body = await c.req.json();
  const wf = c.env.AGENT_RUN_WORKFLOW;
  if (wf && body.run_id) {
    const inst = await wf.get(body.run_id);
    await inst[action]();  // ← any authenticated user can pause ANY run
  }
}
```

User A can pause/resume User B's agent run by knowing the `run_id`. Run IDs are UUIDs but they're returned in API responses and may appear in URLs.

**Fix:** Store `owner_id` in the workflow params and verify before any action:
```ts
const inst = await wf.get(body.run_id);
const status = await inst.status();
if (status.output?._customerId !== c.get('session').customerId) {
  return c.json(makeError('Forbidden', 'not_your_run', requestId()), 403);
}
```

---

### AGENT-010 · `workspace_preview` returns `http://localhost:{port}` — inaccessible to user's browser

**File:** `src/workspace/agent/agent-tool-executor.ts`

```ts
return { status: "started", port, url: `http://localhost:${port}`, tip: "..." };
```

The user's browser cannot reach `http://localhost:5173` inside a Vercel sandbox. This tool gives the agent a false result and the agent tells the user a URL that doesn't work.

**Fix:** Return the sandbox's public tunnel URL. The Vercel sandbox daemon should provide a public URL for the dev server. Until then, return an honest error:
```ts
return { status: "started", port, internalUrl: `http://localhost:${port}`,
  note: "Dev server started inside sandbox. Use the preview tunnel URL from the sandbox status to access it." };
```

---

### AGENT-011 · `quota.ts` fails open on any error — unlimited free agent runs

**File:** `src/core/quota.ts`

```ts
} catch (e: any) {
  console.error("checkAndConsumeQuota failed:", e);
  return { allowed: true, remaining: 999 }; // ← "for dev purposes"
}
```

The comment says "for dev purposes" but this is in production code. If friehub-core is temporarily unreachable (deploy, restart, rate limit), all users get unlimited free agent runs.

**Fix:** Fail closed in production. Use `ENVIRONMENT` env var:
```ts
} catch (e: any) {
  console.error("checkAndConsumeQuota failed:", e);
  if (env.ENVIRONMENT === "development") return { allowed: true, remaining: 999 };
  return { allowed: false, reason: "quota_service_unavailable" };
}
```

---

### AGENT-012 · `compactMessages` produces invalid tool message sequences

**File:** `src/workspace/agent/context.ts`

```ts
const tail = messages.slice(-keepLast);  // ← may start mid-turn
```

If the tail slice begins with a `tool` role message, there's no preceding `assistant` message with `tool_calls`. Most LLM APIs (including Claude via OpenRouter) reject this with a validation error.

**Fix:** Trim the tail to start at a clean turn boundary:
```ts
// Find the first 'user' or 'assistant' message in the tail
const cleanStart = tail.findIndex(m => m.role === 'user' || (m.role === 'assistant' && !m.tool_calls));
const cleanTail = cleanStart > 0 ? tail.slice(cleanStart) : tail;
return [...head, { role: "user", content: summary.join("; ") }, ...cleanTail];
```

---

### AGENT-013 · `CANVAS_DO` binding declared in `Env` type but missing from `wrangler.toml`

**File:** `src/platform/types.ts` declares `CANVAS_DO: DurableObjectNamespace`
**File:** `wrangler.toml` — no `CANVAS_DO` binding

Any code that accesses `env.CANVAS_DO` will throw a runtime error.

**Fix:** Add to `wrangler.toml`:
```toml
[[durable_objects.bindings]]
name = "CANVAS_DO"
class_name = "CanvasDO"

[[migrations]]
tag = "v4"
new_sqlite_classes = ["CanvasDO"]
```

---

### AGENT-014 · `workspace_sandbox_status` creates a real sandbox just to check health

**File:** `src/workspace/agent/agent-tool-executor.ts`

```ts
if (toolName === "workspace_sandbox_status") {
  const provider = new VercelSandboxProvider(env);
  const session = await provider.createSession(...);  // ← creates a real billed sandbox
  await provider.destroySession(session);
  return { status: "healthy", ... };
}
```

Calling this tool creates and destroys a billable Vercel sandbox. The LLM calls this whenever it wants to confirm the environment is running.

**Fix:** Remove this tool from the active tool list. The daemon `runWithDaemon` path already performs a health check (`/ping`). Replace with a passive status check that reads from the DO session store:
```ts
if (toolName === "workspace_sandbox_status") {
  const doId = env.CONVERSATION_DO.idFromName(projectId);
  const stub = env.CONVERSATION_DO.get(doId);
  const res = await stub.fetch("http://do/sandbox");
  if (res.ok) {
    const { session } = await res.json() as any;
    return { status: session ? "active" : "cold", sandboxId: session?.id };
  }
  return { status: "unknown" };
}
```

---

## P2 — Stability & Reliability Issues

---

### AGENT-015 · Shell injection via `workspace_git_commit` message

**File:** `src/workspace/agent/agent-tool-executor.ts`

```ts
await sandboxExec(env,
  `cd /workspace && git add -A && git commit -m "${(args.message || '').replace(/"/g, '\\"')}"`
);
```

Escaping only double quotes. Backticks, `$()`, newlines, and single quotes in the commit message will execute as shell commands.

Example: `args.message = "feat: done $(cat /etc/passwd)"` → runs `cat /etc/passwd`.

**Fix:** Use `--` and pass the message via `stdin` or use a JSON config:
```ts
const safeMessage = (args.message || 'agent commit')
  .replace(/[`$\\]/g, '\\$&')  // escape shell metacharacters
  .replace(/\n/g, ' ')
  .slice(0, 200);
await sandboxExec(env, `cd /workspace && git add -A && git commit -m '${safeMessage}'`);
```

Or better, write the commit message to a temp file:
```ts
await sandboxExec(env, `cd /workspace && printf '%s' ${JSON.stringify(args.message)} > /tmp/commit_msg && git add -A && git commit -F /tmp/commit_msg`);
```

---

### AGENT-016 · Module-level caches are unreliable and shared across users

**File:** `src/workspace/chat/models.ts`

```ts
let _cachedModels: ModelDef[] | null = null;
let _cacheTime = 0;
```

**File:** `src/core/llm-gateway.ts`

```ts
const circuitBreakers = new Map<string, CircuitBreaker>();
const activeRequests = new Map<string, AbortController>();
```

Cloudflare Worker isolates don't guarantee lifetime. An isolate can be replaced mid-traffic. Module-level state:
- Provides no cache guarantee (may always be cold)
- Means a slow `fetchAvailableModels` call blocks all concurrent requests
- `activeRequests` abort controllers may be stale from previous requests

**Fix for model cache:** Move to KV with a short TTL:
```ts
export async function fetchAvailableModels(env: any): Promise<ModelDef[]> {
  const cached = await env.workspace_db.prepare(
    "SELECT value FROM system_config WHERE key = 'ai_models_cache'"
  ).first();
  if (cached && /* check timestamp */) return JSON.parse(cached.value);
  // ... fetch and store
}
```

Circuit breakers at module level are acceptable (they reset anyway), but document the limitation.

---

### AGENT-017 · Warm sandbox reuse always falls through — `reused` flag never set to `true`

**File:** `src/workspace/agent/agent-run-workflow.ts`

The code fetches the existing session from the DO and pings it for health. If healthy, it logs "reusing warm sandbox" — but then falls through to `tryProvider()` anyway and creates a new sandbox:

```ts
try {
  const res = await stub.fetch("http://do/sandbox");
  // ... checks health, logs "Reusing warm sandbox environment..."
  // But there's no `return` here — falls through to tryProvider()
} catch (e) { /* ignore */ }

const tryProvider = async (...) => { /* always creates new sandbox */ };
const result = await tryProvider(...);  // ← always runs
```

Every agent run creates a new sandbox even when a warm one exists.

**Fix:** Return early when the existing session is healthy:
```ts
if (healthRes.ok) {
  await pInstance.keepAlive(existingSession);
  await stub.fetch("http://do/sandbox", { method: "PUT", body: JSON.stringify({ session: existingSession }) });
  return {
    sandboxId: existingSession.id,
    accessToken: existingSession.accessToken,
    isVercel: existingSession.provider === "vercel",
    daemonUrl: dUrl,
    reused: true,  // ← set this
  };
}
```

---

### AGENT-018 · Timeout mismatch — tool loop timeout (15 min) > Workflow step timeout

**File:** `src/workspace/agent/agent-run-workflow.ts`

```ts
const timeoutPromise = new Promise<never>((_, reject) =>
  setTimeout(() => reject(new Error("run_timeout")), 900_000) // 15 min
);
```

Cloudflare Workflows have a step timeout (configurable, default 10 minutes for `step.do`). If the tool loop runs for 12 minutes and the step times out at 10, the Workflow retries the entire `run-tool-loop` step, creating duplicate tool calls, duplicate file writes, and duplicate LLM charges.

**Fix:** Set the inner timeout to be safely below the step timeout. Use 8 minutes for the daemon path:
```ts
const timeoutPromise = new Promise<never>((_, reject) =>
  setTimeout(() => reject(new Error("run_timeout")), 480_000) // 8 min
);
```

And add a step timeout in wrangler config or the workflow definition.

---

### AGENT-019 · DSML parser is active dead code for non-DeepSeek models

**File:** `src/workspace/agent/ai-engine.ts`

```ts
function parseDSML(content: string): { cleanContent: string; toolCalls: any[] } {
  if (!content.includes("<｜｜DSML｜｜tool_calls>")) return { cleanContent: content, toolCalls };
  // ... parses DeepSeek-specific format
```

This parser only triggers on `｜｜DSML｜｜` (fullwidth vertical bars, U+FF5C), a format unique to some Chinese LLMs. Claude via OpenRouter uses standard JSON tool calls. The code is harmless but runs on every LLM response.

It also runs after the standard tool call extraction, meaning if a model uses DSML format, its calls are double-parsed.

**Fix:** Gate it behind a model family check:
```ts
const isDeepSeekFamily = model.includes("deepseek") || model.includes("mimo") || model.includes("glm");
if (isDeepSeekFamily) {
  const dsmlParsed = parseDSML(content);
  // ...
}
```

---

### AGENT-020 · `workspace_preview` starts a dev server but never stops it

**File:** `src/workspace/agent/agent-tool-executor.ts`

```ts
sandboxExec(env, `cd /workspace && npx vite --port ${port} --host 0.0.0.0 &`).catch(() => {});
```

The dev server runs in the background indefinitely. On the next agent run reusing the warm sandbox:
- Port `5173` is already occupied
- `vite` throws `EADDRINUSE`
- The agent tries again on the same port

**Fix:** Add a stop mechanism and check before starting:
```ts
await sandboxExec(env, `fuser -k ${port}/tcp 2>/dev/null; cd /workspace && npx vite --port ${port} --host 0.0.0.0 &`);
```

---

## Part 2: OpenRouter Integration

### What changes when you add OpenRouter

The `LLMGateway` already uses an OpenAI-compatible format. OpenRouter is compatible. These are the specific changes needed.

---

### Step 1 — Add env vars

**`wrangler.toml`** (use secrets, not vars):
```bash
wrangler secret put OPENROUTER_API_KEY
# Set to: sk-or-v1-...
```

**`src/platform/types.ts`** — add to `Env` interface:
```ts
OPENROUTER_API_KEY?: string;
AI_GATEWAY_URL?: string;     // already exists
AI_MODELS_URL?: string;      // already exists
```

---

### Step 2 — Update `LLMGateway.executeFetch` to support both providers

**File:** `src/core/llm-gateway.ts`

```ts
private async executeFetch(model: string, messages: ChatMessage[], options: ChatOptions, reqId: string): Promise<Response> {
  // Detect provider from model string
  const isOpenRouter = model.startsWith("anthropic/") || model.startsWith("openai/") ||
                       model.startsWith("google/") || model.startsWith("meta-llama/") ||
                       model.includes("/");  // OpenRouter models always have a slash

  const api = isOpenRouter
    ? "https://openrouter.ai/api/v1/chat/completions"
    : (this.env.AI_GATEWAY_URL || "https://opencode.ai/zen/go/v1/chat/completions");

  const apiKey = isOpenRouter
    ? (this.env.OPENROUTER_API_KEY || "")
    : this.apiKey;  // existing OPENCODE_ZEN_API_KEY

  const headers: Record<string, string> = {
    "Authorization": `Bearer ${apiKey}`,
    "Content-Type": "application/json",
    "cf-aig-metadata": JSON.stringify({ userId: this.customerId || "anonymous" }),
  };

  // OpenRouter-specific required headers
  if (isOpenRouter) {
    headers["HTTP-Referer"] = "https://pocket.friehub.cloud";
    headers["X-Title"] = "Pocket Workspace";
  }

  const reqBody: any = {
    model,
    max_tokens: options.max_tokens || 8192,
    messages,
    ...(options.tools && options.tools.length > 0 ? { tools: options.tools } : {}),
    ...(options.stream ? { stream: true } : {}),
  };

  const controller = new AbortController();
  activeRequests.set(reqId, controller);
  try {
    return await fetch(api, { method: "POST", headers, body: JSON.stringify(reqBody), signal: controller.signal });
  } finally {
    activeRequests.delete(reqId);
  }
}
```

---

### Step 3 — Add Claude models to `system_config`

Update the `ai_models` value in D1 via the admin API or a migration:

```sql
-- Migration: add Claude models to system_config
UPDATE system_config
SET value = json_patch(value, '{
  "context_windows": {
    "anthropic/claude-sonnet-4-5": 200000,
    "anthropic/claude-opus-4":     200000,
    "anthropic/claude-haiku-4-5":  200000
  },
  "reasoning_models": ["anthropic/claude-opus-4", "deepseek-v4-pro"]
}')
WHERE key = 'ai_models';
```

Or replace via admin API:
```http
POST /admin/config
{ "key": "ai_models", "value": "{ ... merged config ... }" }
```

---

### Step 4 — Update `resolveModel` fallback order

**File:** `src/workspace/chat/models.ts`

```ts
export async function resolveModel(env: any, preferred?: string): Promise<{ api: string; model: string; contextWindow: number }> {
  const available = await fetchAvailableModels(env);
  const found = preferred ? available.find(m => m.id === preferred) : null;

  // Prefer Claude Sonnet as default if OpenRouter key is set
  const defaultModel = env.OPENROUTER_API_KEY
    ? available.find(m => m.id === "anthropic/claude-sonnet-4-5") || available[0]
    : available.find(m => !m.reasoning) || available[0];

  const best = found || defaultModel;

  // API URL is determined per-model inside LLMGateway now, not here
  return {
    api: "",  // LLMGateway chooses the right endpoint
    model: best?.id || "deepseek-v4-flash",
    contextWindow: best?.contextWindow || 32_000,
  };
}
```

---

### Step 5 — Fetch available models from OpenRouter

**File:** `src/workspace/chat/models.ts`

Add OpenRouter to the model list endpoint:

```ts
export async function fetchAvailableModels(env: any): Promise<ModelDef[]> {
  // ... existing local config load ...

  // Fetch from both providers in parallel
  const [localModels, openRouterModels] = await Promise.allSettled([
    fetchFromLocalGateway(env, modelsUrl, env.OPENCODE_ZEN_API_KEY),
    env.OPENROUTER_API_KEY ? fetchFromOpenRouter(env.OPENROUTER_API_KEY) : Promise.resolve([]),
  ]);

  const all = [
    ...(localModels.status === "fulfilled" ? localModels.value : []),
    ...(openRouterModels.status === "fulfilled" ? openRouterModels.value : []),
  ];

  _cachedModels = all.length > 0 ? all : [...FALLBACK_MODELS];
  _cacheTime = Date.now();
  return _cachedModels;
}

async function fetchFromOpenRouter(apiKey: string): Promise<ModelDef[]> {
  const res = await fetch("https://openrouter.ai/api/v1/models", {
    headers: { "Authorization": `Bearer ${apiKey}` }
  });
  if (!res.ok) return [];
  const data = await res.json() as any;
  const CLAUDE_MODELS = ["anthropic/claude-sonnet-4-5", "anthropic/claude-opus-4", "anthropic/claude-haiku-4-5"];
  return (data.data || [])
    .filter((m: any) => CLAUDE_MODELS.includes(m.id))
    .map((m: any) => ({
      id: m.id,
      label: m.name || m.id,
      reasoning: m.id.includes("opus"),
      contextWindow: m.context_length || 200_000,
    }));
}
```

---

### Step 6 — System prompt when using Claude

Claude performs significantly better with a role-specific system prompt. Replace the minimal legacy prompt when Claude is selected:

```ts
function buildSystemPrompt(model: string, projectContext?: string): string {
  const isClaude = model.startsWith("anthropic/claude");

  if (isClaude) {
    return `You are a senior software engineer working inside a cloud development environment called Pocket Workspace.

## Your Environment
- You have a persistent workspace with the user's code files
- You can read, write, delete files, run shell commands, and search the codebase
- Code you write is immediately saved and can be executed

## How to Work
1. **Understand first**: Read relevant files before making changes. Use workspace_search to find patterns.
2. **Work incrementally**: Write complete, working files. Never use placeholders or "// TODO" unless asked.
3. **Verify your work**: After writing code, run it to confirm it works. Fix errors immediately.
4. **Commit when done**: Use workspace_git_commit with a descriptive message when the task is complete.

## Rules
- Always output complete file content when writing — never partial or truncated files
- Never repeat a failing tool call with the same arguments. Try a different approach.
- If a command fails, read the error carefully before retrying
- When you're done, summarize what you changed and why

${projectContext ? `## Project Context\n${projectContext}` : ""}`;
  }

  // DeepSeek / other models use the existing minimal prompt
  return `You are the Pocket Workspace AI Developer. Fulfill the user's request by reading and writing files.`;
}
```

---

## Part 3: Making It Feel Like Claude.ai Web

The goal is a coding agent that feels as capable as Claude's artifact-based coding experience. Here's what the current architecture needs for that:

### What Claude.ai does that pocket-workspace doesn't yet

| Feature | Claude.ai | Pocket Workspace | Fix |
|---|---|---|---|
| Streaming tokens to UI | Artifacts stream inline | Via SSE/DO (partial) | Already connected via ConversationDO — ensure client reads SSE |
| Live tool execution output | Shows `bash` output as it runs | `workspace_run_command` streams via SSE | Works in daemon path. Legacy path doesn't stream. |
| Multi-file awareness | Sees attached files | Reads project files via tools | Add auto-context injection of `package.json` + entry files on first turn |
| Session continuity | Conversation persists | `agent_context` table (broken — fix AGENT-004) | Fix the missing table |
| Undo / revert | No equivalent | `SagaJournal` rollback exists | Expose via a `workspace_undo_last_run` tool |
| See what changed | Diff UI | Sends diff to client via events | Already implemented in `runWithDaemon` finalize step |
| Install packages | Via Claude's environment | `workspace_run_command` + sandbox | Works in daemon path. Ensure sandbox persists `node_modules`. |
| Deploy | Links to Vercel/Netlify | `workspace_deploy` tool | Fix the token exposure (AGENT-002). Return the real URL. |

### The one architectural addition that closes the gap most

**Streaming tool output to the client.** Claude.ai feels alive because you see bash output in real time. The daemon path already streams via SSE events. The client-side missing piece: the `CONVERSATION_DO` emits events but the client needs to listen to the `/events` SSE endpoint continuously throughout the run and render each `thought` event inline.

Make sure the client:
1. Opens `GET /events?topic={customerId}` as a persistent `EventSource`
2. Renders `type: "thought"` events as streaming text
3. Renders `type: "action"` events as tool call chips (like Claude's tool use blocks)
4. Renders `type: "complete"` events as the final answer

This is already instrumented on the server — the client experience is the remaining gap.

---

## Summary — Fix Order

### Week 1 (P0 — Do before anything else)

| ID | Fix |
|---|---|
| AGENT-001 | Rotate Vercel token. Confirm if sandbox bridge URL is yours. Move to env var. |
| AGENT-002 | Pass `VERCEL_ACCESS_TOKEN` as env var in sandbox, not CLI arg |
| AGENT-003 | Route `/agent/run` Hono handler through `handleAgentRun` for quota check |
| AGENT-004 | Create `agent_context` and `search_index` migrations |
| AGENT-005 | Migration to rename `agent_preferences.name` → `key` |
| AGENT-006 | Delete R2 objects in cron before D1 hard-delete |

### Week 2 (P1 — Broken features)

| ID | Fix |
|---|---|
| AGENT-007 | Pass `payload.model` in `runLegacy` |
| AGENT-008 | Fix stop → terminate, not pause |
| AGENT-009 | Verify run ownership before pause/resume |
| AGENT-010 | Fix `workspace_preview` URL to sandbox tunnel |
| AGENT-011 | Fail closed in `quota.ts` production |
| AGENT-012 | Fix `compactMessages` turn boundary |
| AGENT-013 | Add `CANVAS_DO` to `wrangler.toml` |
| AGENT-014 | Replace `workspace_sandbox_status` with DO-backed health check |

### Week 3 (OpenRouter + Claude)

| Step | Change |
|---|---|
| Step 1 | `wrangler secret put OPENROUTER_API_KEY` |
| Step 2 | Update `LLMGateway.executeFetch` for multi-provider |
| Step 3 | Add Claude models to `system_config` |
| Step 4 | Update `resolveModel` default to Claude Sonnet |
| Step 5 | Add OpenRouter to model fetch |
| Step 6 | Add Claude-specific system prompt |

### Week 4 (P2 — Reliability)

| ID | Fix |
|---|---|
| AGENT-015 | Shell injection in git commit message |
| AGENT-016 | Move model cache from module-level to D1/KV |
| AGENT-017 | Fix warm sandbox reuse early return |
| AGENT-018 | Align tool loop timeout with Workflow step timeout |
| AGENT-019 | Gate DSML parser behind model family check |
| AGENT-020 | Kill existing dev server before starting new one |
