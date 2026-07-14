# Frensense Engine Expansion — Implementation Plan (Part 1 of 3)
# Categories: Authorization & IDOR | Race Conditions & Atomicity

This plan covers every vulnerability category found across all audit files in
`Audits_files/` and `codebase_audit.md.resolved`. Work is sequential: each
phase produces corpus files + optional engine changes, then we run the
benchmark to measure the gain before moving to the next.

The engine's corpus system requires per-pattern:
- `corpus/targets/{lang}_{pattern_name}_positive.ts` — the vulnerable shape
- `corpus/targets/{lang}_{pattern_name}_negative.ts` — the safe/clean shape
- `corpus/targets/{lang}_{pattern_name}.toml` — optional advisory override

---

## Phase 1 — Category 1: Authorization & IDOR

**Source audit files:** `codebase_audit.md` BUG-07/BUG-08, `backend-audit.md`
findings #1 and #5, `AGENT_AUDIT.md` AGENT-003/AGENT-009,
`backend-architecture-audit.md` seller status bypass.

**Current engine state:** `CORPUS_TS_IDOR_WORKFLOW_ACTION` and
`CORPUS_TS_FAIL_OPEN_AUTH` exist but miss: (a) DB query before ownership
check, (b) catch-block fail-open, (c) middleware that checks role but not
status, (d) child resource not validated against parent.

---

### 1.1 — ts_idor_pre_query_check

**Pattern:** A route handler resolves a user-supplied ID, executes a DB query
with that ID, receives a result, and ONLY THEN checks ownership. The
authorization comparison happens on the returned row rather than as a guard
before the query.

**Audit bugs covered:** BUG-07 (`handleGetUser`), `backend-audit.md` #1
(every MCP tool handler trusting `project_id` verbatim).

**Corpus files to create:**

`ts_idor_pre_query_check_positive.ts`:
```ts
// [frensense]
// observation = "DB query executes before ownership is verified, enabling user enumeration via 404 vs 403."
// impact = "Attacker probes valid IDs by observing whether response is 404 (not found) or 403 (found, not yours)."
// improvement = "Move the ownership guard before the DB query: if (userId !== session.customerId) return error(403)."

async function handleGetUser(request: Request, session: Session, env: Env) {
  const userId = request.url.split('/').pop() || session.customerId;

  // VULNERABLE: query runs before ownership check
  const row = await env.db.prepare('SELECT id, email FROM User WHERE id = ?')
    .bind(userId).first();

  if (!row) return Response.json({ error: 'not_found' }, { status: 404 });
  if (row.id !== session.customerId) return Response.json({ error: 'forbidden' }, { status: 403 });

  return Response.json(row);
}

async function getProjectFiles(projectId: string, session: Session, db: DB) {
  // VULNERABLE: no ownership check before fetching files
  const files = await db.prepare('SELECT * FROM project_files WHERE project_id = ?')
    .bind(projectId).all();
  return files;
}

async function readWorkspaceFile(projectId: string, path: string, session: Session, env: Env) {
  // VULNERABLE: tool handler trusts client-supplied projectId verbatim
  const file = await env.db.prepare('SELECT content FROM project_files WHERE project_id = ? AND path = ?')
    .bind(projectId, path).first();
  if (!file) return { error: 'not_found' };
  return { content: file.content };
}
```

`ts_idor_pre_query_check_negative.ts`:
```ts
// SAFE: ownership guard runs BEFORE the DB query
async function handleGetUser(request: Request, session: Session, env: Env) {
  const userId = request.url.split('/').pop() || session.customerId;

  // Guard first — no query if not the owner
  if (userId !== session.customerId) {
    return Response.json({ error: 'forbidden' }, { status: 403 });
  }

  const row = await env.db.prepare('SELECT id, email FROM User WHERE id = ?')
    .bind(userId).first();

  if (!row) return Response.json({ error: 'not_found' }, { status: 404 });
  return Response.json(row);
}

async function getProjectFiles(projectId: string, session: Session, db: DB) {
  // SAFE: ownership verified before fetch
  const project = await db.prepare('SELECT owner_id FROM projects WHERE id = ?')
    .bind(projectId).first();
  if (!project || project.owner_id !== session.customerId) {
    throw new Error('FORBIDDEN');
  }
  const files = await db.prepare('SELECT * FROM project_files WHERE project_id = ?')
    .bind(projectId).all();
  return files;
}

async function readWorkspaceFile(projectId: string, path: string, session: Session, env: Env) {
  // SAFE: ownership asserted before any data access
  await assertProjectOwnership(env, projectId, session.customerId);
  const file = await env.db.prepare('SELECT content FROM project_files WHERE project_id = ? AND path = ?')
    .bind(projectId, path).first();
  if (!file) return { error: 'not_found' };
  return { content: file.content };
}

async function assertProjectOwnership(env: Env, projectId: string, customerId: string) {
  const proj = await env.db.prepare('SELECT owner_id FROM projects WHERE id = ?')
    .bind(projectId).first();
  if (!proj || proj.owner_id !== customerId) throw new Error('FORBIDDEN');
}
```

