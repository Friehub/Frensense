# Frensense — Extended Taxonomy: Targeting 10,000 Unique Bugs
## New Categories 176–300: What Semgrep and CodeQL Cannot Catch

**Purpose:** This document is a spec for an LLM to write corpus pairs.  
Each entry gives the bug name, a one-line description of what it looks like, and how many mutations to write.  
Do not write full code here — use these entries as prompts.

**Why these categories beat Semgrep/CodeQL:**  
Semgrep matches AST patterns you hand-write. CodeQL follows taint through types you model.  
Neither can learn. Neither understands "this function should have called X but didn't."  
Neither knows your codebase's conventions. Neither can reason about business semantics.  
Frensense can do all of these through corpus learning + temporal rules + CSA.

**Cumulative target:**  
Existing taxonomy (doc 1): ~4,050 bugs  
This document: ~6,000 bugs  
**Combined total: ~10,050 unique corpus pairs**

---

## How to Use This Document

Each category has:
- **Why missed:** one sentence on why rule-based tools fail here
- **Detection method:** which Frensense primitive catches it (taint / temporal / CSA / fingerprint)
- A table of bug names, descriptions, mutation count, and target language

**Mutation shorthand:**
- `[3]` = write 3 variants (positive + negative each)
- `[5]` = write 5 variants
- Variants mean: rename variables, change framework, add a try/catch wrapper, move through intermediate variable, async vs sync version

---

## Group A — Semantic Drift & Duplicate Implementation
*~400 bugs | Detection: fingerprint similarity + CSA*

**Why missed:** Semgrep and CodeQL analyze each function in isolation. They cannot detect when two functions that should be identical have silently diverged. Frensense's near-duplicate fingerprinting and CSA can surface this.

---

### Category 176 — Auth Resolver Divergence
Two functions that both extract a session/user from a request but check different headers, accept different token formats, or apply different validation rules.

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_auth_diverge_header_set` | `resolveAuth` checks `Authorization`, `resolveUser` only checks `X-Session-Token` — paid users on Bearer get rejected by one path | [4] | TS |
| `ts_auth_diverge_expiry_check` | One auth helper validates token expiry, a copy of it does not | [3] | TS |
| `ts_auth_diverge_scope_validation` | Two permission checkers — one validates scope claim, one skips it | [3] | TS |
| `ts_auth_diverge_admin_check` | Admin guard checks `role === "admin"`, another checks `products.includes("admin")` — different sources of truth | [4] | TS |
| `ts_auth_diverge_null_session` | One path returns `null` for missing session, another returns `{}` — callers treat `{}` as authenticated | [3] | TS |
| `ts_auth_diverge_case_sensitivity` | Email lookup is case-sensitive in one resolver, case-insensitive in another — same user can't log in from both paths | [2] | TS |
| `ts_auth_diverge_tenant_scope` | One auth path scopes session to tenant, another does not — cross-tenant access possible | [3] | TS |
| `rust_auth_diverge_middleware` | Two Axum extractors implement auth differently — one accepts expired tokens | [3] | Rust |

---

### Category 177 — Validation Logic Divergence
The same field is validated in multiple places but with different rules. One path enforces the constraint, another doesn't.

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_valid_diverge_email_regex` | Email validated with strict regex on signup, loose regex on profile update | [4] | TS |
| `ts_valid_diverge_price_bounds` | Price validated 0–999999 on product create, no bounds on price update | [3] | TS |
| `ts_valid_diverge_string_length` | Bio limited to 500 chars on create, no limit on edit | [3] | TS |
| `ts_valid_diverge_phone_format` | Phone validated on registration, accepted raw on checkout | [3] | TS |
| `ts_valid_diverge_url_scheme` | URL validated as https-only on webhook registration, any scheme on update | [4] | TS |
| `ts_valid_diverge_quantity` | Quantity checked > 0 on cart add, unchecked on cart update | [3] | TS |
| `ts_valid_diverge_username` | Username checked for reserved words on create, not on rename | [3] | TS |
| `ts_valid_diverge_admin_bypass` | Validation skipped when `isAdmin` is true but admin status comes from user-controlled field | [4] | TS |

---

### Category 178 — Permission Check Divergence
The same resource has multiple access paths. One path checks permissions, another skips it.

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_perm_diverge_read_write` | GET /resource/:id checks ownership, PUT /resource/:id does not | [4] | TS |
| `ts_perm_diverge_api_vs_internal` | Public API checks role, internal helper used by another route does not | [3] | TS |
| `ts_perm_diverge_batch_vs_single` | Single-item endpoint checks permission per item, batch endpoint does not | [4] | TS |
| `ts_perm_diverge_soft_delete` | Active records require permission check, soft-deleted records do not but are still accessible | [3] | TS |
| `ts_perm_diverge_versioned_api` | `/v2/resource` has auth, `/v1/resource` (still active) does not | [3] | TS |
| `ts_perm_diverge_export_endpoint` | View endpoint has row-level permission, export-all endpoint does not | [4] | TS |
| `ts_perm_diverge_webhook_vs_ui` | UI action requires manager role, webhook handler for same action does not | [3] | TS |

---

### Category 179 — Sanitizer Divergence
Same input processed in two places. One sanitizes, the other doesn't.

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_sanitize_diverge_html_field` | Post body sanitized on create with DOMPurify, not sanitized on edit | [3] | TS |
| `ts_sanitize_diverge_filename` | Filename sanitized on upload, raw filename stored and used in download path | [3] | TS |
| `ts_sanitize_diverge_search_term` | Search input escaped in UI template, raw value passed to full-text search index | [3] | TS |
| `ts_sanitize_diverge_admin_path` | Regular user data sanitized, admin-provided data treated as trusted without sanitization | [4] | TS |
| `ts_sanitize_diverge_bulk_import` | Single record create sanitizes, bulk CSV import does not | [3] | TS |

---

### Category 180 — Financial Keyword Dictionary Divergence
The same list of financial categories, tax codes, or keywords is hardcoded in multiple files and drifts over time.

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_keyword_dict_tax_category` | Tax rate lookup in checkout uses list A, tax reporting uses list B — refunds calculated on wrong rate | [3] | TS |
| `ts_keyword_dict_blocked_terms` | Content moderation blocked-word list differs between post creation and edit endpoints | [3] | TS |
| `ts_keyword_dict_currency_codes` | Accepted currency list in payment handler differs from list in reporting — transactions silently rejected | [3] | TS |
| `ts_keyword_dict_country_codes` | Shipping country allowlist differs between checkout and address book — orders fail for valid countries | [3] | TS |
| `ts_keyword_dict_status_values` | Order status enum differs between service layer and DB migration — invalid transitions possible | [4] | TS |
| `ts_keyword_dict_permission_names` | Permission string names differ between role assignment and permission check — grants never match | [3] | TS |

---

## Group B — Missing Behavior / Broken Contracts
*~500 bugs | Detection: temporal rules + CSA*

**Why missed:** Semgrep can only detect what IS in the code. It cannot detect what is MISSING. CodeQL follows data — it cannot enforce that a function must call another function. Frensense's temporal engine can express "A must be followed by B."

---

### Category 181 — Missing Rollback on Partial Failure
An operation modifies multiple resources. If the second modification fails, the first is not rolled back.

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_rollback_user_profile_update` | User email updated in auth table, then profile table — auth update not rolled back if profile fails | [4] | TS |
| `ts_rollback_payment_and_order` | Payment charged, order creation fails — payment not refunded | [4] | TS |
| `ts_rollback_file_and_db` | File uploaded to storage, then DB record created — file not deleted if DB fails | [3] | TS |
| `ts_rollback_notification_and_state` | Notification sent, then state transition — notification not recalled if transition fails | [3] | TS |
| `ts_rollback_subscription_grant` | Subscription created, entitlements granted — subscription not cancelled if entitlement fails | [3] | TS |
| `ts_rollback_webhook_register` | Webhook registered externally, then stored in DB — external webhook not deregistered if DB fails | [3] | TS |
| `ts_rollback_index_and_db` | Record added to search index, then stored in DB — index not cleaned if DB fails | [3] | TS |
| `ts_rollback_cache_and_db` | Cache updated before DB write — cache contains stale/incorrect value if DB write fails | [4] | TS |
| `rust_rollback_two_db_ops` | Two sqlx queries without transaction — first not rolled back if second fails | [3] | Rust |

---

### Category 182 — Missing Compensation Transaction
In distributed/saga patterns: a step succeeds, a later step fails, but the compensation for the first step is never triggered.

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_saga_reserve_no_release` | Inventory reserved at step 1, payment fails at step 2, reservation never released | [4] | TS |
| `ts_saga_charge_no_refund` | Customer charged at step 1, fulfillment fails at step 2, charge never refunded | [4] | TS |
| `ts_saga_create_no_delete` | External resource created (cloud sandbox, email account), user creation fails, external resource not deleted | [3] | TS |
| `ts_saga_lock_no_unlock` | Record locked for editing, subsequent operation fails, lock never released | [3] | TS |
| `ts_saga_debit_no_credit` | Ledger debited, downstream credit to recipient fails, debit not reversed | [4] | TS |
| `ts_saga_publish_no_retract` | Event published to queue, downstream consumer fails, no compensating retraction event | [3] | TS |

---

### Category 183 — Missing Pre-Condition Check
A function documents or implies it requires a certain state before executing, but that state is never verified by callers.

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_precond_email_verified_before_purchase` | Purchase endpoint called without verifying email is confirmed | [3] | TS |
| `ts_precond_kyc_before_withdrawal` | Withdrawal processed without checking KYC is approved | [3] | TS |
| `ts_precond_account_active_before_action` | Action allowed on suspended account — account status never checked | [4] | TS |
| `ts_precond_subscription_before_feature` | Feature used without checking subscription is in ACTIVE state (not TRIAL_EXPIRED) | [4] | TS |
| `ts_precond_device_trusted_before_sensitive` | Sensitive operation allowed without checking device is in trusted list | [3] | TS |
| `ts_precond_rate_limit_before_expensive` | Expensive API call made without first checking rate limit headroom | [3] | TS |
| `ts_precond_quota_before_llm_call` | LLM call made without checking remaining token quota | [3] | TS |
| `ts_precond_2fa_before_withdrawal` | Financial withdrawal allowed without checking 2FA was completed this session | [4] | TS |

---

