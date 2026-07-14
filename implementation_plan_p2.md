# Frensense Engine Expansion — Implementation Plan (Part 2 of 3)
# Categories: Sensitive Data Exposure | Header Trust & CORS

---

## Phase 3 — Category 3: Sensitive Data Exposure

**Source audit files:** `AGENT_AUDIT.md` AGENT-001/AGENT-002, `codebase_audit.md`
BUG-05/SEC-06/SEC-07, `frontend-audit.md` finding #5/#6, `backend-audit.md` finding #18.

---

### 3.1 — ts_secret_in_cli_args (EXTEND existing)

**Status:** Positive and negative files already exist (`ts_secret_in_cli_args_*`).
They cover the basic token-in-template-string pattern. We need a second positive
and second negative covering the `sandboxExec` / shell-wrapper variant from
AGENT-002 where the token appears in a multi-line backtick command string passed
to a sandbox execution helper rather than directly to `exec`.

**New corpus files to add:**
- `ts_secret_in_cli_args_positive2.ts` — token interpolated into a sandboxExec
  shell command; includes the case where the token ends up in agent result_data
  stored to D1.
- `ts_secret_in_cli_args_negative2.ts` — token injected as an env var at sandbox
  container creation time, not as a CLI argument; deploy command uses no `--token`
  flag in the command string itself.

**Add `.toml` override:**
```
observation = "Secret token interpolated into a shell command string — exposed in process list, shell history, LLM context, and stored results."
impact = "Any process in the sandbox can read the token from ps aux. Shell history persists the token. If the command output is stored (e.g., in D1 result_data), the token is logged permanently."
improvement = "Pass secrets as environment variables injected at sandbox creation time. Never include tokens in CLI argument strings."
```

---

### 3.2 — ts_token_in_memory_state

**Pattern:** After an OAuth exchange or auth API call, the session token is
extracted from the JSON response and stored in a module-level or global JS
state object. This makes it accessible via XSS (`window.state.sessionToken`).
The correct pattern is to rely entirely on the HttpOnly cookie set by the
server and never read the raw token in JavaScript.

**Audit bugs covered:** `codebase_audit.md` BUG-05/SEC-06, `frontend-audit.md`
finding #6.

**New corpus files:**
- `ts_token_in_memory_state_positive.ts` — async function performs
  `fetch('/auth/exchange')`, destructures `{ sessionToken }` from
  `response.json()`, and assigns `state.sessionToken = sessionToken` and/or
  `localStorage.setItem('session', sessionToken)`.
- `ts_token_in_memory_state_negative.ts` — same exchange call uses
  `credentials: 'include'`; does not destructure or store the token; no
  assignment to `localStorage` or global state for token-named variables.
- `ts_token_in_memory_state.toml` — advisory text.

**Advisory text:**
```
observation = "Session token extracted from auth response and stored in JS memory or localStorage."
impact = "Token is accessible to any script on the page origin. One XSS payload exfiltrates the token and achieves full account takeover across all sessions."
improvement = "Rely on the HttpOnly cookie set by the server. Remove the X-Session-Token header from all client requests. Never store raw session tokens in JavaScript."
```

---

### 3.3 — ts_error_message_leak

**Pattern:** A catch block captures an exception and returns the raw
`e.message` or `String(e)` directly inside an API response body. This leaks
internal error details (database error text, stack-adjacent info, library
internals) to the client.

**Audit bugs covered:** `backend-audit.md` finding #18, `frontend-audit.md`
finding #11.

**New corpus files:**
- `ts_error_message_leak_positive.ts` — top-level catch that builds a
  `Response.json({ error: 'internal_error', message: e.message })` or
  `res.json({ error: msg })` where `msg` comes from `e.message` with no
  sanitization layer.
- `ts_error_message_leak_negative.ts` — catch logs `e.message` server-side
  via `console.error` and returns a fixed generic response object with no
  dynamic string from the exception.
- `ts_error_message_leak.toml` — advisory text.