`ts_idor_pre_query_check.toml`:
```toml
observation = "Authorization check occurs after DB query, enabling user/resource enumeration."
impact = "Attacker distinguishes valid IDs from invalid ones via differential error responses, enabling enumeration attacks before ownership is ever checked."
improvement = "Always verify ownership before executing the DB query. Return a uniform 403 for both not-found and forbidden cases."
```

---

### 1.2 — ts_child_resource_idor

**Pattern:** A route validates ownership of a parent resource but then uses a
child resource ID (supplied by the client) without verifying the child belongs
to that validated parent. This allows cross-account targeting via the child.

**Audit bugs covered:** `backend-architecture-audit.md` dispute cross-talk
(CRITICAL), `AGENT_AUDIT.md` AGENT-009 (workflowAction any user can
pause any run).

**Corpus files to create:**

`ts_child_resource_idor_positive.ts`:
```ts
// [frensense]
// observation = "Child resource ID is accepted from the client without verifying it belongs to the validated parent resource."
// impact = "Attacker supplies a child ID belonging to a different user's parent, enabling cross-account data access or escrow lockout."
// improvement = "Verify the child resource's parent matches the validated parent in the same query: WHERE id = ? AND parent_id = ?."

async function openDispute(userId: string, orderId: string, orderLineId: string, db: DB) {
  // Parent is validated correctly
  const order = await db.prepare('SELECT id FROM orders WHERE id = ? AND user_id = ?')
    .bind(orderId, userId).first();
  if (!order) throw new Error('ORDER_NOT_FOUND');

  // VULNERABLE: child orderLineId is accepted without verifying it belongs to orderId
  const line = await db.prepare('SELECT * FROM order_lines WHERE id = ?')
    .bind(orderLineId).first();
  if (line) {
    await createDispute(orderId, orderLineId, line.seller_id, db);
  }
}

async function stopAgentRun(runId: string, session: Session, env: Env) {
  // VULNERABLE: run_id is accepted; no check that it belongs to session.customerId
  const inst = await env.AGENT_RUN_WORKFLOW.get(runId);
  await inst.pause();
}

async function deleteProjectFile(projectId: string, fileId: string, session: Session, db: DB) {
  // Parent validated
  const proj = await db.prepare('SELECT owner_id FROM projects WHERE id = ?')
    .bind(projectId).first();
  if (!proj || proj.owner_id !== session.customerId) throw new Error('FORBIDDEN');

  // VULNERABLE: fileId not checked against projectId — attacker can delete any file
  await db.prepare('DELETE FROM project_files WHERE id = ?').bind(fileId).run();
}
```

`ts_child_resource_idor_negative.ts`:
```ts
// SAFE: child resource verified against the validated parent in the same query
async function openDispute(userId: string, orderId: string, orderLineId: string, db: DB) {
  const order = await db.prepare('SELECT id FROM orders WHERE id = ? AND user_id = ?')
    .bind(orderId, userId).first();
  if (!order) throw new Error('ORDER_NOT_FOUND');

  // SAFE: child must belong to the validated parent
  const line = await db.prepare(
    'SELECT ol.*, op.seller_id FROM order_lines ol JOIN order_packages op ON ol.package_id = op.id WHERE ol.id = ? AND op.order_id = ?'
  ).bind(orderLineId, orderId).first();
  if (!line) throw new Error('INVALID_ORDER_LINE');

  await createDispute(orderId, orderLineId, line.seller_id, db);
}

async function stopAgentRun(runId: string, session: Session, env: Env) {
  const inst = await env.AGENT_RUN_WORKFLOW.get(runId);
  const status = await inst.status();
  // SAFE: ownership verified before action
  if (status.output?._customerId !== session.customerId) {
    throw new Error('FORBIDDEN');
  }
  await inst.terminate();
}

async function deleteProjectFile(projectId: string, fileId: string, session: Session, db: DB) {
  const proj = await db.prepare('SELECT owner_id FROM projects WHERE id = ?')
    .bind(projectId).first();
  if (!proj || proj.owner_id !== session.customerId) throw new Error('FORBIDDEN');

  // SAFE: fileId scoped to projectId in the DELETE
  await db.prepare('DELETE FROM project_files WHERE id = ? AND project_id = ?')
    .bind(fileId, projectId).run();
}
```