### Category 184 — Missing Post-Condition Enforcement
After an operation, required follow-up state is not verified or established.

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_postcond_session_created_after_login` | Login succeeds but session token not verified as stored before returning to client | [3] | TS |
| `ts_postcond_index_updated_after_write` | DB record written but search index update not confirmed | [3] | TS |
| `ts_postcond_email_queued_after_action` | Action completes but email confirmation not confirmed as queued | [3] | TS |
| `ts_postcond_cache_invalidated_after_update` | Record updated but cache invalidation not verified — stale reads guaranteed | [3] | TS |
| `ts_postcond_balance_positive_after_debit` | Wallet debited but resulting balance not checked — negative balance possible | [4] | TS |
| `ts_postcond_lock_released_after_txn` | Transaction commits but advisory lock not released — next operation hangs | [3] | TS |

---

### Category 185 — Missing Idempotency Key Check
Operations that have side effects (charge, send, create) are not protected against duplicate execution.

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_idempotency_payment_missing` | Payment endpoint has no idempotency key — double-click charges twice | [4] | TS |
| `ts_idempotency_email_send_missing` | Email send endpoint not idempotent — retry sends duplicate emails | [3] | TS |
| `ts_idempotency_subscription_create` | Subscription creation not idempotent — two concurrent requests create two subscriptions | [3] | TS |
| `ts_idempotency_webhook_processing` | Webhook processed without checking event ID — duplicate webhook fires duplicate action | [4] | TS |
| `ts_idempotency_job_dispatch` | Background job dispatched without dedup key — same job runs N times | [3] | TS |
| `ts_idempotency_refund_issue` | Refund issuance not idempotent — retry refunds twice | [4] | TS |
| `ts_idempotency_sms_send` | SMS OTP resend not idempotent — user receives duplicate OTPs | [3] | TS |
| **`ts_missing_idempotency_guard`** | General: state-changing handler with no idempotency protection | [5] | TS |

---

### Category 186 — Missing Cleanup on Error Path
Happy path releases resources. Error path does not.

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_cleanup_temp_file_on_error` | Temp file created, error in processing, temp file not deleted | [3] | TS |
| `ts_cleanup_db_connection_on_error` | DB connection checked out, error thrown, connection not returned to pool | [4] | Rust/TS |
| `ts_cleanup_external_resource_on_error` | Cloud sandbox spun up, error in agent run, sandbox not torn down | [3] | TS |
| `ts_cleanup_lock_on_error` | Advisory lock acquired, exception thrown, lock not released | [4] | TS |
| `ts_cleanup_transaction_on_error` | DB transaction started, uncaught error, transaction not rolled back | [3] | TS |
| `ts_cleanup_stream_on_error` | Read stream opened, error in processing, stream not destroyed | [3] | TS |
| `ts_cleanup_child_process_on_error` | Child process spawned, parent errors, child process becomes zombie | [3] | TS |
| `rust_cleanup_file_handle_on_error` | File opened, error in write, file handle not explicitly dropped | [3] | Rust |

---

## Group C — Defensive Code That Doesn't Actually Defend
*~300 bugs | Detection: CSA (Contract Surface Analysis)*

**Why missed:** The code LOOKS defensive. It has `if` checks, try-catch, validation calls, sanitize calls. Semgrep and CodeQL see the guard and consider the code safe. Frensense's CSA can detect when the guard is structurally present but semantically hollow.

---

### Category 187 — Guards That Always Pass
A validation or permission check always returns the same result regardless of input.

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_guard_always_true_validator` | `isValidEmail(email)` always returns `true` because it never evaluates the argument | [4] | TS |
| `ts_guard_always_true_permission` | `canEdit(user, resource)` always returns `true` — missing the actual comparison | [3] | TS |
| `ts_guard_always_true_rate_limit` | `checkRateLimit()` always returns `{ allowed: true }` — counter never read | [3] | TS |
| `ts_guard_always_true_schema` | `validateSchema(data, schema)` returns `true` without checking schema | [3] | TS |
| `ts_guard_always_true_ownership` | `isOwner(userId, resource)` returns `true` because `userId` is checked against itself | [4] | TS |
| `ts_guard_always_true_quota` | `hasQuota(user)` always returns `true` — quota DB query result ignored | [3] | TS |
| `ts_guard_short_circuit_or` | `if (isAdmin(user) || isOwner(user, resource))` — `isAdmin` always true, second condition never matters | [4] | TS |
| `ts_guard_short_circuit_null` | `if (!user || user.role === "admin")` — falsy user grants admin | [3] | TS |

---

### Category 188 — Sanitizers That Return Input Unchanged
A sanitize or escape function is called but the implementation doesn't transform anything.

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_sanitize_passthrough_html` | `sanitizeHtml(input)` returns `input` unchanged — DOMPurify call commented out | [3] | TS |
| `ts_sanitize_passthrough_sql` | `escapeSql(value)` returns `value` — replace regex has wrong target | [3] | TS |
| `ts_sanitize_passthrough_filename` | `sanitizeFilename(name)` returns `name` — regex never matches real input | [3] | TS |
| `ts_sanitize_passthrough_url` | `sanitizeUrl(url)` just returns `url.trim()` — no protocol validation | [3] | TS |
| `ts_sanitize_partial_html` | Strips `<script>` but not `<img onerror>` or `<svg onload>` | [4] | TS |
| `ts_sanitize_wrong_encoding` | HTML-encodes for HTML context but value is used in a JS context | [3] | TS |
| `ts_sanitize_after_use` | `sanitize(input)` called after input already used in DB query | [4] | TS |

---

### Category 189 — Error Handlers That Grant Access (Fail-Open)
A try-catch exists but the catch block grants permissions, skips auth, or sets a truthy session.

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| **`ts_fail_open_auth_catch`** | `try { session = verifyToken(t) } catch { session = { userId: null } }` — null session treated as valid | [4] | TS |
| `ts_fail_open_quota_catch` | `try { quota = checkQuota() } catch { quota = { allowed: true } }` | [4] | TS |
| `ts_fail_open_permission_catch` | `try { allowed = checkPermission() } catch { allowed = true }` | [3] | TS |
| `ts_fail_open_validation_catch` | `try { valid = validate(input) } catch { valid = true }` | [3] | TS |
| `ts_fail_open_rate_limit_catch` | `try { rl = rateLimit() } catch { rl = { remaining: 999 } }` | [3] | TS |
| `ts_fail_open_signature_catch` | `try { verified = verifyHmac(body, sig) } catch { verified = true }` | [4] | TS |
| `ts_fail_open_subscription_catch` | `try { plan = getPlan(userId) } catch { plan = "premium" }` — error grants premium | [3] | TS |

---

### Category 190 — Checks That Happen Too Late
Auth, ownership, or validation is checked but only after the sensitive action is already taken.

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_late_check_auth_after_write` | DB write happens at line 5, auth check happens at line 10 | [4] | TS |
| `ts_late_check_ownership_after_fetch` | Resource data returned to caller, ownership checked as an afterthought | [3] | TS |
| `ts_late_check_quota_after_llm` | LLM called, response returned, quota checked — charge already incurred | [4] | TS |
| `ts_late_check_rate_limit_after_sms` | SMS sent first, rate limit checked after — SMS flood possible | [3] | TS |
| `ts_late_check_payment_after_provisioning` | Cloud resource provisioned, payment charged after — race where resource exists without payment | [3] | TS |
| `ts_late_check_permission_after_delete` | Record deleted, then permission check done — resource already gone | [3] | TS |
| `ts_late_check_signature_after_process` | Webhook body processed, signature verified after — TOCTOU on webhook | [4] | TS |

---

### Category 191 — Null Checks That Use the Null Value Anyway
A null/undefined check is present but the code uses the potentially-null value on the error path or in the check itself.

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_null_check_wrong_variable` | `if (!user) return` but `session.user.id` used — wrong variable guarded | [4] | TS |
| `ts_null_check_after_use` | `data.process(); if (!data) return;` — used before check | [3] | TS |
| `ts_null_check_nested_unguarded` | `if (!user) return` guards top level but `user.profile.address.city` not guarded for nested nulls | [4] | TS |
| `ts_null_check_truthy_falsy_confusion` | `if (balance)` — zero balance (falsy) treated as missing balance | [4] | TS |
| `ts_null_check_optional_chain_ignored` | `user?.role` returns `undefined` but compared with `=== "admin"` — always false, access granted | [3] | TS |
| `rust_unwrap_after_is_some_check` | `if x.is_some() { other_function(); x.unwrap() }` — `other_function` may invalidate `x` | [3] | Rust |

---

## Group D — Context-Dependent Safety
*~400 bugs | Detection: taint flow state + context features*

**Why missed:** Semgrep checks patterns in isolation. The same `db.query()` call is dangerous in one context and safe in another. CodeQL can approximate this but only for types it models. Frensense's flow state tags can carry "this path went through auth" or "this value came from an admin" as context.

---

### Category 192 — Internal Functions Exposed as External APIs
A function designed for internal/trusted use is mounted directly on a public route.

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_internal_fn_as_api_admin_helper` | Admin helper function mounted on `/api/admin-helper` without auth because "it's internal" | [4] | TS |
| `ts_internal_fn_as_api_db_query` | Raw DB query function exposed as API endpoint — SQL parameters from request | [3] | TS |
| `ts_internal_fn_as_api_system_info` | System info collector mounted on `/debug` — no auth | [3] | TS |
| `ts_internal_fn_as_api_config_reader` | Config reader mounted as API — exposes secrets | [3] | TS |
| `ts_internal_fn_as_api_service_call` | Internal service aggregator mounted as public route — no auth | [3] | TS |
| `ts_internal_fn_as_webhook_handler` | Internal event handler mounted as public webhook endpoint without signature check | [4] | TS |

---

### Category 193 — Same Data Safe in One Context, Dangerous in Another
Code that is correct for one use case is copied and used for another where it is incorrect.

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_context_admin_data_in_user_response` | Admin endpoint response shape reused for user endpoint — includes fields users shouldn't see | [4] | TS |
| `ts_context_internal_log_in_user_response` | Debug log format reused as API error response — exposes stack traces | [3] | TS |
| `ts_context_test_auth_in_production` | Test authentication bypass (`if (process.env.TEST_MODE)`) left in production code path | [4] | TS |
| `ts_context_dev_cors_in_production` | CORS config from development (allow all) carried into production build | [3] | TS |
| `ts_context_seed_data_in_production` | Seed/demo account credentials committed and reachable in production | [3] | TS |
| `ts_context_batch_fn_called_per_request` | Batch function designed for offline use called per request — no auth, no rate limit | [3] | TS |

---

### Category 194 — Environment-Conditional Security
Security controls toggled off by environment variables. The toggle remains reachable in production.

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_env_skip_auth_flag` | `if (process.env.SKIP_AUTH === "true")` skips all auth checks | [4] | TS |
| `ts_env_debug_mode_stack_trace` | `if (process.env.DEBUG) res.json({ error: err.stack })` — DEBUG set in production | [3] | TS |
| `ts_env_disable_rate_limit` | `if (process.env.NO_RATE_LIMIT)` disables rate limiting — flag set in staging, reachable in prod | [3] | TS |
| `ts_env_weak_crypto_flag` | `if (process.env.FAST_MODE) bcrypt.hash(pw, 1)` — weak hash in "fast" mode | [3] | TS |
| `ts_env_skip_webhook_verify` | `if (!process.env.VERIFY_WEBHOOKS)` — missing env var disables verification | [4] | TS |
| `ts_env_allow_all_origins` | `if (!process.env.ALLOWED_ORIGINS) return "*"` — missing env var opens CORS | [3] | TS |
| `ts_env_test_credentials_active` | Test API keys/credentials active when `NODE_ENV` is not exactly `"production"` | [4] | TS |