**Advisory text:**
```
observation = "Raw exception message returned verbatim to API client."
impact = "Internal error details (database schema, file paths, library internals) are exposed to attackers, reducing the cost of targeted exploitation."
improvement = "Log the full exception server-side. Return a static generic error string to the client. Never include e.message in response bodies."
```

---

### 3.4 — ts_raw_credential_in_response

**Pattern:** A database record containing a sensitive credential (session token,
API key, raw password hash) is mapped directly into an API response without
transformation. The correct pattern is to return an opaque derivative (e.g.,
`sha256(token).slice(0,16)`) that can serve as a stable handle for operations
like session revocation without exposing the raw credential.

**Audit bugs covered:** `codebase_audit.md` SEC-07, `AGENT_AUDIT.md` AGENT-002
(token in stored result_data).

**Relationship to existing rules:** This extends `CROSS_FILE_TAINT` with
`TaintOrigin::Database`. The corpus rule here trains the *intra-procedural*
pattern where the DB result field name contains `token`, `secret`, `key`, or
`password` and is assigned directly into a response object.

**New corpus files:**
- `ts_raw_credential_in_response_positive.ts` — function queries a Session or
  ApiKey table, maps `r.token` directly as a field in the response array, and
  returns `res.json(sessions)` with no hashing or masking.
- `ts_raw_credential_in_response_negative.ts` — same query, but maps the token
  through a hash: `id: sha256(r.token).slice(0, 16)`, returning the opaque
  handle. The full raw token is never present in the returned object.
- `ts_raw_credential_in_response.toml` — advisory text.

**Advisory text:**
```
observation = "Raw database credential (token, API key) returned directly in API response body."
impact = "Any XSS payload or compromised client can call the session list endpoint and collect all active raw session tokens — full account takeover for every active device."
improvement = "Return an opaque hash of the credential as the client-facing identifier. The delete/revoke endpoint accepts the hash, never the raw value."
```

---

### 3.5 — ts_incomplete_resource_deletion (EXTEND existing)

**Status:** Rule exists. Extend with a second positive covering the specific
pattern from `AGENT_AUDIT.md` AGENT-006: D1 rows are hard-deleted by a cron
but the corresponding R2 object key is never deleted. Existing rule covers
soft-delete state flag not cleared; new variant covers the cross-storage-system
deletion gap.

**Add `ts_incomplete_resource_deletion_positive2.ts`** — a cron/cleanup
function that runs `DELETE FROM project_files WHERE deleted_at < ?` but never
fetches the `content` field (which contains `r2://` keys) and never calls
`env.WORKSPACE_FILES.delete(key)` before the DB delete.

**Add `ts_incomplete_resource_deletion_negative2.ts`** — same cleanup function
that first SELECTs expired rows with `content LIKE 'r2://%'`, iterates and
calls `env.WORKSPACE_FILES.delete(key)` for each, then executes the DELETE.

---

## Phase 4 — Category 4: Header Trust & CORS

**Source audit files:** `codebase_audit.md` SEC-01/SEC-05, `backend-audit.md`
finding #16, `frontend-audit.md` finding #10.

---

### 4.1 — ts_open_redirect (EXTEND existing)

**Status:** Rule exists (`ts_open_redirect_positive.ts`). Current positive
covers a simple `res.redirect(req.query.url)` pattern. We need to extend it
with the `X-Forwarded-Host` header variant from SEC-05 — where the client
controls the redirect destination via a request header rather than a query
parameter.

**Add `ts_open_redirect_positive2.ts`** — a `getGoogleRedirectUri` function
that calls `request.headers.get('X-Forwarded-Host')` and, if truthy, builds
and returns a `https://${forwardedHost}/auth/callback` URL without validating
the host against an allowlist.

**Add `ts_open_redirect_negative2.ts`** — same function but with an explicit
`VALID_HOSTS` allowlist check; returns the safe default origin when the header
is absent or not in the allowlist.