`ts_child_resource_idor.toml`:
```toml
observation = "Child resource ID is client-supplied and used without binding it to the validated parent resource."
impact = "Enables cross-account resource manipulation by substituting a child ID belonging to a different parent."
improvement = "Include the validated parent ID in every child resource query: WHERE child_id = ? AND parent_id = ?."
```

---

### 1.3 — ts_catch_fail_open (EXTEND existing)

**Status:** Positive and negative files already exist (`ts_catch_fail_open_*`).
We need to add a second negative variant covering the `quota` / `middleware`
pattern from AGENT-011, and ensure the `.toml` is strong.

**Audit bugs covered:** `AGENT_AUDIT.md` AGENT-011 (quota fails open with
`{ allowed: true, remaining: 999 }` in production), `codebase_audit.md`
SEC-02 (empty allowed origins returns raw origin).

**Action:** Add `ts_catch_fail_open_negative2.ts`:
```ts
// SAFE variant 2: fail closed in production when auth service is unreachable
async function checkAndConsumeQuota(userId: string, env: Env) {
  try {
    const raw = await env.QUOTA_KV.get(`quota:${userId}`);
    const quota = raw ? JSON.parse(raw) : null;
    if (!quota || quota.remaining <= 0) return { allowed: false, reason: 'quota_exceeded' };
    await env.QUOTA_KV.put(`quota:${userId}`, JSON.stringify({ ...quota, remaining: quota.remaining - 1 }));
    return { allowed: true, remaining: quota.remaining - 1 };
  } catch (e) {
    console.error('checkAndConsumeQuota failed:', e);
    // SAFE: fail closed in production; only allow in dev
    if (env.ENVIRONMENT === 'development') return { allowed: true, remaining: 999 };
    return { allowed: false, reason: 'quota_service_unavailable' };
  }
}
```

**Add `ts_catch_fail_open.toml`:**
```toml
observation = "A catch block returns a permissive authorization result, failing open when the auth/quota service is unreachable."
impact = "Any transient error in auth/quota infrastructure grants all users unlimited access to gated resources."
improvement = "Fail closed by default. Only fail open explicitly in development environments gated by an ENVIRONMENT check."
```

---

### 1.4 — ts_role_without_status_check

**Pattern:** Middleware or a guard checks that a user has a certain role (e.g.,
`SELLER`, `ADMIN`) but never verifies the associated profile's lifecycle status
(`PENDING_VERIFICATION`, `SUSPENDED`, `REJECTED`, `ACTIVE`). The role alone
is granted immediately on onboarding before verification completes.

**Audit bugs covered:** `backend-architecture-audit.md` [HIGH] Seller Profile
Status Bypass.

**Corpus files to create:**

`ts_role_without_status_check_positive.ts`:
```ts
// [frensense]
// observation = "Middleware checks role membership but not the associated profile's lifecycle status."
// impact = "Unverified, suspended, or rejected users can perform all role-gated operations."
// improvement = "Query the profile record and assert status === 'ACTIVE' before calling next()."

// VULNERABLE: role checked, status never verified
const sellerProcedure = protectedProcedure.use(({ ctx, next }) => {
  if (ctx.session.user.role !== 'SELLER' && ctx.session.user.role !== 'ADMIN') {
    throw new TRPCError({ code: 'FORBIDDEN' });
  }
  // MISSING: no check that ctx.session.user's Seller profile.status === 'ACTIVE'
  return next();
});

async function handleSellerDashboard(req: Request, session: Session, db: DB) {
  if (session.role !== 'seller') {
    return Response.json({ error: 'forbidden' }, { status: 403 });
  }
  // MISSING: session.status is never checked; suspended sellers reach here
  const metrics = await db.prepare('SELECT * FROM seller_metrics WHERE seller_id = ?')
    .bind(session.sellerId).all();
  return Response.json(metrics);
}
```