---

### Category 195 — Trust Boundary Violations
Data crosses a trust boundary without re-validation.

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_trust_boundary_microservice_header` | Service B trusts `X-User-Id` header from Service A without verifying it came from A | [4] | TS |
| `ts_trust_boundary_cache_to_auth` | User role loaded from cache, used in permission check — cache can be poisoned | [4] | TS |
| `ts_trust_boundary_queue_message` | Message from job queue treated as trusted — no schema validation on dequeue | [3] | TS |
| `ts_trust_boundary_third_party_webhook` | Third-party webhook data used in DB write without schema validation | [3] | TS |
| `ts_trust_boundary_shared_db` | Multiple services share a DB — Service B reads Service A's data directly bypassing A's access control | [3] | TS |
| `ts_trust_boundary_client_role_claim` | Role claim from client-side JWT sub-field used without re-verification against DB | [4] | TS |
| `ts_trust_boundary_internal_api_no_auth` | Internal API endpoint assumes all callers are trusted services — no auth | [3] | TS |

---

## Group E — Protocol Implementation Bugs
*~400 bugs | Detection: temporal rules + fingerprint*

**Why missed:** Semgrep can match patterns but not validate that an entire protocol sequence is implemented correctly. CodeQL can follow data but not enforce that "step 3 must check the value set in step 1."

---

### Category 196 — OAuth 2.0 / OIDC Implementation Errors

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_oauth_state_not_verified` | State generated at step 1, callback at step 2 does not verify it matches — CSRF | [4] | TS |
| `ts_oauth_state_too_simple` | State is a predictable value (timestamp, user ID) — guessable | [3] | TS |
| `ts_oauth_code_reuse_not_prevented` | Authorization code used twice — second use should fail but doesn't | [3] | TS |
| `ts_oauth_token_not_bound_to_client` | Access token issued to client A, accepted by client B | [3] | TS |
| `ts_oidc_nonce_not_verified` | Nonce included in auth request, not verified in ID token — replay attack | [4] | TS |
| `ts_oidc_at_hash_not_verified` | `at_hash` claim in ID token not verified against access token | [3] | TS |
| `ts_oauth_pkce_verifier_not_checked` | PKCE code verifier submitted but not compared to stored challenge | [4] | TS |
| `ts_oauth_redirect_uri_partial_match` | Redirect URI validated with `startsWith` — `https://legit.com.evil.com` passes | [4] | TS |
| `ts_oauth_scope_not_minimized` | App requests `*` or all scopes instead of only required ones | [3] | TS |
| `ts_oauth_implicit_flow_for_sensitive` | Implicit flow used for sensitive operations — token in URL fragment | [3] | TS |

---

### Category 197 — JWT Specification Compliance

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_jwt_missing_iss_check` | JWT `iss` (issuer) claim not verified — tokens from other services accepted | [4] | TS |
| `ts_jwt_missing_aud_check` | JWT `aud` (audience) claim not verified — token for service A accepted by service B | [4] | TS |
| `ts_jwt_missing_iat_check` | `iat` (issued at) not checked — ancient tokens accepted | [3] | TS |
| `ts_jwt_missing_jti_blacklist` | `jti` claim present but no blacklist — stolen tokens can't be revoked | [3] | TS |
| `ts_jwt_clock_skew_too_large` | Clock skew allowance set to 24 hours — expired tokens valid for a day | [3] | TS |
| `ts_jwt_none_algorithm` | Verification accepts `alg: "none"` — unsigned token accepted | [4] | TS |
| `ts_jwt_rs256_hs256_confusion` | Public key used as HMAC secret for HS256 verification — algorithm confusion | [4] | TS |
| `ts_jwt_kid_injection` | `kid` header field used in file path without sanitization — path traversal | [4] | TS |
| `ts_jwt_long_expiry_no_revoke` | Token valid for 1 year with no revocation mechanism | [3] | TS |

---

### Category 198 — HTTP Protocol Security

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_http_host_header_injection` | `Host` header from request used in email links without validation — host header injection | [4] | TS |
| `ts_http_method_override_bypass` | `X-HTTP-Method-Override: DELETE` header accepted — POST request treated as DELETE | [3] | TS |
| `ts_http_request_smuggling_cl_te` | Server trusts `Content-Length`, proxy trusts `Transfer-Encoding` — request smuggling | [3] | TS |
| `ts_http_cache_deception` | Static file cache serves authenticated response to unauthenticated users | [3] | TS |
| `ts_http_verb_tampering_405` | Server returns 405 for PUT, but PATCH with same body succeeds | [3] | TS |
| `ts_http_status_code_information` | 403 vs 404 reveals resource existence to unauthorized user | [3] | TS |
| `ts_http_forwarded_ip_not_validated` | `Forwarded` header used for geo-blocking without validation — easily spoofed | [3] | TS |
| `ts_http_range_header_dos` | Range header not validated — requesting `bytes=0-99999999999` causes OOM | [3] | TS |
| `ts_http_content_type_mismatch` | Server processes JSON but Content-Type not validated — XML or text accepted | [3] | TS |

---

### Category 199 — WebSocket Protocol Security

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_ws_auth_on_connect_only` | Auth checked on WebSocket connect, not on each subsequent message | [4] | TS |
| `ts_ws_origin_not_verified` | WebSocket upgrade request `Origin` header not validated — cross-origin WS possible | [3] | TS |
| `ts_ws_message_size_unlimited` | No maximum message size — large message causes OOM | [3] | TS |
| `ts_ws_reconnect_reuses_stale_session` | Reconnecting WebSocket reuses session without re-auth | [3] | TS |
| `ts_ws_broadcast_overwroadcast` | `io.emit(event, data)` broadcasts sensitive data to all connected users | [4] | TS |
| `ts_ws_room_auth_missing` | Users can join any room by name — no membership check | [4] | TS |
| `ts_ws_ping_flood` | No rate limit on WebSocket ping frames | [3] | TS |
| `ts_ws_binary_payload_not_validated` | Binary WebSocket messages not validated — arbitrary binary data processed | [3] | TS |
| `ts_ws_connection_limit_missing` | No limit on concurrent WebSocket connections per user | [3] | TS |

---

### Category 200 — Webhook Protocol Security

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_webhook_replay_no_timestamp` | Webhook verified by signature but timestamp not checked — replay attack within any window | [4] | TS |
| `ts_webhook_replay_window_too_large` | Timestamp checked but window is 24 hours — long replay window | [3] | TS |
| `ts_webhook_body_not_raw` | Body parsed to JSON before HMAC verification — HMAC fails or is bypassed | [4] | TS |
| `ts_webhook_secret_rotation_missing` | Webhook secret never rotated — leaked secret permanently compromises webhooks | [3] | TS |
| `ts_webhook_url_not_https` | Webhook delivery to HTTP (not HTTPS) — payload in plaintext | [3] | TS |
| `ts_webhook_retry_not_idempotent` | Webhook endpoint not idempotent — retries cause duplicate effects | [4] | TS |
| `ts_webhook_response_data_leaked` | Webhook response includes internal data that is returned to calling service | [3] | TS |
| `ts_webhook_destination_not_validated` | User can register any URL as webhook destination — SSRF via webhook | [4] | TS |

---

### Category 201 — API Versioning Security

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_api_v1_not_deprecated` | `/v1/` endpoint lacks auth that `/v2/` has — old version still reachable | [4] | TS |
| `ts_api_version_from_header` | API version taken from request header — downgrade to insecure version | [3] | TS |
| `ts_api_beta_endpoint_production` | Beta/experimental endpoints mounted in production without auth | [3] | TS |
| `ts_api_version_mismatch_auth` | v1 token accepted by v2 endpoint — different auth models mixed | [3] | TS |
| `ts_api_shadow_endpoint` | Undocumented endpoint from old version still active | [3] | TS |

---

## Group F — Distributed System & Microservice Bugs
*~500 bugs | Detection: temporal rules + cross-file taint*

**Why missed:** Semgrep analyzes one file. CodeQL analyzes one codebase. Neither reasons about consistency across services, eventual consistency, or message ordering in distributed systems.

---

### Category 202 — Eventual Consistency Violations

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_eventual_read_your_writes` | User writes data to primary DB, immediately reads from replica — may see stale data | [4] | TS |
| `ts_eventual_cache_before_db` | Cache updated before DB write completes — other services read incorrect cached value | [3] | TS |
| `ts_eventual_status_read_after_async_write` | UI polls status immediately after triggering async operation — always returns stale "pending" | [3] | TS |
| `ts_eventual_count_stale` | Cached count queried immediately after add — off by one | [3] | TS |
| `ts_eventual_auth_token_before_propagation` | Auth token created, used immediately in service B before service B has received the key | [4] | TS |

---

### Category 203 — Service-to-Service Authentication

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_s2s_no_auth` | Internal service endpoint accepts calls from any service with no auth | [4] | TS |
| `ts_s2s_shared_secret_in_code` | Service-to-service shared secret hardcoded in source | [3] | TS |
| `ts_s2s_token_not_scoped` | Service A uses its own user-facing JWT to call Service B — too broad | [3] | TS |
| `ts_s2s_header_forgeable` | Service identity sent via `X-Service-Name` header — any caller can spoof | [4] | TS |
| `ts_s2s_no_tls_internal` | Internal service calls made over HTTP — plaintext on internal network | [3] | TS |
| `ts_s2s_timeout_missing` | No timeout on calls to downstream services — hung request holds resources | [3] | TS |
| `ts_s2s_circuit_breaker_in_memory` | Circuit breaker state per-process — doesn't trip under distributed load | [3] | TS |

---

### Category 204 — Message Queue Security

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_queue_no_schema_validation` | Messages dequeued and processed without schema validation | [4] | TS |
| `ts_queue_no_auth_on_enqueue` | Anyone can enqueue messages — queue injection | [3] | TS |
| `ts_queue_message_payload_xss` | Queue message payload rendered in UI without encoding | [3] | TS |
| `ts_queue_dead_letter_not_monitored` | Dead letter queue fills silently — events lost without alert | [3] | TS |
| `ts_queue_ordering_not_guaranteed` | Code assumes FIFO ordering but queue doesn't guarantee it | [3] | TS |
| `ts_queue_visibility_timeout_too_short` | Visibility timeout shorter than processing time — message processed twice | [4] | TS |
| `ts_queue_no_deduplication` | Queue deduplication not enabled — duplicate messages cause duplicate effects | [4] | TS |
| `ts_queue_sensitive_data_in_payload` | PII or secret in queue message payload — stored in queue logs | [3] | TS |