**Engine change:** Extend the taint seeder in `runner.rs` to treat
`request.headers.get(...)` call expressions as `TaintOrigin::UserInput` seeds.
Currently only `req.params`, `req.query`, `req.body`, and `event.data` are
seeded. Adding `headers.get` unlocks this entire class of header-injection
vulnerabilities through the existing cross-file taint pipeline.

---

### 4.2 — ts_cors_wildcard_credentials (EXTEND existing)

**Status:** Positive and negative files exist. Review the existing positive to
confirm it covers both the object-literal form (`{ "Access-Control-Allow-Origin": "*", ... }`)
and the function-return form (`return { ... "Access-Control-Allow-Origin": allow ... }` where
`allow` is `"*"` when no origin matches). If the function-return form is not
covered, add `ts_cors_wildcard_credentials_positive2.ts` covering it.

**Add `ts_cors_wildcard_credentials_positive2.ts`** — a `makeCors(origin)` function
where `const allow = ALLOWED_ORIGINS.includes(origin) ? origin : '*'` and the
returned object unconditionally includes `"Access-Control-Allow-Credentials": "true"`,
meaning the wildcard fallback is set alongside credentials.

**Add `ts_cors_wildcard_credentials_negative2.ts`** — same function where the
fallback is the first allowed origin rather than `'*'`, and `Allow-Credentials`
is only included when `allow !== '*'`.

---

### 4.3 — ts_hardcoded_privilege_in_token (EXTEND existing)

**Status:** Rule exists. Add a second positive variant covering the specific
pattern from `backend-audit.md` finding #4: a login handler that mints a
session token with `tier: 'free'` hardcoded in the KV record payload,
regardless of the user's actual subscription tier in the database.

**Add `ts_hardcoded_privilege_in_token_positive2.ts`** — a `handleLogin` or
`handleGoogleCallback` function that calls `env.KV.put('tok:' + token, JSON.stringify({ customerId: acct.customerId, tier: 'free' }))` with
the tier literal hardcoded, not derived from a DB lookup.

**Add `ts_hardcoded_privilege_in_token_negative2.ts`** — same handler that
queries the license/subscription table before minting the token and stores the
real tier value from the database result.

---

## Phase 3 & 4 — Engine Changes Required

### runner.rs — Header Source Seeding

Add `request.headers.get` to the taint source seeder. In the AST walker
section that identifies `TaintOrigin::UserInput` function entry points, add a
secondary scan: if the function body contains any `call_expression` whose
callee text ends with `.headers.get`, mark that function as an additional
`UserInput` taint source for the cross-file resolver.

This is a 10-line addition to the existing DB-read seeder block added in the
previous session. No architectural changes needed.

### cross_file_taint.rs — Response Body Sinks

Add `res.json`, `c.json`, `context.json`, `Response.json`, and `.send` to
the sink list checked against `TaintOrigin::Database` paths. Currently the
sink check only tests for `eval`, `exec`, `query`, and `send`. Adding JSON
response methods means a DB-read token flowing into a response body is caught
by the existing resolver without any new infrastructure.

---

## Phase 3 & 4 — Benchmark Expectations

| New Pattern | Estimated TP gain | FP risk |
|---|---|---|
| `ts_secret_in_cli_args` extension | +1 TP | Low |
| `ts_token_in_memory_state` | +1 TP | Medium — localStorage usage is common |
| `ts_error_message_leak` | +2 TP | Medium — any catch-return touches this |
| `ts_raw_credential_in_response` | +1 TP | Low — token field name heuristic |
| `ts_incomplete_resource_deletion` ext | +1 TP | Low — already tuned rule |
| `ts_open_redirect` header ext + engine | +2 TP | Low — header.get + URL sink |
| `ts_cors_wildcard_credentials` ext | +1 TP | Low — already tuned rule |
| `ts_hardcoded_privilege_in_token` ext | +1 TP | Low — literal 'free' in KV.put |

**Projected post-Phase-4 recall: 86–88% (32–33/37 files)**

---

*Continue in implementation_plan_p3.md — Categories 5, 6, 7 (Missing Guards,
Financial Logic, Client-Side XSS) + Full Engine Upgrade Roadmap*