`ts_role_without_status_check_negative.ts`:
```ts
// SAFE: both role and profile status verified before granting access
const sellerProcedure = protectedProcedure.use(async ({ ctx, next }) => {
  const seller = await prisma.seller.findUnique({
    where: { userId: ctx.session.user.id },
    select: { status: true }
  });
  if (!seller || (seller.status !== 'ACTIVE' && ctx.session.user.role !== 'ADMIN')) {
    throw new TRPCError({ code: 'FORBIDDEN', message: 'Seller profile is not active' });
  }
  return next();
});

async function handleSellerDashboard(req: Request, session: Session, db: DB) {
  const sellerProfile = await db.prepare('SELECT status FROM sellers WHERE user_id = ?')
    .bind(session.userId).first();
  if (!sellerProfile || sellerProfile.status !== 'ACTIVE') {
    return Response.json({ error: 'forbidden' }, { status: 403 });
  }
  const metrics = await db.prepare('SELECT * FROM seller_metrics WHERE seller_id = ?')
    .bind(session.sellerId).all();
  return Response.json(metrics);
}
```

`ts_role_without_status_check.toml`:
```toml
observation = "Role check passes but profile lifecycle status (PENDING/SUSPENDED/REJECTED) is never verified."
impact = "Unverified or suspended accounts perform all role-gated operations including financial reporting and product management."
improvement = "After role check, query the profile table and assert status === 'ACTIVE'. Wrap both in a single reusable middleware."
```

---

## Phase 2 — Category 2: Race Conditions & Atomicity

**Source audit files:** `backend-audit.md` findings #3 (double-spend credits),
`backend-architecture-audit.md` CRITICAL webhook double-fund, `codebase_audit.md`
BUG-03 (Date.now() tenant ID), `AGENT_AUDIT.md` AGENT-011.

**Current engine state:** `CORPUS_TS_RACE_CONDITION_READ_CHECK_WRITE` exists
but only detects KV-level get-check-put sequences. Misses: (a) status check
outside a DB transaction, (b) weak entropy ID generation.

---

### 2.1 — ts_race_condition_read_check_write (EXTEND existing)

The existing rule fires on KV get→check→put. We need to extend it with two
new negative variants that cover the DB transaction pattern, so the engine
learns that checking status INSIDE a `$transaction` / `WHERE status = 'PENDING'`
filter is the correct pattern.

**Add `ts_race_condition_read_check_write_negative6.ts`:**
```ts
// SAFE variant 6: status check is atomic inside the transaction filter
async function handlePaymentWebhook(paymentId: string, prisma: PrismaClient) {
  // SAFE: the WHERE clause makes the update conditional — if another request
  // already set status to SUCCESS, this update finds no matching row and throws,
  // preventing the duplicate fundWallet call.
  await prisma.$transaction(async (tx) => {
    const updated = await tx.payment.update({
      where: { id: paymentId, status: 'PENDING' },  // atomic check-and-update
      data: { status: 'SUCCESS' }
    });
    await fundWallet(updated.userId, updated.amount, tx);
  });
}

async function deductCreditsAtomic(userId: string, amount: number, db: D1Database) {
  // SAFE: single atomic SQL statement — check and write in one operation
  const result = await db.prepare(
    'UPDATE credits SET balance = balance - ? WHERE user_id = ? AND balance >= ?'
  ).bind(amount, userId, amount).run();
  return result.meta.changes > 0;
}
```

**Add `ts_race_condition_read_check_write_negative7.ts`:**
```ts
// SAFE variant 7: distributed lock pattern with expiry
async function deductCreditsWithLock(userId: string, amount: number, kv: KVNamespace) {
  const lockKey = `lock:credits:${userId}`;
  const acquired = await kv.put(lockKey, '1', { expirationTtl: 10, condition: 'not-exist' });
  if (!acquired) throw new Error('CONCURRENT_MODIFICATION');
  try {
    const raw = await kv.get(`credits:${userId}`);
    const balance = raw ? parseInt(raw, 10) : 0;
    if (balance < amount) return false;
    await kv.put(`credits:${userId}`, String(balance - amount));
    return true;
  } finally {
    await kv.delete(lockKey);
  }
}
```

**Add extended positive variant `ts_race_condition_read_check_write_positive2.ts`:**
```ts
// [frensense]
// observation = "Status check-then-act is performed outside a database transaction, enabling concurrent double execution."
// impact = "Two concurrent webhook deliveries both read PENDING status, both enter the transaction, both execute fundWallet — wallet credited twice."
// improvement = "Move the status guard inside the transaction using a conditional WHERE clause: WHERE id = ? AND status = 'PENDING'."

async function handlePaymentWebhook(paymentId: string, prisma: PrismaClient) {
  const payment = await prisma.payment.findUnique({ where: { id: paymentId } });

  // VULNERABLE: status check outside transaction — concurrent requests both pass
  if (payment?.status === 'SUCCESS') return;

  await prisma.$transaction(async (tx) => {
    await tx.payment.update({
      where: { id: payment!.id },  // no status guard here
      data: { status: 'SUCCESS' }
    });
    if (payment!.orderId === 'WALLET_FUND') {
      await fundWallet(payment!.userId, payment!.amount, tx);
    }
  });
}
```