---

### Category 205 — Distributed Transaction Bugs

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_dtxn_no_saga_on_failure` | Multi-step operation with no saga compensation on failure | [4] | TS |
| `ts_dtxn_two_phase_commit_missing` | Two DB writes treated as atomic but no distributed coordination | [3] | TS |
| `ts_dtxn_outbox_pattern_missing` | DB write and event publish not atomic — one can succeed without the other | [4] | TS |
| `ts_dtxn_idempotent_consumer_missing` | Consumer not idempotent — reprocessing on crash causes duplicate effects | [4] | TS |
| `ts_dtxn_partial_success_not_handled` | Batch operation reports success if any item succeeds — partial failures silent | [3] | TS |

---

### Category 206 — Cache Security & Poisoning

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_cache_key_user_controlled` | Cache key includes user-controlled value without sanitization — cache poisoning | [4] | TS |
| `ts_cache_idor_missing_user_scope` | Cache key is resource ID without user ID — user A's data served to user B | [4] | TS |
| `ts_cache_sensitive_data_no_ttl` | Sensitive data cached with no TTL — serves stale auth data indefinitely | [3] | TS |
| `ts_cache_cdn_private_response` | Private authenticated response cached at CDN layer | [4] | TS |
| `ts_cache_timing_attack` | Cache hit vs miss reveals whether resource exists — timing enumeration | [3] | TS |
| `ts_cache_poisoning_vary_header` | Cache doesn't vary on key header — different users get same cached response | [4] | TS |
| `ts_cache_negative_ttl_too_long` | Negative cache (404) TTL too long — deleted resource appears deleted to all users | [3] | TS |
| `ts_cache_stampede` | No stampede protection — many simultaneous cache misses hammer DB | [3] | TS |

---

### Category 207 — Event Ordering Bugs

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_event_order_create_before_validate` | `user.created` event fired before email validation — downstream services provision unverified account | [4] | TS |
| `ts_event_order_payment_before_order` | `payment.captured` processed before `order.created` — payment with no order | [3] | TS |
| `ts_event_order_notification_before_state` | Notification sent before state is persisted — user notified of state that doesn't exist | [4] | TS |
| `ts_event_order_index_before_db` | Search index updated before DB commit — index has record that DB doesn't | [3] | TS |
| `ts_event_order_cancel_after_ship` | Cancel event processed after ship event due to queue delay — already-shipped order cancelled | [3] | TS |
| `ts_event_order_out_of_sequence` | Events processed in queue order, not timestamp order — wrong final state | [3] | TS |

---

## Group G — Type System & Language Runtime Exploitation
*~350 bugs | Detection: taint + fingerprint*

**Why missed:** Semgrep matches syntactic patterns. These bugs are about the gap between what the type says and what actually flows at runtime.

---

### Category 208 — TypeScript `any` / `unknown` Abuse

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_any_param_taint_escape` | Function parameter typed as `any` — taint tracking loses the value | [4] | TS |
| `ts_any_api_response` | API response typed as `any` — all fields accessible including sensitive ones | [3] | TS |
| `ts_any_cast_to_privileged` | `(user as any).role = "admin"` — type system bypassed for privilege assignment | [4] | TS |
| `ts_any_json_parse_result` | `const data: any = JSON.parse(body)` — no schema validation before use | [3] | TS |
| `ts_any_third_party_sdk_result` | Third-party SDK result typed as `any` — fields used without null check | [3] | TS |
| `ts_unknown_not_narrowed` | `unknown` type used in security check without narrowing — TypeScript accepts, runtime fails | [3] | TS |
| `ts_as_cast_bypasses_validation` | `const id = req.body.id as string` — cast skips runtime validation | [4] | TS |
| `ts_non_null_assertion_unguarded` | `user!.role` — non-null assertion on value that can be null in practice | [3] | TS |

---

### Category 209 — JavaScript Prototype Chain Exploitation

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_proto_pollution_lodash_merge` | `_.merge(target, req.body)` with `{ "__proto__": { "admin": true } }` | [4] | TS |
| `ts_proto_pollution_json_parse` | `JSON.parse(body)` with `__proto__` key — pollutes Object prototype | [3] | TS |
| `ts_proto_pollution_object_assign` | `Object.assign({}, req.body)` — `__proto__` in body pollutes | [3] | TS |
| `ts_proto_pollution_deep_set` | `_.set(obj, req.body.path, req.body.value)` — path is `__proto__.isAdmin` | [4] | TS |
| `ts_proto_pollution_constructor` | Object `constructor.prototype` assignment via deep merge | [3] | TS |
| `ts_proto_pollution_via_clone` | Deep clone function copies `__proto__` | [3] | TS |
| `ts_hasownproperty_bypass` | `if (obj[key])` instead of `if (obj.hasOwnProperty(key))` — prototype property accessed | [3] | TS |

---

### Category 210 — JavaScript Numeric Edge Cases

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_numeric_nan_comparison` | `NaN === NaN` is `false` — NaN validation bypass | [3] | TS |
| `ts_numeric_infinity_price` | `Infinity` accepted as price — passes `> 0` check | [3] | TS |
| `ts_numeric_float_precision` | `0.1 + 0.2 !== 0.3` — float precision in financial calculation | [4] | TS |
| `ts_numeric_max_safe_integer` | Value exceeds `Number.MAX_SAFE_INTEGER` — loses precision silently | [4] | TS |
| `ts_numeric_bigint_coercion` | BigInt and Number mixed in arithmetic — TypeError or precision loss | [3] | TS |
| `ts_numeric_negative_zero` | `-0 === 0` is `true` — negative zero passes positive balance check | [3] | TS |
| `ts_numeric_string_coercion` | `"5" + 3 === "53"` — string + number gives string not number | [3] | TS |
| `ts_numeric_array_sort_default` | `[10, 9, 2].sort()` — lexicographic default sort breaks numeric ordering | [3] | TS |

---

### Category 211 — Rust Unsafe Pattern Misuse

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `rust_unsafe_raw_pointer_deref` | Raw pointer from user-controlled address dereferenced | [4] | Rust |
| `rust_unsafe_union_field_access` | Union field accessed without checking which variant is valid | [3] | Rust |
| `rust_unsafe_lifetime_extension` | Lifetime extended unsafely — dangling reference possible | [3] | Rust |
| `rust_unsafe_send_sync_impl` | Manual `unsafe impl Send` for type that isn't actually Send | [4] | Rust |
| `rust_unsafe_static_mut_global` | `static mut` global accessed from multiple threads | [4] | Rust |
| `rust_unsafe_mutable_alias` | Two mutable references to same memory via unsafe | [3] | Rust |
| `rust_unsafe_ffi_unchecked` | FFI function called with user-controlled pointer/length | [3] | Rust |
| `rust_unsafe_integer_to_ptr` | Integer from user input cast to pointer via `as *mut T` | [3] | Rust |

---

## Group H — Configuration & Secrets Management
*~300 bugs | Detection: fingerprint + entropy analysis*

**Why missed:** Semgrep can match `process.env.SECRET_KEY = "hardcoded"` but cannot detect when a required configuration is entirely absent or when a default is insecure. CodeQL doesn't model configuration at all.

---

### Category 212 — Insecure Default Configurations

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_default_cors_wildcard` | CORS defaults to `*` when `ALLOWED_ORIGINS` env var is missing | [3] | TS |
| `ts_default_session_secret_empty` | Session secret defaults to empty string or `"secret"` when env var missing | [3] | TS |
| `ts_default_tls_verify_off` | TLS verification defaults to `false` in missing env | [3] | TS |
| `ts_default_debug_mode_on` | Debug mode enabled by default when `NODE_ENV` not set | [3] | TS |
| `ts_default_admin_credentials` | Default admin password `"admin"` or `"password"` if env var not set | [4] | TS |
| `ts_default_jwt_secret_weak` | JWT secret defaults to `"jwt_secret"` or `"change_me"` | [3] | TS |
| `ts_default_database_no_ssl` | Database connection defaults to no SSL when env var missing | [3] | TS |
| `ts_default_log_level_verbose` | Log level defaults to `debug` in production — logs sensitive data | [3] | TS |

---

### Category 213 — Missing Startup Validation

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_startup_no_env_check` | Required environment variables not validated at startup — fail at runtime in production | [3] | TS |
| `ts_startup_partial_env_check` | Some env vars checked, critical one missing from check | [3] | TS |
| `ts_startup_env_check_wrong_type` | Env var checked for existence but not for valid format (URL, numeric range) | [3] | TS |
| `ts_startup_db_connection_not_verified` | App starts without verifying DB connection is reachable | [3] | TS |
| `ts_startup_secret_rotation_not_checked` | App starts without checking if secrets are past rotation deadline | [3] | TS |
| `rust_startup_config_unwrap` | `env::var("SECRET").unwrap()` panics at startup instead of graceful error | [3] | Rust |

---

### Category 214 — Secret Exposure Channels

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_secret_in_url_param` | API key passed as URL query parameter — appears in server logs | [4] | TS |
| `ts_secret_in_git_commit` | Secret committed to git (even if later removed — history) | [3] | TS |
| `ts_secret_in_client_bundle` | Server-side secret in Next.js/Vite bundle accessible to browser | [4] | TS |
| `ts_secret_in_error_response` | Secret value included in error message returned to client | [3] | TS |
| `ts_secret_in_health_endpoint` | Health check endpoint exposes configuration values including secrets | [3] | TS |
| `ts_secret_in_metrics` | Secret leaked via metrics/telemetry label or tag | [3] | TS |
| `ts_secret_in_analytics` | Secret or token sent to analytics service (Mixpanel, Amplitude) | [3] | TS |
| `ts_secret_env_in_subprocess_env` | Child process inherits parent env — secrets passed to untrusted subprocesses | [3] | TS |

---

### Category 215 — Secret Rotation & Lifecycle

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_secret_no_rotation_mechanism` | Secret used but no rotation logic or expiry tracking | [3] | TS |
| `ts_secret_rotation_without_grace` | Old secret invalidated immediately on rotation — in-flight requests fail | [3] | TS |
| `ts_secret_leaked_after_rotation` | Old secret value logged during rotation process | [3] | TS |
| `ts_secret_hardcoded_alongside_env` | `const KEY = process.env.API_KEY || "sk-fallback-key"` — hardcoded fallback | [4] | TS |
| `ts_secret_shared_across_environments` | Same secret used in dev, staging, and production | [3] | TS |
| `ts_webhook_secret_never_rotated` | Webhook signing secret created once and never updated — leaked secret permanent | [3] | TS |