---

### 2.2 — ts_weak_id_entropy

**Pattern:** An identifier, token, or key is generated using `Date.now()`,
`Math.random()`, or `new Date().getTime()`. These lack cryptographic strength
and have millisecond (or sub-second) resolution, causing collisions under
concurrent load.

**Audit bugs covered:** `codebase_audit.md` BUG-03 (tenantId = `tnt_${Date.now()}`).

**Corpus files to create:**

`ts_weak_id_entropy_positive.ts`:
```ts
// [frensense]
// observation = "Identifier generated with Date.now() or Math.random() — low entropy, collision-prone under concurrent load."
// impact = "Two concurrent registrations within the same millisecond produce identical IDs. Causes silent conflicts, data corruption, or security token guessing."
// improvement = "Use crypto.randomUUID() or a cryptographically secure random bytes source for all IDs and tokens."

function createTenant(name: string, ownerId: string) {
  // VULNERABLE: millisecond resolution — collides under concurrent load
  const tenantId = `tnt_${Date.now()}`;
  return { id: tenantId, name, ownerId };
}

function generateSessionToken(): string {
  // VULNERABLE: Math.random is not cryptographically secure
  return Math.random().toString(36).slice(2);
}

function createInviteCode(): string {
  // VULNERABLE: timestamp-based codes are guessable
  return `inv_${new Date().getTime()}`;
}

function generateApiKey(userId: string): string {
  // VULNERABLE: predictable key
  return `key_${userId}_${Date.now()}`;
}
```

`ts_weak_id_entropy_negative.ts`:
```ts
// SAFE: cryptographically secure ID generation
function createTenant(name: string, ownerId: string) {
  const tenantId = `tnt_${crypto.randomUUID()}`;
  return { id: tenantId, name, ownerId };
}

function generateSessionToken(): string {
  const bytes = new Uint8Array(32);
  crypto.getRandomValues(bytes);
  return Array.from(bytes).map(b => b.toString(16).padStart(2, '0')).join('');
}

function createInviteCode(): string {
  return `inv_${crypto.randomUUID()}`;
}

function generateApiKey(userId: string): string {
  const secret = crypto.randomUUID().replace(/-/g, '');
  return `fhp_${userId.slice(0, 8)}_${secret}`;
}
```

`ts_weak_id_entropy.toml`:
```toml
observation = "Identifier or token generated using Date.now() or Math.random() — insufficient entropy for unique or secure IDs."
impact = "Collision under concurrent load causes duplicate IDs or silent data corruption. Guessable tokens enable account takeover or invite abuse."
improvement = "Use crypto.randomUUID() for all IDs. Use crypto.getRandomValues() for security tokens."
```

---

### 2.3 — ts_missing_idempotency_guard

**Pattern:** A confirm/webhook/callback handler performs an `INSERT` (or ORM
`.create()`) without first checking if the operation has already been
completed. Replay attacks (email confirm clicked twice, webhook retried by
provider) create duplicate records.

**Audit bugs covered:** `codebase_audit.md` BUG-04 / ARCH-02
(`createDefaultTenant` with no guard), `backend-architecture-audit.md`
webhook double-fund.

**Corpus files to create:**

`ts_missing_idempotency_guard_positive.ts`:
```ts
// [frensense]
// observation = "Confirm/webhook handler performs INSERT without an idempotency pre-check or conditional INSERT."
// impact = "Replaying a confirm link or webhook creates duplicate records (double tenant, double wallet credit, double subscription)."
// improvement = "Pre-check for existing record before INSERT, or use INSERT OR IGNORE / WHERE NOT EXISTS / conditional update."

async function handleConfirm(token: string, db: D1Database) {
  const user = await db.prepare('SELECT id, default_tenant_id FROM User WHERE confirm_token = ?')
    .bind(token).first();
  if (!user) return Response.json({ error: 'invalid_token' }, { status: 400 });

  // VULNERABLE: no check if tenant already exists — clicking confirm twice creates two tenants
  const tenantId = `tnt_${crypto.randomUUID()}`;
  await db.prepare('INSERT INTO tenants (id, name, owner_id) VALUES (?, ?, ?)')
    .bind(tenantId, `${user.id}'s workspace`, user.id).run();

  await db.prepare('UPDATE User SET default_tenant_id = ?, confirmed = 1 WHERE id = ?')
    .bind(tenantId, user.id).run();
}