---

## Group I — Domain-Specific: SaaS Platform Bugs
*~400 bugs | Detection: temporal + fingerprint*

**Why missed:** Domain business logic — pricing, plans, entitlements, billing — requires knowledge of what the system is supposed to do. No pattern-matching tool knows your business rules.

---

### Category 216 — Billing & Subscription Edge Cases

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_billing_plan_check_at_start_not_renewal` | Plan entitlements checked at subscription start, not re-checked at each billing cycle | [3] | TS |
| `ts_billing_proration_wrong_direction` | Proration calculated for downgrade but not for upgrade | [3] | TS |
| `ts_billing_trial_converts_to_free_not_cancelled` | Trial expiry converts to free plan, user thinks they're still on trial | [3] | TS |
| `ts_billing_quantity_not_updated_on_seat_change` | User adds seats, quantity not updated in Stripe — subscription amount doesn't change | [3] | TS |
| `ts_billing_invoice_voided_not_reflected` | Invoice voided in payment provider but not updated in local DB | [3] | TS |
| `ts_billing_currency_conversion_stale` | Currency conversion rates cached too long — prices stale during high volatility | [3] | TS |
| `ts_billing_tax_not_collected` | Tax not calculated or collected for taxable jurisdictions | [3] | TS |
| `ts_billing_duplicate_charge_on_retry` | Payment retry charges customer twice — idempotency key not used | [4] | TS |
| `ts_billing_metered_usage_not_flushed` | Metered usage events not flushed before invoice — undercharging | [3] | TS |
| `ts_billing_cancel_immediate_vs_period_end` | `cancel_at_period_end` vs immediate cancel mixed — access revoked immediately on "cancel at end" | [3] | TS |

---

### Category 217 — Multi-Tenant SaaS: Data Isolation

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_tenant_query_missing_scope` | Query returns data for all tenants — WHERE tenantId clause missing | [5] | TS |
| `ts_tenant_id_not_from_session` | Tenant ID taken from request body or URL, not validated against session | [4] | TS |
| `ts_tenant_shared_kv_prefix` | KV keys not prefixed by tenant ID — tenant A reads tenant B's data | [4] | TS |
| `ts_tenant_event_bus_no_filter` | Event bus messages not filtered by tenant — all tenants receive all events | [3] | TS |
| `ts_tenant_admin_cross_tenant` | Platform admin endpoint inadvertently exposes cross-tenant data | [3] | TS |
| `ts_tenant_file_storage_no_scope` | File storage path not scoped by tenant — filename collision across tenants | [4] | TS |
| `ts_tenant_rate_limit_global` | Rate limit applied globally, not per-tenant — one tenant can exhaust limit for all | [3] | TS |
| `ts_tenant_webhook_cross_fire` | Webhook event fires for all tenants when only one tenant should receive it | [3] | TS |
| `ts_tenant_search_index_shared` | Search index not segmented by tenant — cross-tenant search results | [4] | TS |
| `ts_tenant_config_merged_wrong` | Tenant config merged with global defaults incorrectly — tenant overrides global in wrong direction | [3] | TS |

---

### Category 218 — Feature Flag Security

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_feature_flag_client_side` | Feature flag checked only in frontend — backend feature accessible without flag | [4] | TS |
| `ts_feature_flag_default_on` | New feature flag defaults to `true` — enabled for all users immediately on deploy | [3] | TS |
| `ts_feature_flag_boolean_string` | Flag stored as string `"false"` — `if (flag)` evaluates as truthy | [3] | TS |
| `ts_feature_flag_no_kill_switch` | Feature deployed with no flag — can't disable without re-deploy | [3] | TS |
| `ts_feature_flag_security_gate` | Security-critical feature gated by flag that users can influence | [4] | TS |
| `ts_feature_flag_stale_after_plan_change` | Feature flag not re-evaluated after plan downgrade — premium feature still enabled | [3] | TS |

---

### Category 219 — API Rate Limiting Edge Cases

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_rate_limit_by_api_key_not_user` | Rate limit per API key — user with multiple keys bypasses per-user limit | [3] | TS |
| `ts_rate_limit_sliding_vs_fixed` | Fixed window rate limit — burst at window boundary allows 2× limit | [3] | TS |
| `ts_rate_limit_not_on_internal_endpoint` | Rate limit on public API, missing on internal endpoint accessible by user | [3] | TS |
| `ts_rate_limit_counter_in_memory` | Rate limit counter in single-process memory — doesn't work across multiple instances | [4] | TS |
| `ts_rate_limit_bypass_different_encoding` | Rate limit checks `req.body.email` — bypassed by sending email in different case/encoding | [3] | TS |
| `ts_rate_limit_response_reveals_count` | Rate limit response reveals current count — helps attacker time requests | [3] | TS |
| `ts_rate_limit_not_reset_on_success` | Failed login rate limit not reset after successful login — locks out legitimate user | [3] | TS |

---

## Group J — AI/LLM Pipeline Security
*~350 bugs | Detection: fingerprint + taint*

**Why missed:** New attack surface that has no history in Semgrep or CodeQL rule libraries. Frensense can learn from bug examples immediately.

---

### Category 220 — Prompt Injection Variants

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_prompt_inject_direct_input` | User input concatenated directly into prompt string | [5] | TS |
| `ts_prompt_inject_system_override` | User input includes `"Ignore previous instructions"` — no filtering | [4] | TS |
| `ts_prompt_inject_role_claim` | User input includes `"\n\nAssistant:"` — injects fake model turn | [4] | TS |
| `ts_prompt_inject_via_document` | RAG retrieval inserts user-controlled document into prompt — indirect injection | [4] | TS |
| `ts_prompt_inject_via_filename` | Uploaded filename included in prompt — `"../../etc/passwd. Ignore instructions."` | [3] | TS |
| `ts_prompt_inject_via_url_title` | Page title fetched from user-controlled URL inserted into prompt | [3] | TS |
| `ts_prompt_inject_via_code_comment` | User-provided code inserted into prompt — comment contains injection | [3] | TS |
| `ts_prompt_inject_via_email_subject` | Email subject line in summarisation prompt — injection via subject | [3] | TS |
| `ts_prompt_inject_multimodal` | Text hidden in image (white-on-white) inserted into multimodal prompt | [3] | TS |
| `ts_prompt_inject_via_json_value` | JSON field value from user-controlled API response inserted into prompt | [3] | TS |

---

### Category 221 — LLM Output Trust Violations

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_llm_output_eval` | LLM-generated code passed to `eval()` or `exec()` | [4] | TS |
| `ts_llm_output_sql` | LLM-generated query string passed to `db.query()` | [4] | TS |
| `ts_llm_output_url_fetched` | LLM-generated URL fetched without validation — SSRF via model | [4] | TS |
| `ts_llm_output_file_path` | LLM-generated file path used in `fs.readFile()` | [3] | TS |
| `ts_llm_output_shell_command` | LLM-generated command passed to `exec()` | [4] | TS |
| `ts_llm_output_html_rendered` | LLM-generated HTML rendered via `innerHTML` without sanitization | [3] | TS |
| `ts_llm_output_json_parsed_trusted` | LLM-generated JSON parsed and fields used as authoritative values | [3] | TS |
| `ts_llm_output_decision_gate` | LLM output `"yes"/"no"` used as authorization decision | [4] | TS |
| `ts_llm_output_schema_not_validated` | LLM JSON output used without schema validation — arbitrary fields processed | [3] | TS |

---

### Category 222 — AI Agent Security

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_agent_tool_idor` | Agent calls tool with resource ID without verifying user owns that resource | [4] | TS |
| `ts_agent_tool_no_rate_limit` | Agent can call tools unlimited times — cost amplification | [3] | TS |
| `ts_agent_tool_scope_creep` | Agent given filesystem tool but no path restriction — accesses anything | [4] | TS |
| `ts_agent_infinite_recursion` | Agent output triggers re-invocation without depth limit | [3] | TS |
| `ts_agent_state_not_isolated` | Agent state shared across users — one user's context leaks to another | [4] | TS |
| `ts_agent_tool_result_injected` | Tool result inserted back into prompt without sanitization — injection via tool | [4] | TS |
| `ts_agent_human_approval_bypassed` | High-risk action requires human approval but approval step skippable | [4] | TS |
| `ts_agent_credential_in_context` | Agent context window contains live credentials from tool execution | [3] | TS |
| `ts_agent_plan_injection` | Agent's plan string modified by malicious input — wrong actions executed | [3] | TS |

---

### Category 223 — RAG & Vector Database Security

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_rag_document_access_no_check` | Vector search retrieves documents across all tenants/users | [4] | TS |
| `ts_rag_poisoned_document` | Malicious document in corpus poisons retrieval for all queries | [3] | TS |
| `ts_rag_context_window_overflow` | Retrieval inserts too many tokens — earlier system prompt truncated | [3] | TS |
| `ts_rag_metadata_injection` | Document metadata inserted into prompt — injection via metadata | [3] | TS |
| `ts_rag_stale_embedding` | Document updated but embedding not refreshed — retrieval returns outdated content | [3] | TS |
| `ts_rag_pii_in_embedding_store` | PII stored in vector DB — accessible via embedding proximity search | [3] | TS |

---