async function handleSubscriptionWebhook(customerId: string, plan: string, db: D1Database) {
  // VULNERABLE: no idempotency check — provider retries create duplicate subscriptions
  await db.prepare('INSERT INTO subscriptions (customer_id, plan, created_at) VALUES (?, ?, ?)')
    .bind(customerId, plan, new Date().toISOString()).run();
}
```

`ts_missing_idempotency_guard_negative.ts`:
```ts
// SAFE: idempotency pre-check before INSERT
async function handleConfirm(token: string, db: D1Database) {
  const user = await db.prepare('SELECT id, default_tenant_id, confirmed FROM User WHERE confirm_token = ?')
    .bind(token).first();
  if (!user) return Response.json({ error: 'invalid_token' }, { status: 400 });

  // SAFE: early return if already confirmed — idempotent
  if (user.default_tenant_id || user.confirmed) {
    return Response.json({ status: 'already_confirmed' });
  }

  const tenantId = `tnt_${crypto.randomUUID()}`;
  await db.prepare('INSERT OR IGNORE INTO tenants (id, name, owner_id) VALUES (?, ?, ?)')
    .bind(tenantId, `${user.id}'s workspace`, user.id).run();

  await db.prepare('UPDATE User SET default_tenant_id = ?, confirmed = 1 WHERE id = ? AND confirmed = 0')
    .bind(tenantId, user.id).run();
}

async function handleSubscriptionWebhook(customerId: string, plan: string, db: D1Database) {
  // SAFE: INSERT OR IGNORE with unique constraint prevents duplicates
  await db.prepare('INSERT OR IGNORE INTO subscriptions (customer_id, plan, created_at) VALUES (?, ?, ?)')
    .bind(customerId, plan, new Date().toISOString()).run();
}
```

`ts_missing_idempotency_guard.toml`:
```toml
observation = "Confirm/webhook handler inserts a record without verifying it does not already exist."
impact = "Replaying the operation (link clicked twice, webhook retried) creates duplicate records — orphaned tenants, double wallet credits, duplicate subscriptions."
improvement = "Add a pre-flight existence check or use INSERT OR IGNORE / conditional update. Mark confirm tokens as consumed atomically."
```

---

## Phase 1 & 2 — Engine Changes Required

### runner.rs — Extended Taint Seeder

In addition to corpus files, two small engine changes enable the new rules to
reach their full recall:

**Change 1:** Treat `$transaction(` as a scope boundary in the
`RACE_CONDITION_READ_CHECK_WRITE` temporal check. When a `db.prepare.bind`
call and its preceding `if (row.status === ...)` check are separated by a
`$transaction(` boundary, the check is classified as an unguarded out-of-transaction
check and flagged.

**Change 2 (existing gap):** Extend the intra-procedural DB sink scanner
(currently flags `.insert`, `.update`, `.remove`) to also flag
`.create(` and `.upsert(` as write sinks. This ensures ORM-based code
(Prisma, TypeORM) is covered alongside raw SQL patterns.

---

## Phase 1 & 2 — Benchmark Expectations

Based on the current distribution (81.08% recall, 79.43% precision):

| New Pattern | Estimated TP gain | Expected FP risk |
|---|---|---|
| `ts_idor_pre_query_check` | +2 to +3 TP | Low — guarded by DB query + ownership check co-occurrence |
| `ts_child_resource_idor` | +1 to +2 TP | Low — specific parent+child query pattern |
| `ts_catch_fail_open` extension | +1 TP | Low — already tuned |
| `ts_role_without_status_check` | +1 TP | Medium — role check without status is common in non-vulnerable code |
| `ts_weak_id_entropy` | +1 TP | Low — `Date.now()` in ID context is specific |
| `ts_missing_idempotency_guard` | +1 to +2 TP | Medium — any confirm handler without a pre-check |
| Race condition TX extension | +1 TP | Low — requires `$transaction` + out-of-block check co-occurrence |

**Projected post-Phase-2 recall: 83–85% (31–32/37 files)**

---

*Continue in implementation_plan_p2.md — Categories 3, 4, 5 (Sensitive Data
Exposure, Header Trust, Missing Guards & Null Safety)*