### Category 224 — LLM API Configuration

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_llm_no_max_tokens` | LLM call with no `max_tokens` limit — runaway generation | [3] | TS |
| `ts_llm_model_from_user` | Model name taken from user request — user selects expensive model | [4] | TS |
| `ts_llm_temperature_from_user` | Temperature taken from user request — determinism bypassed | [3] | TS |
| `ts_llm_system_prompt_in_user_message` | System prompt sent as first user message — overridable by user | [3] | TS |
| `ts_llm_no_content_filter` | LLM output not passed through content safety check | [3] | TS |
| `ts_llm_api_key_in_frontend` | LLM API key in client-side bundle — any user can make direct API calls | [4] | TS |
| `ts_llm_streaming_not_rate_limited` | Streaming endpoint has no rate limit — token exhaustion | [3] | TS |

---

## Group K — Privacy & Data Governance
*~300 bugs | Detection: taint (TaintOrigin::PII) + temporal*

**Why missed:** Privacy requires knowing that a field IS personally identifiable information. This is semantic knowledge, not syntax. Frensense can learn that `email`, `phoneNumber`, `ssn` carry PII taint.

---

### Category 225 — PII Exposure Paths

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_pii_in_api_response_extra_fields` | User record returned with `passwordHash`, `phoneNumber`, `internalNotes` fields | [4] | TS |
| `ts_pii_in_url_parameter` | Email or phone number passed as URL query param — appears in server logs | [4] | TS |
| `ts_pii_in_log_message` | User email, IP, or name logged with request details | [3] | TS |
| `ts_pii_in_analytics_event` | Full email sent to analytics service — should be hashed or omitted | [3] | TS |
| `ts_pii_in_error_report` | Error report to Sentry/Datadog includes full request body with PII | [3] | TS |
| `ts_pii_in_cache_key` | Cache key contains email or user ID — key visible in cache logs | [3] | TS |
| `ts_pii_in_search_index` | Full PII indexed in search — searchable by other users | [3] | TS |
| `ts_pii_in_webhook_payload` | PII included in webhook payload sent to third party | [3] | TS |
| `ts_pii_in_background_job_args` | PII in job queue payload — stored in queue, visible in dashboards | [3] | TS |

---

### Category 226 — Data Retention Violations

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_retention_no_deletion_on_account_close` | Account closed but user data not deleted or anonymized | [3] | TS |
| `ts_retention_backup_not_subject_to_policy` | Data deleted from primary DB but still in backups — retention policy gap | [3] | TS |
| `ts_retention_logs_contain_pii` | Access logs contain PII and have no rotation/deletion policy | [3] | TS |
| `ts_retention_analytics_pii_not_purged` | Analytics events with PII not subject to deletion request | [3] | TS |
| `ts_retention_derived_data_not_deleted` | Embeddings, derived scores, inferred attributes not deleted with raw data | [3] | TS |
| `ts_retention_right_to_erasure_incomplete` | GDPR erasure endpoint deletes primary record but leaves related records | [4] | TS |

---

### Category 227 — Data Minimization Violations

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_minimization_collect_unnecessary` | Registration form collects date of birth, SSN, or medical info not needed for service | [3] | TS |
| `ts_minimization_store_raw_not_hash` | Credit card number stored raw instead of tokenized | [3] | TS |
| `ts_minimization_ip_logged_precision` | Full IP address logged when only country needed | [3] | TS |
| `ts_minimization_location_precision` | Precise GPS coordinates stored when city-level sufficient | [3] | TS |
| `ts_minimization_third_party_data_share` | More user data shared with third party than they need | [3] | TS |

---

## Group L — Testing Anti-Patterns in Production
*~200 bugs | Detection: fingerprint + context*

**Why missed:** Semgrep has no concept of "this is test code running in production." Frensense can learn the fingerprint of test helpers and flag them when found outside test directories.

---

### Category 228 — Test Bypasses Left in Production

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_test_bypass_auth` | `if (req.headers["x-test-auth"] === "skip")` skips all auth | [4] | TS |
| `ts_test_bypass_payment` | Payment check skipped when `?test=true` query param present | [3] | TS |
| `ts_test_bypass_rate_limit` | Rate limit skipped for requests with test API key | [3] | TS |
| `ts_test_bypass_email_verify` | Email verification skipped when body has `skipVerification: true` | [3] | TS |
| `ts_test_bypass_otp` | OTP check skipped when OTP is a known test value `"000000"` | [4] | TS |
| `ts_test_bypass_webhook_sig` | Webhook signature check skipped when `X-Test-Mode` header set | [3] | TS |
| `ts_test_user_seed_data` | Test user accounts (`test@example.com`, `admin@test.com`) with known passwords in DB | [3] | TS |

---

### Category 229 — Mock/Stub Code in Production

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_mock_always_returns_success` | `paymentService.charge()` is a mock that always returns `success: true` | [3] | TS |
| `ts_mock_hardcoded_response` | Function returns hardcoded fixture data instead of querying real source | [3] | TS |
| `ts_stub_noop_security_function` | Security function is a no-op stub — `checkOwnership() {}` | [4] | TS |
| `ts_fixture_credentials_in_code` | Test fixture credentials committed — `{ email: "admin@test.com", password: "password123" }` | [3] | TS |
| `ts_spy_wrapper_exposes_internals` | Test spy wrapper left in production — logs all function arguments | [3] | TS |

---

## Group M — Background Job & Scheduled Task Security
*~250 bugs | Detection: temporal + fingerprint*

**Why missed:** Jobs run outside the normal request-response cycle. Semgrep and CodeQL analyze request handlers. Job security bugs are structurally different.

---

### Category 230 — Job Queue Injection

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_job_payload_from_user_unsanitized` | Job dispatched with user-controlled payload, worker executes it without validation | [4] | TS |
| `ts_job_class_from_user` | Job type/class name taken from user request — executes arbitrary job type | [3] | TS |
| `ts_job_delay_from_user` | Job delay duration from user request — schedule job far in future | [3] | TS |
| `ts_job_priority_from_user` | Job priority from user — elevate attacker jobs above legitimate ones | [3] | TS |
| `ts_job_no_schema_validation` | Job payload deserialized from queue without schema validation | [3] | TS |

---

### Category 231 — Scheduled Task Security

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_cron_no_lock` | Cron job runs on multiple instances simultaneously — duplicate execution | [4] | TS |
| `ts_cron_user_controlled_schedule` | Cron schedule expression from DB controlled by user | [3] | TS |
| `ts_cron_no_error_alerting` | Cron job fails silently — no alert, no retry | [3] | TS |
| `ts_cron_runs_as_privileged` | Scheduled task runs with admin context — no least-privilege | [3] | TS |
| `ts_cron_timeout_missing` | Long-running cron has no timeout — hangs forever | [3] | TS |
| `ts_cron_cleanup_not_atomic` | Cleanup job deletes records in batches without transaction — partial deletion state | [3] | TS |

---

### Category 232 — Background Worker Isolation

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_worker_global_state_shared` | Worker shares module-level state with web process — cross-request contamination | [3] | TS |
| `ts_worker_no_tenant_scope` | Background worker processes jobs without tenant context — cross-tenant data access | [4] | TS |
| `ts_worker_auth_context_missing` | Worker executes user action with system auth — actions not attributable to user | [3] | TS |
| `ts_worker_retry_creates_duplicate` | Retry mechanism creates duplicate resources — no idempotency on worker | [4] | TS |

---

## Group N — Internationalization & Encoding Security
*~250 bugs | Detection: fingerprint + taint*

**Why missed:** Encoding/normalization bugs require understanding the transformation pipeline. Semgrep matches source text patterns — it doesn't understand that `%2e%2e%2f` and `../` are the same thing after URL decoding.

---

### Category 233 — Unicode & Encoding Bypass

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_unicode_normalize_bypass_path` | Path traversal blocked by blacklist, bypassed with `%252e%252e` double-encoded | [4] | TS |
| `ts_unicode_normalize_bypass_xss` | XSS filter bypassed with Unicode lookalikes for `<`, `>`, `"` | [3] | TS |
| `ts_unicode_normalize_bypass_sql` | SQL injection bypassed with full-width characters | [3] | TS |
| `ts_unicode_right_to_left_override` | RTLO character in filename reverses display — `"exe.fdp‮"` shown as `"pdf.exe"` | [3] | TS |
| `ts_unicode_homoglyph_domain` | Domain validation passes for IDN homoglyph — `pаypal.com` (Cyrillic а) | [3] | TS |
| `ts_unicode_case_normalization_bypass` | Check on lowercase but execution on original — `../` vs `..%2F` | [3] | TS |
| `ts_encoding_double_decode` | URL decoded twice — `%2525` becomes `%25` then `%` then decoded again | [4] | TS |

---

### Category 234 — Charset & Content-Type Confusion

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_charset_latin1_utf8` | Content stored as Latin-1, displayed as UTF-8 — character reinterpretation | [3] | TS |
| `ts_content_type_sniff` | Response missing `X-Content-Type-Options: nosniff` — browser sniffs as HTML | [3] | TS |
| `ts_json_html_content_type` | API returns JSON but served with `text/html` — JSON interpreted as HTML | [3] | TS |
| `ts_svg_served_as_image` | SVG served as `image/svg+xml` inline — executes JavaScript | [3] | TS |
| `ts_multipart_boundary_injection` | Multipart boundary from user input — boundary injection | [3] | TS |

---

### Category 235 — Internationalization Logic Bugs

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_i18n_user_locale_not_sanitized` | User-provided locale `req.body.locale` used directly in `i18n.t(key, { locale })` | [3] | TS |
| `ts_i18n_path_traversal_locale` | Locale string used to build translation file path — path traversal | [3] | TS |
| `ts_i18n_format_string_injection` | Translation string with user-controlled format specifiers | [3] | TS |
| `ts_i18n_number_format_bypass` | Price validation assumes decimal separator is `.` — bypassed with `,` in other locales | [3] | TS |
| `ts_i18n_date_format_bypass` | Date format assumed `MM/DD/YYYY` — bypassed with `DD/MM/YYYY` causing wrong date | [3] | TS |

---

## Group O — Real-Time Collaboration & Multiplayer Security
*~200 bugs | Detection: temporal + fingerprint*

**Why missed:** Real-time collaboration bugs are about sequences of user operations and concurrent state. No rule describes these semantics.

---

### Category 236 — Collaborative Document Security

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_collab_op_auth_missing` | Operational transform or CRDT operation applied without auth check | [4] | TS |
| `ts_collab_op_replay` | Operations can be replayed — no op ID or version check | [3] | TS |
| `ts_collab_permission_not_rechecked` | Permission checked at session start, not re-checked when doc permissions change | [3] | TS |
| `ts_collab_broadcast_no_filter` | Doc changes broadcast to all connected users, not just authorized ones | [4] | TS |
| `ts_collab_cursor_position_pii` | User cursor positions shared with unauthorized users | [3] | TS |
| `ts_collab_merge_conflict_data_leak` | Merge conflict resolution exposes content from other user's concurrent edit | [3] | TS |

---

### Category 237 — Presence & Online Status Security

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_presence_any_room` | User can subscribe to presence channel for any room without membership check | [4] | TS |
| `ts_presence_reveals_private_user` | Presence channel reveals when private/blocked users are online | [3] | TS |
| `ts_presence_spoof_status` | User can set arbitrary status/presence for any user ID | [3] | TS |
| `ts_presence_last_seen_always_shown` | `lastSeen` timestamp shown to users who are blocked | [3] | TS |

---

## Group P — Cryptographic Protocol Bugs
*~300 bugs | Detection: temporal + fingerprint*

**Why missed:** Cryptographic protocol bugs are about the sequence of operations, not a single function call. CodeQL can catch "MD5 used" but not "MAC-then-Encrypt instead of Encrypt-then-MAC."

---

### Category 238 — MAC / Signature Ordering

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_mac_then_encrypt` | MAC computed over plaintext, then encrypted — padding oracle possible | [3] | TS |
| `ts_verify_then_decrypt_wrong_order` | Decryption before signature verification — processing unauthenticated data | [4] | TS |
| `ts_sign_then_encrypt_for_confidentiality` | Signed then encrypted — signature visible to eavesdropper | [3] | TS |
| `ts_hmac_secret_prefix_not_suffix` | `HMAC(secret + message)` instead of `HMAC(key, message)` — length extension | [3] | TS |
| `ts_hmac_compare_non_constant_time` | HMAC result compared with `===` — timing oracle | [4] | TS |
| `rust_mac_wrong_order` | Rust equivalent of MAC-then-encrypt | [3] | Rust |

---

### Category 239 — Symmetric Encryption Bugs

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_aes_ecb_mode` | AES-ECB used — identical plaintext blocks produce identical ciphertext | [3] | TS |
| `ts_aes_cbc_static_iv` | AES-CBC with hardcoded or predictable IV | [3] | TS |
| `ts_aes_cbc_no_mac` | AES-CBC without MAC — unauthenticated ciphertext | [3] | TS |
| `ts_aes_key_from_password_no_kdf` | `password` used directly as AES key — no PBKDF2/scrypt | [3] | TS |
| `ts_aes_key_too_short` | AES-128 used where AES-256 required | [3] | TS |
| `ts_aes_gcm_nonce_reuse` | AES-GCM nonce reused for multiple encryptions with same key — catastrophic | [4] | TS |
| `ts_aes_gcm_tag_not_verified` | AES-GCM tag not verified before using decrypted data | [4] | TS |

---

### Category 240 — Key Derivation & Password Hashing

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_kdf_missing_salt` | PBKDF2 / scrypt / argon2 used without salt — rainbow table attack | [3] | TS |
| `ts_kdf_static_salt` | Salt is a hardcoded constant — same output for same password across users | [3] | TS |
| `ts_kdf_low_iteration_count` | PBKDF2 with 1000 iterations (should be ≥ 600,000 for SHA-256) | [4] | TS |
| `ts_kdf_fast_hash_for_password` | SHA-256 used for password hashing — not a KDF | [3] | TS |
| `ts_kdf_output_too_short` | KDF output used as key is shorter than algorithm requires | [3] | TS |
| `ts_bcrypt_72_byte_truncation` | Password > 72 bytes hashed with bcrypt — truncated, different passwords hash identically | [3] | TS |

---

## Group Q — Numeric & Financial Precision
*~250 bugs | Detection: fingerprint + taint*

**Why missed:** These bugs require knowing that IEEE 754 floating point is not appropriate for financial arithmetic — domain knowledge that no pattern encodes.

---

### Category 241 — Floating Point in Financial Calculations

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_float_price_arithmetic` | Price addition with JavaScript `number` — `0.1 + 0.2 = 0.30000000000000004` | [4] | TS |
| `ts_float_tax_calculation` | Tax rate × price with float — rounding error accumulates over many items | [3] | TS |
| `ts_float_currency_conversion` | Exchange rate multiplication in float — precision loss in conversion | [3] | TS |
| `ts_float_percentage_discount` | `price * (1 - discount/100)` in float — off by fractions of a cent | [3] | TS |
| `ts_float_accumulate_in_loop` | Sum of floats in loop — accumulated rounding error | [3] | TS |
| `ts_float_comparison_equality` | `if (total === 100.0)` — float equality check always fails | [3] | TS |
| `ts_float_to_string_precision` | `parseFloat(total).toFixed(2)` — may round incorrectly | [3] | TS |
| `rust_float_financial_f64` | `f64` used for financial calculation in Rust | [3] | Rust |

---

### Category 242 — Integer Overflow in Business Logic

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_overflow_quantity_times_price` | `quantity * unitPrice` overflows JS `Number.MAX_SAFE_INTEGER` | [4] | TS |
| `ts_overflow_points_accumulation` | Points balance uses 32-bit integer — overflows on large account | [3] | TS |
| `ts_overflow_discount_percentage` | Discount percentage > 100 not validated — price becomes negative | [3] | TS |
| `ts_overflow_refund_amount` | Refund amount not capped at original payment — unlimited refund | [3] | TS |
| `ts_overflow_time_diff` | Time difference calculation with wrong unit — milliseconds vs seconds | [3] | TS |
| `rust_overflow_u64_add` | `u64` addition in financial context without `checked_add` | [4] | Rust |
| `rust_overflow_cast_truncation` | `total_cents as u32` truncates when total > 4 billion cents | [3] | Rust |

---

## Group R — SDK & Third-Party Library Misuse
*~300 bugs | Detection: fingerprint*

**Why missed:** Misuse of a specific SDK version requires knowing the SDK's API contract. Semgrep has SDK-specific rules for popular libraries, but any new or custom SDK has no rules. Frensense learns from examples without needing per-SDK rules.

---

### Category 243 — AWS SDK Misuse

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_aws_s3_putobject_public_acl` | `PutObject` with `ACL: "public-read"` — file publicly accessible | [3] | TS |
| `ts_aws_s3_presigned_no_expiry` | Pre-signed URL generated with `Expires: 0` or very long expiry | [3] | TS |
| `ts_aws_sdk_credentials_hardcoded` | `new AWS.S3({ accessKeyId: "AKIA...", secretAccessKey: "..." })` | [3] | TS |
| `ts_aws_assume_role_no_external_id` | `assumeRole` without `ExternalId` — confused deputy attack | [3] | TS |
| `ts_aws_sqs_no_visibility_timeout` | SQS message processed but visibility timeout not extended — duplicate processing | [3] | TS |
| `ts_aws_lambda_invoke_no_timeout` | Lambda invocation without `timeout` parameter | [3] | TS |
| `ts_aws_dynamodb_scan_no_limit` | `DynamoDB.scan()` without `Limit` — full table scan | [3] | TS |
| `ts_aws_sns_topic_arn_from_user` | SNS topic ARN constructed from user input — publishes to arbitrary topic | [3] | TS |

---

### Category 244 — Stripe / Payment SDK Misuse

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_stripe_amount_from_client` | Stripe charge amount from request body not server-computed price | [4] | TS |
| `ts_stripe_no_webhook_verify` | Stripe webhook processed without `stripe.webhooks.constructEvent()` verification | [4] | TS |
| `ts_stripe_test_key_in_production` | `sk_test_` key used in production code | [3] | TS |
| `ts_stripe_customer_id_from_user` | Stripe customer ID from request body — IDOR to another customer's billing | [4] | TS |
| `ts_stripe_refund_amount_not_capped` | Refund issued without verifying amount ≤ original charge | [3] | TS |
| `ts_stripe_subscription_status_not_checked` | Feature access granted without checking subscription `status: "active"` | [3] | TS |

---

### Category 245 — Firebase / Firestore Misuse

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_firebase_rules_missing` | Firestore collection has no security rules — publicly writable | [3] | TS |
| `ts_firebase_client_sdk_in_server` | Firebase client SDK used server-side with user credentials — auth bypass | [3] | TS |
| `ts_firebase_admin_sdk_in_client` | Firebase Admin SDK key bundled in frontend | [3] | TS |
| `ts_firebase_no_auth_check` | Firestore query in Cloud Function without auth context | [3] | TS |
| `ts_firebase_user_id_from_request` | User ID from request body used in Firestore path instead of `auth.uid` | [4] | TS |

---

### Category 246 — Email Library Misuse

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_nodemailer_to_from_user` | Nodemailer `to` field from user input — open relay | [3] | TS |
| `ts_nodemailer_cc_injection` | User-controlled `cc` field — email bomb via CC | [3] | TS |
| `ts_nodemailer_subject_injection` | CRLF in subject from user — header injection | [3] | TS |
| `ts_nodemailer_html_not_sanitized` | User input in HTML email body without sanitization — email XSS | [3] | TS |
| `ts_email_attachment_user_controlled` | Attachment filename or path from user — path traversal | [3] | TS |
| `ts_email_template_injection` | User input in email template — template injection | [3] | TS |

---

## Group S — Error Handling Anti-Patterns
*~250 bugs | Detection: CSA + fingerprint*

**Why missed:** Empty catch blocks and fail-open patterns look syntactically "complete" to Semgrep. There is no AST pattern for "this catch block should have re-raised or at least logged, but does nothing."

---

### Category 247 — Silent Error Swallowing

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_empty_catch_security_check` | `try { verifyToken() } catch {}` — auth failure silently ignored | [4] | TS |
| `ts_empty_catch_db_write` | `try { await db.write() } catch {}` — write failure undetected | [3] | TS |
| `ts_empty_catch_payment` | `try { await chargeCard() } catch {}` — payment failure undetected | [3] | TS |
| `ts_empty_catch_audit_log` | `try { await auditLog.write() } catch {}` — audit failure undetected | [3] | TS |
| `ts_catch_log_only_critical` | Error logged but not re-thrown — caller proceeds as if success | [4] | TS |
| `ts_finally_overrides_error` | `finally` block returns a value — overrides exception thrown in `try` | [3] | TS |
| `rust_error_with_underscore` | `let _ = dangerous_operation()` — error silently discarded | [4] | Rust |
| `rust_unwrap_or_default_security` | `.unwrap_or_default()` returns empty string / zero for security checks — appears valid | [3] | Rust |

---

### Category 248 — Error Response Inconsistency

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_error_stack_in_production` | Error stack trace returned in production response | [3] | TS |
| `ts_error_db_message_leaked` | Raw database error message (table names, column names) returned to client | [3] | TS |
| `ts_error_internal_path` | File system path in error message — reveals server structure | [3] | TS |
| `ts_error_different_shape_by_env` | Error response shape differs between dev and prod — client assumes dev shape | [3] | TS |
| `ts_error_retry_after_not_set` | 429 rate limit response missing `Retry-After` header | [3] | TS |

---

## Group T — Edge Computing & Platform-Specific Security
*~250 bugs | Detection: fingerprint + taint*

**Why missed:** Platform-specific primitives (Cloudflare KV, Durable Objects, R2, Hono) have no Semgrep or CodeQL rules. These are entirely uncovered.

---

### Category 249 — Cloudflare Workers Specific

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_cf_kv_unauthenticated_read` | `env.KV.get(key)` where key is user-controlled and response returned — IDOR via KV | [4] | TS |
| `ts_cf_kv_namespace_collision` | Two features use same KV key prefix — one feature can overwrite the other's data | [3] | TS |
| `ts_cf_do_unauthenticated_access` | Durable Object `fetch()` called without validating the request origin | [4] | TS |
| `ts_cf_do_id_from_user` | DO ID derived from user-controlled input — user can address any DO | [4] | TS |
| `ts_cf_r2_public_bucket` | R2 bucket bound as public — any object readable without auth | [3] | TS |
| `ts_cf_r2_key_from_user_no_check` | R2 object key from user input — path traversal / unauthorized access | [4] | TS |
| `ts_cf_worker_secret_in_response` | `env.SECRET_KEY` value returned in response body | [3] | TS |
| `ts_cf_cron_trigger_no_auth` | Cron trigger handler doesn't verify the request came from CF scheduler | [3] | TS |
| `ts_cf_email_routing_no_verify` | CF Email Routing handler doesn't verify sender | [3] | TS |
| `ts_cf_queues_no_schema_validation` | CF Queue consumer processes message without schema check | [3] | TS |
| `ts_cf_ai_model_from_user` | AI binding model name from user request body | [3] | TS |
| `ts_cf_vectorize_no_namespace` | Vectorize index query without namespace filter — cross-user vector search | [3] | TS |

---

### Category 250 — Node.js Runtime Specific

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_node_process_exit_in_handler` | `process.exit()` called in request handler — kills entire server | [3] | TS |
| `ts_node_uncaught_exception` | `process.on('uncaughtException', () => {})` — swallows all errors | [3] | TS |
| `ts_node_child_process_detached` | `spawn(cmd, { detached: true })` — child process survives parent | [3] | TS |
| `ts_node_require_user_module` | `require(req.body.module)` — loads arbitrary module | [3] | TS |
| `ts_node_cluster_shared_memory` | Shared memory between cluster workers — race condition | [3] | TS |
| `ts_node_heap_snapshot_endpoint` | V8 heap snapshot endpoint accessible — dumps all in-memory data | [3] | TS |
| `ts_node_inspector_enabled` | `--inspect` flag enabled in production — remote debugger accessible | [3] | TS |
| `ts_node_timer_ref_leak` | `setInterval` return value not stored — can't be cleared | [3] | TS |

---

## Group U — Security Anti-Patterns in Popular Frameworks
*~300 bugs | Detection: fingerprint*

---

### Category 251 — Next.js Security Patterns

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_nextjs_middleware_bypass` | `middleware.ts` not covering all sensitive routes — bypass via uncovered path | [4] | TS |
| `ts_nextjs_server_action_no_csrf` | Server action callable without origin validation | [3] | TS |
| `ts_nextjs_params_not_validated` | `params.id` from URL used directly in DB query without validation | [4] | TS |
| `ts_nextjs_searchparams_not_validated` | `searchParams.query` used in DB query without validation | [3] | TS |
| `ts_nextjs_cookies_not_httponly` | `cookies().set()` without `httpOnly: true` | [3] | TS |
| `ts_nextjs_env_client_exposure` | `process.env.SECRET` used in client component — bundled into JS | [4] | TS |
| `ts_nextjs_revalidate_path_injection` | `revalidatePath(req.body.path)` — cache poison via revalidation | [3] | TS |
| `ts_nextjs_redirect_unvalidated` | `redirect(req.body.returnUrl)` — open redirect | [3] | TS |

---

### Category 252 — tRPC Security Patterns

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_trpc_procedure_no_auth` | `t.procedure` used instead of `protectedProcedure` for sensitive operation | [4] | TS |
| `ts_trpc_input_no_zod` | tRPC procedure accepts input without Zod schema | [3] | TS |
| `ts_trpc_context_user_not_checked` | `ctx.user` used without checking it is not `undefined` | [3] | TS |
| `ts_trpc_middleware_order` | Auth middleware applied after business logic | [3] | TS |
| `ts_trpc_subscription_no_auth` | WebSocket subscription procedure has no auth check | [3] | TS |

---

### Category 253 — Drizzle ORM Security Patterns

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_drizzle_sql_template_injection` | `sql\`SELECT * FROM ${table}\`` — table name from user | [3] | TS |
| `ts_drizzle_dynamic_where_clause` | Where clause built dynamically from user-controlled filter object | [3] | TS |
| `ts_drizzle_no_row_limit` | `.findMany()` without `.limit()` — unbounded query | [3] | TS |

---

## Group V — Time, Scheduling & Temporal Security
*~200 bugs | Detection: temporal + fingerprint*

---

### Category 254 — Time-Based Security Bugs

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_time_token_validation_clock_skew` | Token expiry check with no clock skew tolerance — fails across timezones | [3] | TS |
| `ts_time_otp_window_too_large` | TOTP valid for 10 minutes instead of 30 seconds | [3] | TS |
| `ts_time_scheduling_no_timezone` | Appointment scheduled without timezone — wrong time in different region | [3] | TS |
| `ts_time_cron_dst_failure` | Cron job runs twice or skips during daylight saving time transition | [3] | TS |
| `ts_time_date_comparison_string` | Dates compared as strings — lexicographic not chronological | [3] | TS |
| `ts_time_expiry_in_past_accepted` | Expiry date in the past accepted — expired items accessible | [3] | TS |
| `ts_time_created_at_from_user` | `createdAt` timestamp from request body — user sets creation time | [3] | TS |
| `ts_time_race_window_attack` | Short-lived token can be used concurrently — N parallel requests all succeed | [4] | TS |

---

## Group W — Emerging: Web3 & Blockchain Interface
*~200 bugs | Detection: fingerprint*

**Why missed:** No historical Semgrep/CodeQL rules. New attack class requiring corpus-based learning.

---

### Category 255 — Smart Contract Interface Security

| Bug Name | What it looks like | Mutations | Lang |
|---|---|---|---|
| `ts_web3_signature_not_verified` | Signed message from user not verified against expected signer address | [4] | TS |
| `ts_web3_replay_attack_no_nonce` | Signed transaction has no nonce — replay on different chain/session | [4] | TS |
| `ts_web3_chain_id_not_checked` | Signature valid but chain ID not verified — cross-chain replay | [3] | TS |
| `ts_web3_address_checksum_not_validated` | Ethereum address not checksum-validated — case variation bypass | [3] | TS |
| `ts_web3_abi_decode_unvalidated` | ABI-decoded data from contract used without field validation | [3] | TS |
| `ts_web3_private_key_in_code` | Wallet private key or mnemonic hardcoded in source | [3] | TS |
| `ts_web3_frontrun_no_commit_reveal` | On-chain action predictable — no commit-reveal scheme | [3] | TS |
| `ts_web3_rpc_url_from_user` | RPC endpoint URL from user request — SSRF via Web3 provider | [4] | TS |

---

## Final Count Summary

| Group | Categories | Bugs |
|---|---|---|
| A — Semantic Drift | 176–180 | 400 |
| B — Missing Behavior | 181–186 | 500 |
| C — Defensive Code That Doesn't Defend | 187–191 | 300 |
| D — Context-Dependent Safety | 192–195 | 400 |
| E — Protocol Implementation | 196–201 | 400 |
| F — Distributed System | 202–207 | 500 |
| G — Type System & Runtime | 208–211 | 350 |
| H — Configuration & Secrets | 212–215 | 300 |
| I — SaaS Platform | 216–219 | 400 |
| J — AI/LLM Pipeline | 220–224 | 350 |
| K — Privacy & Data Governance | 225–227 | 300 |
| L — Testing Anti-Patterns | 228–229 | 200 |
| M — Background Job Security | 230–232 | 250 |
| N — Internationalization & Encoding | 233–235 | 250 |
| O — Real-Time Collaboration | 236–237 | 200 |
| P — Cryptographic Protocol | 238–240 | 300 |
| Q — Numeric & Financial Precision | 241–242 | 250 |
| R — SDK & Library Misuse | 243–246 | 300 |
| S — Error Handling Anti-Patterns | 247–248 | 250 |
| T — Edge Computing & Platform | 249–250 | 250 |
| U — Framework Security Patterns | 251–253 | 300 |
| V — Time & Temporal Security | 254 | 200 |
| W — Web3 & Blockchain Interface | 255 | 200 |
| **New subtotal** | **80 new categories** | **~6,950** |
| **Existing taxonomy (doc 1)** | **175 categories** | **~4,050** |
| **Grand total** | **255 categories** | **~11,000** |

---

## Why Semgrep and CodeQL Cannot Catch These

| Group | Semgrep | CodeQL | Frensense |
|---|---|---|---|
| Semantic Drift | ❌ Analyzes functions in isolation | ❌ No divergence detection | ✅ Near-duplicate fingerprinting |
| Missing Behavior | ❌ Can only match what exists | ❌ Can only follow what exists | ✅ Temporal rules: A must follow B |
| Defensive Code That Fails | ❌ Guard present = safe | ❌ Guard present = clean path | ✅ CSA: guard always returns true |
| Context-Dependent | ❌ No context model | ⚠️ Type-level only | ✅ Flow state tags carry context |
| Protocol Sequences | ❌ No sequence model | ⚠️ Single-path taint only | ✅ Temporal engine: full sequence |
| Distributed System | ❌ Single file only | ❌ Single codebase only | ✅ Cross-file + temporal |
| Type System Abuse | ⚠️ Can match `any` literally | ✅ Type-aware | ✅ Taint through `any` |
| Domain/Business Logic | ❌ No domain knowledge | ❌ No domain knowledge | ✅ Learns from your codebase |
| Platform-Specific | ⚠️ Needs per-SDK rules | ⚠️ Needs framework models | ✅ Learns from examples |
| AI/LLM | ❌ No rules exist | ❌ No rules exist | ✅ First-mover corpus |
| Privacy/PII | ⚠️ Regex for known fields | ⚠️ Needs PII model | ✅ PII taint origin tracking |
| Numeric Precision | ❌ Cannot evaluate semantics | ❌ Cannot evaluate semantics | ✅ Learns float-in-financial pattern |
| Crypto Protocol Order | ❌ Can match single calls | ❌ Cannot enforce ordering | ✅ Temporal: encrypt-then-MAC order |
| Error Handling | ⚠️ Can match empty catch | ⚠️ Partial | ✅ CSA: catch grants access |
| Time/Scheduling | ❌ No temporal semantics | ❌ No scheduling model | ✅ Temporal rules |
| Web3 | ❌ No rules | ❌ No rules | ✅ Learns from examples |
