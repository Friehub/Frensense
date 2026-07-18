# Frensense — Engine TOML Registry Reference
## All Stable Configuration Files

These files encode **universal truths about programming** — things that don't change when your codebase changes.
Domain-specific knowledge lives in the corpus. Platform-specific idioms live in `framework_models/`.
Temporal ordering rules, sanitizer truth tables, and sink severity maps go here.

Rule of thumb: if you'd have to update this file more than once a year, it belongs in the corpus instead.

---

# FILE 1 — `temporal_rules.toml`
# Parses directly into `TemporalRuleToml` via `add_rules_from_toml()`
# Format: [[rule]] with sequence, behavior, severity, tags

```toml
# ─────────────────────────────────────────────────
# FINANCIAL ORDERING RULES
# ─────────────────────────────────────────────────

[[rule]]
sequence = ["fundWallet", "createLedgerEntry"]
behavior = "Every wallet credit must be followed by a ledger entry in the same function"
severity = "critical"
tags = ["financial", "ledger", "consistency"]

[[rule]]
sequence = ["debitWallet", "createLedgerEntry"]
behavior = "Every wallet debit must be followed by a ledger entry in the same function"
severity = "critical"
tags = ["financial", "ledger", "consistency"]

[[rule]]
sequence = ["chargeCard", "createPaymentRecord"]
behavior = "Every card charge must be followed by a payment record"
severity = "critical"
tags = ["financial", "payment", "idempotency"]

[[rule]]
sequence = ["issueRefund", "createRefundRecord"]
behavior = "Every refund must be followed by a refund record"
severity = "critical"
tags = ["financial", "refund", "consistency"]

[[rule]]
sequence = ["transferFunds", "createLedgerEntry"]
behavior = "Every fund transfer must produce a ledger entry"
severity = "critical"
tags = ["financial", "ledger"]

[[rule]]
sequence = ["awardPoints", "createPointsEntry"]
behavior = "Every points award must produce a points ledger entry"
severity = "high"
tags = ["financial", "loyalty"]

# ─────────────────────────────────────────────────
# STATE TRANSITION RULES
# ─────────────────────────────────────────────────

[[rule]]
sequence = ["updateOrderStatus", "publishEvent"]
behavior = "Every order status change must publish a domain event"
severity = "high"
tags = ["event-driven", "consistency", "order"]

[[rule]]
sequence = ["updateSubscriptionStatus", "publishEvent"]
behavior = "Every subscription state change must publish an event"
severity = "high"
tags = ["event-driven", "consistency", "subscription"]

[[rule]]
sequence = ["updatePaymentStatus", "publishEvent"]
behavior = "Every payment state change must publish an event"
severity = "high"
tags = ["event-driven", "payment"]

[[rule]]
sequence = ["cancelOrder", "releaseStock"]
behavior = "Every order cancellation must release reserved inventory"
severity = "high"
tags = ["inventory", "consistency", "order"]

[[rule]]
sequence = ["cancelOrder", "issueRefund"]
behavior = "Every cancellation of a paid order must trigger a refund"
severity = "critical"
tags = ["financial", "order", "refund"]

[[rule]]
sequence = ["approveReturn", "issueRefund"]
behavior = "Every approved return must trigger a refund"
severity = "critical"
tags = ["financial", "return", "refund"]

[[rule]]
sequence = ["approveReturn", "createLedgerEntry"]
behavior = "Every approved return must produce a ledger debit for the seller"
severity = "critical"
tags = ["financial", "return", "ledger"]

[[rule]]
sequence = ["fulfillOrder", "deductStock"]
behavior = "Every fulfilled order must deduct from stock"
severity = "high"
tags = ["inventory", "order"]

# ─────────────────────────────────────────────────
# AUTHORIZATION ORDERING RULES
# ─────────────────────────────────────────────────

[[rule]]
sequence = ["checkOwnership", "updateResource"]
behavior = "Ownership must be verified before any resource mutation"
severity = "critical"
tags = ["idor", "authorization"]

[[rule]]
sequence = ["checkOwnership", "deleteResource"]
behavior = "Ownership must be verified before resource deletion"
severity = "critical"
tags = ["idor", "authorization"]

[[rule]]
sequence = ["checkPermission", "performAdminAction"]
behavior = "Permission must be checked before any privileged action"
severity = "critical"
tags = ["authorization", "privilege"]

[[rule]]
sequence = ["verifySession", "accessSensitiveData"]
behavior = "Session must be verified before sensitive data access"
severity = "critical"
tags = ["authorization", "session"]

# ─────────────────────────────────────────────────
# BILLING / QUOTA ORDERING RULES
# ─────────────────────────────────────────────────

[[rule]]
sequence = ["checkCredits", "callLLM"]
behavior = "Credit balance must be checked before any LLM call"
severity = "critical"
tags = ["billing", "quota", "llm"]

[[rule]]
sequence = ["deductCredits", "callLLM"]
behavior = "Credits must be deducted before the LLM call executes"
severity = "critical"
tags = ["billing", "quota", "llm"]

[[rule]]
sequence = ["checkQuota", "spawnSandbox"]
behavior = "Quota must be verified before spawning a compute sandbox"
severity = "critical"
tags = ["billing", "quota", "compute"]

[[rule]]
sequence = ["checkPlan", "accessPremiumFeature"]
behavior = "Subscription plan must be verified before accessing premium features"
severity = "high"
tags = ["billing", "subscription", "feature"]

[[rule]]
sequence = ["verifyWebhookSignature", "processWebhookEvent"]
behavior = "Webhook signature must be verified before processing the event"
severity = "critical"
tags = ["webhook", "security", "authentication"]

# ─────────────────────────────────────────────────
# AUDIT / COMPLIANCE ORDERING RULES
# ─────────────────────────────────────────────────

[[rule]]
sequence = ["deleteRecord", "createAuditEntry"]
behavior = "Every data deletion must create an audit trail entry"
severity = "high"
tags = ["audit", "compliance", "gdpr"]

[[rule]]
sequence = ["performAdminAction", "createAuditEntry"]
behavior = "Every admin action must produce an audit log entry"
severity = "high"
tags = ["audit", "compliance", "admin"]

[[rule]]
sequence = ["exportUserData", "createAuditEntry"]
behavior = "Every data export must be recorded in the audit log"
severity = "high"
tags = ["audit", "compliance", "gdpr"]

[[rule]]
sequence = ["changePermission", "createAuditEntry"]
behavior = "Every permission change must be audited"
severity = "high"
tags = ["audit", "compliance"]

# ─────────────────────────────────────────────────
# CLEANUP / RESOURCE MANAGEMENT
# ─────────────────────────────────────────────────

[[rule]]
sequence = ["acquireLock", "releaseLock"]
behavior = "Every lock acquisition must be followed by a release"
severity = "error"
tags = ["resource", "lock", "deadlock"]

[[rule]]
sequence = ["openConnection", "closeConnection"]
behavior = "Every opened connection must be closed"
severity = "warning"
tags = ["resource", "connection", "leak"]

[[rule]]
sequence = ["openFile", "closeFile"]
behavior = "Every opened file handle must be closed"
severity = "warning"
tags = ["resource", "file", "leak"]

[[rule]]
sequence = ["beginTransaction", "commitOrRollback"]
behavior = "Every started transaction must be committed or rolled back"
severity = "error"
tags = ["resource", "transaction", "database"]

[[rule]]
sequence = ["spawnSandbox", "destroySandbox"]
behavior = "Every spawned sandbox must be destroyed when done"
severity = "high"
tags = ["resource", "compute", "cost"]

[[rule]]
sequence = ["registerWebhook", "deregisterWebhookOnFailure"]
behavior = "If post-registration steps fail, registered webhook must be removed"
severity = "medium"
tags = ["resource", "webhook", "cleanup"]

# ─────────────────────────────────────────────────
# NOTIFICATION ORDERING RULES
# ─────────────────────────────────────────────────

[[rule]]
sequence = ["processRefund", "notifyUser"]
behavior = "Every processed refund must notify the user"
severity = "medium"
tags = ["notification", "user-experience"]

[[rule]]
sequence = ["chargeCard", "sendReceipt"]
behavior = "Every successful charge must send a receipt"
severity = "medium"
tags = ["notification", "financial"]

[[rule]]
sequence = ["resetPassword", "invalidateOtherSessions"]
behavior = "Password reset must invalidate all other active sessions"
severity = "high"
tags = ["security", "session", "password"]

[[rule]]
sequence = ["changeEmail", "verifyNewEmail"]
behavior = "Email change must trigger verification of the new address"
severity = "high"
tags = ["security", "identity", "email"]
```

---

# FILE 2 — `semantic_filters.toml` (expanded additions)
# Append these to the existing file

```toml
# ─────────────────────────────────────────────────
# TAINT / INJECTION PATTERNS
# ─────────────────────────────────────────────────

[ts_sql_injection_template_literal]
contains_call_to = ["prepare", "query", "execute", "raw"]
contains_node_type = ["template_string", "template_literal"]
must_not_contain_call_to = [".bind", "parameterize", "escape"]

[ts_sql_injection_concat]
contains_call_to = ["prepare", "query", "execute"]
contains_node_type = ["binary_expression"]
must_not_contain_call_to = [".bind", "parameterize"]

[ts_sqli_prisma_query_raw_unsafe]
contains_call_to = ["queryRawUnsafe", "executeRawUnsafe"]

[ts_sqli_sequelize_computed_where_key]
contains_call_to = ["findOne", "findAll", "findAndCountAll", "update", "destroy"]
contains_node_type = ["computed_property_name", "index_member_expression"]

[ts_nosqli_mongo_computed_key]
contains_call_to = ["find", "findOne", "updateOne", "deleteOne"]
contains_node_type = ["computed_property_name"]

[ts_command_injection_template]
contains_call_to = ["exec", "execSync", "spawn", "spawnSync", "execFile"]
contains_node_type = ["template_string", "template_literal"]

[ts_ssrf_fetch_constructed]
contains_call_to = ["fetch", "axios", "got", "request"]
contains_node_type = ["template_string", "binary_expression"]
must_not_contain_call_to = ["allowlist", "validateUrl", "isAllowedUrl"]

[ts_path_traversal_readfile]
contains_call_to = ["readFile", "readFileSync", "createReadStream", "open"]
must_not_contain_call_to = ["path.basename", "assertSafePath", "sanitizePath"]

# ─────────────────────────────────────────────────
# AUTHENTICATION PATTERNS
# ─────────────────────────────────────────────────

[ts_jwt_decode_vs_verify]
contains_call_to = ["decode", "jwt.decode", "jwtDecode"]
must_not_contain_call_to = ["verify", "jwt.verify", "verifyToken"]

[ts_fail_open_auth_catch]
contains_call_to = ["verifyToken", "verify", "authenticate", "resolveAuth"]
contains_node_type = ["catch_clause", "try_statement"]
must_not_contain_call_to = ["throw", "return null", "return 401", "return 403"]

[ts_auth_unauthenticated_db_write]
contains_call_to = ["prepare", "query", "insert", "update", "create", "upsert"]
must_not_contain_call_to = ["verifyToken", "resolveAuth", "requireAuth", "session", "getSession"]
must_not_match_function_name = ["register", "signup", "createAccount", "resetPassword", "forgotPassword", "verify", "confirm", "healthCheck", "handleWebhook"]

[ts_missing_2fa_guard]
function_name_regex = "withdraw|transfer|deleteAccount|changePassword|exportData"
must_not_contain_call_to = ["verify2fa", "checkMfa", "require2fa", "totp"]

[ts_session_not_invalidated_logout]
function_name_regex = "logout|signout|revoke"
must_not_contain_call_to = ["delete", "destroy", "invalidate", "revoke", "remove"]

# ─────────────────────────────────────────────────
# IDOR / AUTHORIZATION PATTERNS
# ─────────────────────────────────────────────────

[ts_idor_url_resource_id]
contains_call_to = ["prepare", "findUnique", "findById", "getById", "update", "delete"]
must_not_contain_call_to = ["checkOwnership", "verifyOwner", "assertOwner", "isOwner"]
must_not_match_function_name = ["admin", "system", "internal"]

[ts_idor_child_resource]
contains_call_to = ["findUnique", "findFirst", "findById"]
must_not_contain_call_to = ["checkOwnership", "verifyParent", "assertAccess"]

[ts_missing_payment_gate]
contains_call_to = ["callLLM", "spawnSandbox", "createSandbox", "runCode", "generateImage"]
must_not_contain_call_to = ["checkCredits", "deductCredits", "checkPlan", "checkQuota", "verifySubscription"]

[ts_quota_fail_open]
contains_call_to = ["checkQuota", "checkCredits", "getRemainingTokens"]
contains_node_type = ["catch_clause"]
must_not_contain_call_to = ["throw", "return.*false", "reject"]

# ─────────────────────────────────────────────────
# RACE CONDITION PATTERNS
# ─────────────────────────────────────────────────

[ts_race_condition_read_check_write]
contains_call_to = ["findFirst", "findUnique", "get", "select"]
must_not_contain_call_to = ["transaction", "$transaction", "BEGIN", "FOR UPDATE", "updateMany"]

[ts_idempotency_check_outside_txn]
contains_call_to = ["findFirst", "findUnique"]
must_not_contain_call_to = ["transaction", "$transaction", "updateMany"]
function_name_regex = "webhook|event|payment|charge|hook"

[ts_credit_double_spend]
contains_call_to = ["deductCredits", "decrementBalance", "updateBalance"]
must_not_contain_call_to = ["transaction", "$transaction", "updateMany", "decrement"]

# ─────────────────────────────────────────────────
# BUSINESS LOGIC PATTERNS
# ─────────────────────────────────────────────────

[ts_state_machine_direct_update]
contains_call_to = ["update", "updateOne", "save", "patch"]
contains_node_type = ["string", "property_assignment"]
must_not_contain_call_to = ["transition", "updateStatus", "changeState", "stateMachine"]
must_not_match_function_name = ["updateStatus", "changeState", "transition"]

[ts_coupon_scope_bypass]
contains_call_to = ["applyCoupon", "applyDiscount", "validateCoupon"]
must_not_contain_call_to = ["filter", "sellerId", "scope", "applicableItems"]

[ts_missing_ledger_after_wallet_fund]
contains_call_to = ["fundWallet", "creditWallet", "addToBalance", "incrementBalance"]
must_not_contain_call_to = ["createLedgerEntry", "insertLedger", "ledger.create", "recordTransaction"]

[ts_missing_event_after_state_change]
contains_call_to = ["updateStatus", "changeState", "transition"]
must_not_contain_call_to = ["publishEvent", "emit", "dispatch", "publish", "send"]

[ts_missing_stock_release_after_cancel]
function_name_regex = "cancel|cancelOrder|refundOrder"
must_not_contain_call_to = ["releaseStock", "restoreInventory", "returnToStock", "incrementStock"]

# ─────────────────────────────────────────────────
# INFORMATION DISCLOSURE PATTERNS
# ─────────────────────────────────────────────────

[ts_debug_stack_leak]
contains_call_to = ["json", "send", "respond"]
contains_node_type = ["member_expression"]
must_not_contain_call_to = ["sanitizeError", "formatError", "publicMessage"]

[ts_unfiltered_json_response]
contains_call_to = ["findUnique", "findFirst", "findById", "findOne"]
must_not_contain_call_to = ["select", "omit", "pick", "exclude", "sanitize", "toPublic"]

[ts_console_log_credential]
contains_call_to = ["console.log", "console.info", "console.debug", "logger.info", "logger.debug"]
must_not_match_file_path_pattern = ["*.test.ts", "*.spec.ts", "__tests__/"]

# ─────────────────────────────────────────────────
# CRYPTO PATTERNS
# ─────────────────────────────────────────────────

[ts_insecure_random]
contains_call_to = ["Math.random", "Date.now"]
must_not_match_function_name = ["generatePlaceholder", "createLoadingId", "animationFrame"]

[ts_weak_id_entropy]
contains_call_to = ["Math.random", "Date.now", "Date.now().toString"]
function_name_regex = "generate.*Token|create.*Token|generate.*Id|create.*Secret|generate.*Key|generate.*Nonce|generate.*Code"

[ts_missing_webhook_signature]
function_name_regex = "handleWebhook|webhook|onWebhook|processWebhook"
must_not_contain_call_to = ["createHmac", "timingSafeEqual", "verifySignature", "constructEvent", "verify"]

[ts_tls_verify_disabled]
contains_call_to = ["https", "tls", "createServer", "request"]
must_not_contain_call_to = ["rejectUnauthorized: true"]

# ─────────────────────────────────────────────────
# CLOUDFLARE WORKERS SPECIFIC
# ─────────────────────────────────────────────────

[ts_cf_kv_unauthenticated_write]
contains_call_to = [".put", "KV.put", "CACHE.put"]
must_not_contain_call_to = ["verifyToken", "resolveAuth", "requireAuth", "session"]
must_not_match_function_name = ["healthCheck", "handleCors", "preflight"]

[ts_cf_do_unauthenticated_access]
contains_call_to = ["idFromName", "idFromString", "get(doId)"]
must_not_contain_call_to = ["verifyToken", "resolveAuth", "requireAuth"]

[ts_module_level_mutable_state]
must_not_contain_node_type = ["function_declaration", "arrow_function", "class_declaration"]
contains_node_type = ["lexical_declaration", "variable_declaration"]

[ts_workers_missing_wait_until]
contains_call_to = ["fetch", "publish", "sendEmail", "log", "analytics"]
must_not_contain_call_to = ["ctx.waitUntil", "context.waitUntil"]
function_name_regex = "fetch|handle|handler"

# ─────────────────────────────────────────────────
# AI/LLM PATTERNS
# ─────────────────────────────────────────────────

[ts_prompt_injection_user_input]
contains_call_to = ["createChatCompletion", "complete", "generateText", "invoke", "run"]
contains_node_type = ["template_string", "binary_expression"]
must_not_contain_call_to = ["sanitizeInput", "stripPromptInjection", "validatePrompt"]

[ts_llm_output_eval]
contains_call_to = ["eval", "Function", "vm.runInNewContext"]
must_not_contain_call_to = ["parseCodeBlock", "validateSyntax", "lintCode"]

[ts_llm_no_token_limit]
contains_call_to = ["createChatCompletion", "complete", "invoke", "generateText"]
must_not_contain_call_to = ["max_tokens", "maxTokens", "token_limit"]

[ts_llm_model_from_user]
contains_call_to = ["createChatCompletion", "complete", "invoke"]
contains_node_type = ["member_expression", "subscript_expression"]
must_not_contain_call_to = ["ALLOWED_MODELS", "validateModel", "allowedModels.includes"]

# ─────────────────────────────────────────────────
# SUPPRESSION CONTEXTS
# All patterns: suppress in these file/function contexts
# ─────────────────────────────────────────────────

[_global_suppression]
must_not_match_file_path_pattern = [
    "*.test.ts",
    "*.spec.ts",
    "*.test.js",
    "*.spec.js",
    "__tests__/",
    "test/",
    "tests/",
    "fixtures/",
    "mocks/",
    "__mocks__/",
    "*.mock.ts",
    "seed.ts",
    "seed.js",
    "migrations/"
]
```

---

# FILE 3 — `sinks.toml`
# Qualified sink name → category + severity
# Engine reads this to populate CorpusSourceSinkRegistry with pre-seeded entries
# bypassing MIN_OCCURRENCES. Format requires new loader: `load_static_sinks()`

```toml
# ALWAYS_REGISTER = true means these bypass MIN_OCCURRENCES check
# These are universally dangerous regardless of context

# ─────────────────────────────────────────────────
# CODE EXECUTION — severity: critical
# ─────────────────────────────────────────────────

[[sink]]
name = "eval"
qualified = "eval"
category = "CodeExecution"
severity = "critical"
always_register = true
languages = ["typescript", "javascript"]

[[sink]]
name = "Function"
qualified = "Function"
category = "CodeExecution"
severity = "critical"
always_register = true
languages = ["typescript", "javascript"]

[[sink]]
name = "runInNewContext"
qualified = "vm.runInNewContext"
category = "CodeExecution"
severity = "critical"
always_register = true
languages = ["typescript", "javascript"]

[[sink]]
name = "runInThisContext"
qualified = "vm.runInThisContext"
category = "CodeExecution"
severity = "critical"
always_register = true
languages = ["typescript", "javascript"]

[[sink]]
name = "runInContext"
qualified = "vm.Script.runInContext"
category = "CodeExecution"
severity = "critical"
always_register = true
languages = ["typescript", "javascript"]

# ─────────────────────────────────────────────────
# COMMAND INJECTION — severity: critical
# ─────────────────────────────────────────────────

[[sink]]
name = "exec"
qualified = "exec"
category = "CommandInjection"
severity = "critical"
always_register = true
languages = ["typescript", "javascript"]

[[sink]]
name = "execSync"
qualified = "execSync"
category = "CommandInjection"
severity = "critical"
always_register = true
languages = ["typescript", "javascript"]

[[sink]]
name = "spawn"
qualified = "spawn"
category = "CommandInjection"
severity = "critical"
always_register = true
languages = ["typescript", "javascript"]

[[sink]]
name = "spawnSync"
qualified = "spawnSync"
category = "CommandInjection"
severity = "critical"
always_register = true
languages = ["typescript", "javascript"]

[[sink]]
name = "execFile"
qualified = "execFile"
category = "CommandInjection"
severity = "critical"
always_register = true
languages = ["typescript", "javascript"]

[[sink]]
name = "execFileSync"
qualified = "execFileSync"
category = "CommandInjection"
severity = "critical"
always_register = true
languages = ["typescript", "javascript"]

# Rust command injection
[[sink]]
name = "Command_new"
qualified = "Command.new"
category = "CommandInjection"
severity = "critical"
always_register = true
languages = ["rust"]

[[sink]]
name = "Command_arg"
qualified = "Command.arg"
category = "CommandInjection"
severity = "high"
always_register = true
languages = ["rust"]

# ─────────────────────────────────────────────────
# SQL INJECTION — severity: critical
# ─────────────────────────────────────────────────

[[sink]]
name = "query"
qualified = "db.query"
category = "SqlInjection"
severity = "critical"
always_register = true
languages = ["typescript", "javascript"]

[[sink]]
name = "queryRawUnsafe"
qualified = "prisma.queryRawUnsafe"
category = "SqlInjection"
severity = "critical"
always_register = true
languages = ["typescript"]

[[sink]]
name = "executeRawUnsafe"
qualified = "prisma.executeRawUnsafe"
category = "SqlInjection"
severity = "critical"
always_register = true
languages = ["typescript"]

[[sink]]
name = "raw"
qualified = "knex.raw"
category = "SqlInjection"
severity = "critical"
always_register = true
languages = ["typescript", "javascript"]

[[sink]]
name = "prepare"
qualified = "DB.prepare"
category = "SqlInjection"
severity = "critical"
condition = "argument_contains_template_expression"
always_register = true
languages = ["typescript"]

# Rust SQL sinks
[[sink]]
name = "query_raw"
qualified = "sqlx.query"
category = "SqlInjection"
severity = "critical"
condition = "argument_is_not_string_literal"
always_register = true
languages = ["rust"]

[[sink]]
name = "execute_raw"
qualified = "diesel.execute"
category = "SqlInjection"
severity = "critical"
always_register = true
languages = ["rust"]

# ─────────────────────────────────────────────────
# PATH TRAVERSAL — severity: high
# ─────────────────────────────────────────────────

[[sink]]
name = "readFile"
qualified = "fs.readFile"
category = "PathTraversal"
severity = "high"
always_register = true
languages = ["typescript", "javascript"]

[[sink]]
name = "readFileSync"
qualified = "fs.readFileSync"
category = "PathTraversal"
severity = "high"
always_register = true
languages = ["typescript", "javascript"]

[[sink]]
name = "writeFile"
qualified = "fs.writeFile"
category = "PathTraversal"
severity = "high"
always_register = true
languages = ["typescript", "javascript"]

[[sink]]
name = "writeFileSync"
qualified = "fs.writeFileSync"
category = "PathTraversal"
severity = "high"
always_register = true
languages = ["typescript", "javascript"]

[[sink]]
name = "unlink"
qualified = "fs.unlink"
category = "PathTraversal"
severity = "high"
always_register = true
languages = ["typescript", "javascript"]

[[sink]]
name = "createReadStream"
qualified = "fs.createReadStream"
category = "PathTraversal"
severity = "high"
always_register = true
languages = ["typescript", "javascript"]

# Rust path traversal
[[sink]]
name = "read_to_string"
qualified = "fs.read_to_string"
category = "PathTraversal"
severity = "high"
always_register = true
languages = ["rust"]

[[sink]]
name = "File_open"
qualified = "File.open"
category = "PathTraversal"
severity = "high"
always_register = true
languages = ["rust"]

# ─────────────────────────────────────────────────
# SSRF — severity: high
# ─────────────────────────────────────────────────

[[sink]]
name = "fetch"
qualified = "fetch"
category = "Ssrf"
severity = "high"
always_register = true
languages = ["typescript", "javascript"]

[[sink]]
name = "get"
qualified = "axios.get"
category = "Ssrf"
severity = "high"
always_register = true
languages = ["typescript", "javascript"]

[[sink]]
name = "post"
qualified = "axios.post"
category = "Ssrf"
severity = "high"
always_register = true
languages = ["typescript", "javascript"]

# Rust SSRF
[[sink]]
name = "reqwest_get"
qualified = "reqwest.get"
category = "Ssrf"
severity = "high"
always_register = true
languages = ["rust"]

[[sink]]
name = "reqwest_post"
qualified = "reqwest.post"
category = "Ssrf"
severity = "high"
always_register = true
languages = ["rust"]

# ─────────────────────────────────────────────────
# XSS — severity: high
# ─────────────────────────────────────────────────

[[sink]]
name = "innerHTML"
qualified = "innerHTML"
category = "Xss"
severity = "high"
always_register = true
languages = ["typescript", "javascript"]

[[sink]]
name = "outerHTML"
qualified = "outerHTML"
category = "Xss"
severity = "high"
always_register = true
languages = ["typescript", "javascript"]

[[sink]]
name = "document.write"
qualified = "document.write"
category = "Xss"
severity = "critical"
always_register = true
languages = ["typescript", "javascript"]

[[sink]]
name = "dangerouslySetInnerHTML"
qualified = "dangerouslySetInnerHTML"
category = "Xss"
severity = "high"
always_register = true
languages = ["typescript"]

# ─────────────────────────────────────────────────
# STORAGE WRITE (UNAUTH) — severity: high
# ─────────────────────────────────────────────────

[[sink]]
name = "KV_put"
qualified = "KV.put"
category = "StorageWrite"
severity = "high"
always_register = true
languages = ["typescript"]

[[sink]]
name = "KV_delete"
qualified = "KV.delete"
category = "StorageWrite"
severity = "high"
always_register = true
languages = ["typescript"]

[[sink]]
name = "R2_put"
qualified = "R2.put"
category = "StorageWrite"
severity = "high"
always_register = true
languages = ["typescript"]

[[sink]]
name = "DO_storage_put"
qualified = "storage.put"
category = "StorageWrite"
severity = "high"
always_register = true
languages = ["typescript"]

# ─────────────────────────────────────────────────
# OPEN REDIRECT — severity: high
# ─────────────────────────────────────────────────

[[sink]]
name = "redirect"
qualified = "redirect"
category = "OpenRedirect"
severity = "high"
always_register = true
languages = ["typescript", "javascript"]

[[sink]]
name = "c_redirect"
qualified = "c.redirect"
category = "OpenRedirect"
severity = "high"
always_register = true
languages = ["typescript"]

[[sink]]
name = "res_redirect"
qualified = "res.redirect"
category = "OpenRedirect"
severity = "high"
always_register = true
languages = ["typescript", "javascript"]

# ─────────────────────────────────────────────────
# CREDENTIAL LEAK — severity: high
# ─────────────────────────────────────────────────

[[sink]]
name = "console_log"
qualified = "console.log"
category = "CredentialLeak"
severity = "medium"
origin_filter = "Environment"
always_register = true
languages = ["typescript", "javascript"]

[[sink]]
name = "logger_info"
qualified = "logger.info"
category = "CredentialLeak"
severity = "medium"
origin_filter = "Environment"
always_register = true
languages = ["typescript", "javascript"]

# ─────────────────────────────────────────────────
# UNSAFE RUST — severity: critical
# ─────────────────────────────────────────────────

[[sink]]
name = "transmute"
qualified = "mem.transmute"
category = "UnsafeMemory"
severity = "critical"
always_register = true
languages = ["rust"]

[[sink]]
name = "from_utf8_unchecked"
qualified = "str.from_utf8_unchecked"
category = "UnsafeMemory"
severity = "critical"
always_register = true
languages = ["rust"]

[[sink]]
name = "from_raw_parts"
qualified = "slice.from_raw_parts"
category = "UnsafeMemory"
severity = "critical"
always_register = true
languages = ["rust"]

[[sink]]
name = "from_raw_parts_mut"
qualified = "slice.from_raw_parts_mut"
category = "UnsafeMemory"
severity = "critical"
always_register = true
languages = ["rust"]
```

---

# FILE 4 — `sanitizers.toml`
# Functions that clear taint. Engine reads this for the SanitizerRegistry.
# full = taint clears unconditionally
# context = taint clears only for the specified sink category

```toml
# ─────────────────────────────────────────────────
# FULL SANITIZERS — clear all taint on return value
# ─────────────────────────────────────────────────

[[sanitizer]]
name = "parseInt"
type = "full"
languages = ["typescript", "javascript"]
note = "Numeric coercion — injection impossible on a number"

[[sanitizer]]
name = "parseFloat"
type = "full"
languages = ["typescript", "javascript"]

[[sanitizer]]
name = "Number"
type = "full"
languages = ["typescript", "javascript"]

[[sanitizer]]
name = "BigInt"
type = "full"
languages = ["typescript", "javascript"]

[[sanitizer]]
name = "Math.round"
type = "full"
languages = ["typescript", "javascript"]

[[sanitizer]]
name = "Math.floor"
type = "full"
languages = ["typescript", "javascript"]

[[sanitizer]]
name = "Math.ceil"
type = "full"
languages = ["typescript", "javascript"]

[[sanitizer]]
name = "Math.abs"
type = "full"
languages = ["typescript", "javascript"]

[[sanitizer]]
name = "Boolean"
type = "full"
languages = ["typescript", "javascript"]
note = "Boolean coercion — injection impossible on a boolean"

[[sanitizer]]
name = "crypto.randomUUID"
type = "full"
languages = ["typescript", "javascript"]
note = "Replaces user input with cryptographically random value"

[[sanitizer]]
name = "uuidv4"
type = "full"
languages = ["typescript", "javascript"]

[[sanitizer]]
name = "nanoid"
type = "full"
languages = ["typescript", "javascript"]

[[sanitizer]]
name = "DOMPurify.sanitize"
type = "full"
languages = ["typescript", "javascript"]

[[sanitizer]]
name = "sanitizeHtml"
type = "full"
languages = ["typescript", "javascript"]

[[sanitizer]]
name = "he.escape"
type = "full"
languages = ["typescript", "javascript"]

[[sanitizer]]
name = "escapeHtml"
type = "full"
languages = ["typescript", "javascript"]

[[sanitizer]]
name = "validator.escape"
type = "full"
languages = ["typescript", "javascript"]

[[sanitizer]]
name = "xss"
type = "full"
languages = ["typescript", "javascript"]

# Rust full sanitizers
[[sanitizer]]
name = "parse_u64"
type = "full"
pattern = ".parse::<u64>()"
languages = ["rust"]

[[sanitizer]]
name = "parse_i64"
type = "full"
pattern = ".parse::<i64>()"
languages = ["rust"]

[[sanitizer]]
name = "parse_usize"
type = "full"
pattern = ".parse::<usize>()"
languages = ["rust"]

[[sanitizer]]
name = "parse_f64"
type = "full"
pattern = ".parse::<f64>()"
languages = ["rust"]

[[sanitizer]]
name = "Uuid_new_v4"
type = "full"
pattern = "Uuid::new_v4()"
languages = ["rust"]

[[sanitizer]]
name = "percent_encode"
type = "full"
languages = ["rust"]

# ─────────────────────────────────────────────────
# CONTEXT SANITIZERS — clear taint for specific sink category only
# ─────────────────────────────────────────────────

[[sanitizer]]
name = "bind"
type = "context"
clears_for = ["SqlInjection"]
languages = ["typescript", "javascript", "rust"]
note = "Parameterised binding clears SQL injection. Taint can still reach other sinks."

[[sanitizer]]
name = "encodeURIComponent"
type = "context"
clears_for = ["Ssrf", "OpenRedirect"]
languages = ["typescript", "javascript"]
note = "URL encoding clears SSRF/redirect injection. Does NOT clear SQL or command injection."

[[sanitizer]]
name = "encodeURI"
type = "context"
clears_for = ["Ssrf", "OpenRedirect"]
languages = ["typescript", "javascript"]

[[sanitizer]]
name = "new URL"
type = "context"
clears_for = ["Ssrf", "OpenRedirect"]
languages = ["typescript", "javascript"]
note = "URL constructor validates and structures URL. Throws on invalid input."

[[sanitizer]]
name = "path.basename"
type = "context"
clears_for = ["PathTraversal"]
languages = ["typescript", "javascript"]
note = "Strips directory component. Safe for filename-only use. Does NOT sanitize for full path use."

[[sanitizer]]
name = "assertSafePath"
type = "context"
clears_for = ["PathTraversal"]
languages = ["typescript", "javascript"]

[[sanitizer]]
name = "validateUrl"
type = "context"
clears_for = ["Ssrf", "OpenRedirect"]
languages = ["typescript", "javascript"]

[[sanitizer]]
name = "sqlx_query_macro"
type = "context"
clears_for = ["SqlInjection"]
pattern = "sqlx::query!"
languages = ["rust"]
note = "Macro enforces string literal — cannot be injected"

# ─────────────────────────────────────────────────
# NOT SANITIZERS — common mistakes, explicitly listed to prevent false negatives
# These must NOT be registered as sanitizers
# ─────────────────────────────────────────────────

[[not_sanitizer]]
name = "path.normalize"
reason = "Normalizes path separators but does NOT prevent directory traversal"

[[not_sanitizer]]
name = "JSON.stringify"
reason = "Serializes to JSON string but does not sanitize injection characters"

[[not_sanitizer]]
name = "toString"
reason = "Type coercion only — preserves injection payload"

[[not_sanitizer]]
name = "trim"
reason = "Removes whitespace only"

[[not_sanitizer]]
name = "toLowerCase"
reason = "Case conversion only"

[[not_sanitizer]]
name = "toUpperCase"
reason = "Case conversion only"

[[not_sanitizer]]
name = "String"
reason = "String coercion — same as toString()"

[[not_sanitizer]]
name = "decodeURIComponent"
reason = "Decodes URL encoding — may INCREASE injection risk by normalizing encoded attacks"

[[not_sanitizer]]
name = "decodeURI"
reason = "Decodes URI — may reveal encoded attack payload"

[[not_sanitizer]]
name = "atob"
reason = "Decodes base64 — reveals encoded payload and corrupts non-ASCII"

[[not_sanitizer]]
name = "unescape"
reason = "Deprecated URL decoder — reveals encoded payload"
```

---

# FILE 5 — `propagators.toml`
# Functions where taint on argument 0 (or N) flows through to the return value
# Engine uses this in follow_taint() to seed taint on call return bindings

```toml
# ─────────────────────────────────────────────────
# IDENTITY PROPAGATORS — return = arg[0]
# ─────────────────────────────────────────────────

[[propagator]]
name = "JSON.parse"
tainted_arg = 0
taints_return = true
languages = ["typescript", "javascript"]
note = "Parsed JSON inherits taint of the source string"

[[propagator]]
name = "Buffer.from"
tainted_arg = 0
taints_return = true
languages = ["typescript", "javascript"]

[[propagator]]
name = "atob"
tainted_arg = 0
taints_return = true
languages = ["typescript", "javascript"]

[[propagator]]
name = "btoa"
tainted_arg = 0
taints_return = true
languages = ["typescript", "javascript"]

[[propagator]]
name = "decodeURIComponent"
tainted_arg = 0
taints_return = true
languages = ["typescript", "javascript"]

[[propagator]]
name = "decodeURI"
tainted_arg = 0
taints_return = true
languages = ["typescript", "javascript"]

[[propagator]]
name = "JSON.stringify"
tainted_arg = 0
taints_return = true
languages = ["typescript", "javascript"]

[[propagator]]
name = "String"
tainted_arg = 0
taints_return = true
languages = ["typescript", "javascript"]

[[propagator]]
name = "Object.assign"
tainted_arg = 1
taints_return = true
languages = ["typescript", "javascript"]
note = "Source object (arg 1+) taints the target"

[[propagator]]
name = "Object.create"
tainted_arg = 0
taints_return = true
languages = ["typescript", "javascript"]

[[propagator]]
name = "structuredClone"
tainted_arg = 0
taints_return = true
languages = ["typescript", "javascript"]

# ─────────────────────────────────────────────────
# ARRAY METHOD PROPAGATORS
# ─────────────────────────────────────────────────

[[propagator]]
name = "Array.map"
tainted_receiver = true
taints_return = true
languages = ["typescript", "javascript"]
note = "map() of tainted array returns tainted array"

[[propagator]]
name = "Array.filter"
tainted_receiver = true
taints_return = true
languages = ["typescript", "javascript"]

[[propagator]]
name = "Array.reduce"
tainted_receiver = true
taints_return = true
languages = ["typescript", "javascript"]

[[propagator]]
name = "Array.find"
tainted_receiver = true
taints_return = true
languages = ["typescript", "javascript"]

[[propagator]]
name = "Array.flatMap"
tainted_receiver = true
taints_return = true
languages = ["typescript", "javascript"]

[[propagator]]
name = "Array.from"
tainted_arg = 0
taints_return = true
languages = ["typescript", "javascript"]

[[propagator]]
name = "Array.join"
tainted_receiver = true
taints_return = true
languages = ["typescript", "javascript"]

[[propagator]]
name = "Array.slice"
tainted_receiver = true
taints_return = true
languages = ["typescript", "javascript"]

[[propagator]]
name = "Array.concat"
tainted_receiver = true
taints_return = true
languages = ["typescript", "javascript"]

[[propagator]]
name = "Array.flat"
tainted_receiver = true
taints_return = true
languages = ["typescript", "javascript"]

# ─────────────────────────────────────────────────
# STRING METHOD PROPAGATORS
# ─────────────────────────────────────────────────

[[propagator]]
name = "String.replace"
tainted_receiver = true
taints_return = true
languages = ["typescript", "javascript"]
note = ".replace() may not sanitize — only removes specific pattern"

[[propagator]]
name = "String.replaceAll"
tainted_receiver = true
taints_return = true
languages = ["typescript", "javascript"]

[[propagator]]
name = "String.slice"
tainted_receiver = true
taints_return = true
languages = ["typescript", "javascript"]

[[propagator]]
name = "String.substring"
tainted_receiver = true
taints_return = true
languages = ["typescript", "javascript"]

[[propagator]]
name = "String.split"
tainted_receiver = true
taints_return = true
languages = ["typescript", "javascript"]

[[propagator]]
name = "String.trim"
tainted_receiver = true
taints_return = true
languages = ["typescript", "javascript"]

[[propagator]]
name = "String.toLowerCase"
tainted_receiver = true
taints_return = true
languages = ["typescript", "javascript"]

[[propagator]]
name = "String.toUpperCase"
tainted_receiver = true
taints_return = true
languages = ["typescript", "javascript"]

[[propagator]]
name = "String.concat"
tainted_receiver = true
taints_return = true
languages = ["typescript", "javascript"]

[[propagator]]
name = "String.padStart"
tainted_receiver = true
taints_return = true
languages = ["typescript", "javascript"]

[[propagator]]
name = "String.padEnd"
tainted_receiver = true
taints_return = true
languages = ["typescript", "javascript"]

# ─────────────────────────────────────────────────
# OBJECT PROPAGATORS
# ─────────────────────────────────────────────────

[[propagator]]
name = "Object.keys"
tainted_receiver = true
taints_return = true
languages = ["typescript", "javascript"]
note = "Object keys from user input may be tainted (prototype pollution vector)"

[[propagator]]
name = "Object.values"
tainted_receiver = true
taints_return = true
languages = ["typescript", "javascript"]

[[propagator]]
name = "Object.entries"
tainted_receiver = true
taints_return = true
languages = ["typescript", "javascript"]

[[propagator]]
name = "Object.fromEntries"
tainted_arg = 0
taints_return = true
languages = ["typescript", "javascript"]

# ─────────────────────────────────────────────────
# PROMISE PROPAGATORS
# ─────────────────────────────────────────────────

[[propagator]]
name = "Promise.resolve"
tainted_arg = 0
taints_return = true
languages = ["typescript", "javascript"]

[[propagator]]
name = "Promise.all"
tainted_arg = 0
taints_return = true
languages = ["typescript", "javascript"]
note = "If any promise in the array is tainted, the resolved array is tainted"

[[propagator]]
name = "await"
tainted_receiver = true
taints_return = true
languages = ["typescript", "javascript"]
note = "Awaiting a tainted promise propagates taint to the resolved value"

# ─────────────────────────────────────────────────
# RUST PROPAGATORS
# ─────────────────────────────────────────────────

[[propagator]]
name = "to_string"
tainted_receiver = true
taints_return = true
languages = ["rust"]

[[propagator]]
name = "clone"
tainted_receiver = true
taints_return = true
languages = ["rust"]

[[propagator]]
name = "into"
tainted_receiver = true
taints_return = true
languages = ["rust"]

[[propagator]]
name = "as_str"
tainted_receiver = true
taints_return = true
languages = ["rust"]

[[propagator]]
name = "as_bytes"
tainted_receiver = true
taints_return = true
languages = ["rust"]

[[propagator]]
name = "from_utf8"
tainted_arg = 0
taints_return = true
languages = ["rust"]

[[propagator]]
name = "serde_json_from_str"
tainted_arg = 0
taints_return = true
pattern = "serde_json::from_str"
languages = ["rust"]

[[propagator]]
name = "serde_json_from_value"
tainted_arg = 0
taints_return = true
pattern = "serde_json::from_value"
languages = ["rust"]

[[propagator]]
name = "format_macro"
tainted_arg = "any"
taints_return = true
pattern = "format!"
languages = ["rust"]
note = "format!() with any tainted argument produces tainted string"
```

---

# FILE 6 — `sources.toml`
# Known source types and source methods per language
# Supplements corpus-learned sources

```toml
# ─────────────────────────────────────────────────
# PARAMETER TYPES — UserInput origin
# Engine already learns these from corpus but these are always-register
# ─────────────────────────────────────────────────

[[source]]
origin = "UserInput"
kind = "parameter_type"
type_name = "Request"
languages = ["typescript", "javascript"]

[[source]]
origin = "UserInput"
kind = "parameter_type"
type_name = "IncomingMessage"
languages = ["typescript", "javascript"]

[[source]]
origin = "UserInput"
kind = "parameter_type"
type_name = "NextRequest"
languages = ["typescript", "javascript"]

[[source]]
origin = "UserInput"
kind = "parameter_type"
type_name = "Context"
languages = ["typescript"]

[[source]]
origin = "UserInput"
kind = "parameter_type"
type_name = "HonoContext"
languages = ["typescript"]

[[source]]
origin = "UserInput"
kind = "parameter_type"
type_name = "Env"
languages = ["typescript"]

# Rust sources
[[source]]
origin = "UserInput"
kind = "parameter_type"
type_name = "HttpRequest"
languages = ["rust"]

[[source]]
origin = "UserInput"
kind = "parameter_type"
type_name = "Request<Body>"
languages = ["rust"]

[[source]]
origin = "UserInput"
kind = "parameter_type"
type_name = "Path<T>"
languages = ["rust"]

[[source]]
origin = "UserInput"
kind = "parameter_type"
type_name = "Query<T>"
languages = ["rust"]

[[source]]
origin = "UserInput"
kind = "parameter_type"
type_name = "Json<T>"
languages = ["rust"]

[[source]]
origin = "UserInput"
kind = "parameter_type"
type_name = "Form<T>"
languages = ["rust"]

# ─────────────────────────────────────────────────
# METHOD CALL SOURCES — return value is tainted
# ─────────────────────────────────────────────────

[[source]]
origin = "UserInput"
kind = "method_return"
receiver_type = "Request"
method = "json"
languages = ["typescript", "javascript"]

[[source]]
origin = "UserInput"
kind = "method_return"
receiver_type = "Request"
method = "text"
languages = ["typescript", "javascript"]

[[source]]
origin = "UserInput"
kind = "method_return"
receiver_type = "Request"
method = "formData"
languages = ["typescript", "javascript"]

[[source]]
origin = "UserInput"
kind = "method_return"
receiver_type = "Request"
method = "arrayBuffer"
languages = ["typescript", "javascript"]

[[source]]
origin = "UserInput"
kind = "method_return"
receiver_type = "Context"
method = "req.json"
languages = ["typescript"]

[[source]]
origin = "UserInput"
kind = "method_return"
receiver_type = "Context"
method = "req.query"
languages = ["typescript"]

[[source]]
origin = "UserInput"
kind = "method_return"
receiver_type = "Context"
method = "req.header"
languages = ["typescript"]

[[source]]
origin = "UserInput"
kind = "method_return"
receiver_type = "Context"
method = "req.param"
languages = ["typescript"]

# WebSocket sources
[[source]]
origin = "UserInput"
kind = "event_data"
event_name = "message"
receiver_type = "WebSocket"
languages = ["typescript", "javascript"]

[[source]]
origin = "UserInput"
kind = "event_data"
event_name = "data"
receiver_type = "Socket"
languages = ["typescript", "javascript"]

# ─────────────────────────────────────────────────
# ENVIRONMENT SOURCES — TaintOrigin::Environment
# ─────────────────────────────────────────────────

[[source]]
origin = "Environment"
kind = "property_access"
object = "process.env"
languages = ["typescript", "javascript"]
note = "Any process.env access is an environment source"

[[source]]
origin = "Environment"
kind = "property_access"
object = "env"
languages = ["typescript"]
note = "Cloudflare Workers env binding"

[[source]]
origin = "Environment"
kind = "method_return"
method = "Deno.env.get"
languages = ["typescript"]

# ─────────────────────────────────────────────────
# DATABASE SOURCES — TaintOrigin::Database
# ─────────────────────────────────────────────────

[[source]]
origin = "Database"
kind = "method_return"
method = "findUnique"
languages = ["typescript"]

[[source]]
origin = "Database"
kind = "method_return"
method = "findFirst"
languages = ["typescript"]

[[source]]
origin = "Database"
kind = "method_return"
method = "findMany"
languages = ["typescript"]

[[source]]
origin = "Database"
kind = "method_return"
receiver_type = "KVNamespace"
method = "get"
languages = ["typescript"]
note = "KV reads are database-origin taint"

[[source]]
origin = "Database"
kind = "method_return"
receiver_type = "D1Database"
method = "first"
languages = ["typescript"]

[[source]]
origin = "Database"
kind = "method_return"
receiver_type = "D1Database"
method = "all"
languages = ["typescript"]

# ─────────────────────────────────────────────────
# NETWORK SOURCES — TaintOrigin::Network
# ─────────────────────────────────────────────────

[[source]]
origin = "Network"
kind = "method_return"
method = "fetch"
chained_methods = ["json", "text"]
languages = ["typescript", "javascript"]
note = "Response from external fetch is network-origin taint"

[[source]]
origin = "Network"
kind = "method_return"
receiver_type = "Response"
method = "json"
languages = ["typescript", "javascript"]

[[source]]
origin = "Network"
kind = "method_return"
receiver_type = "Response"
method = "text"
languages = ["typescript", "javascript"]
```

---

# FILE 7 — `pii_field_names.toml`
# Field names that carry PII taint — seeded as Database origin with PII tag
# Engine uses this to flag these fields when they flow to logs/responses/third parties

```toml
[config]
# When any of these field names appear as property accesses on DB results,
# tag them as TaintTag::Pii in addition to TaintOrigin::Database
tag = "Pii"
severity_multiplier = 1.5

[[field]]
names = ["email", "emailAddress", "email_address", "userEmail"]
category = "ContactInfo"

[[field]]
names = ["phone", "phoneNumber", "phone_number", "mobile", "mobileNumber", "cell"]
category = "ContactInfo"

[[field]]
names = ["password", "passwordHash", "password_hash", "hashedPassword", "hashed_password", "passwordDigest"]
category = "Credentials"
severity = "critical"

[[field]]
names = ["ssn", "socialSecurityNumber", "social_security_number", "taxId", "tax_id", "nin", "tfn"]
category = "GovernmentId"
severity = "critical"

[[field]]
names = ["creditCard", "credit_card", "cardNumber", "card_number", "pan", "cvv", "cvc", "expiryDate"]
category = "PaymentInfo"
severity = "critical"

[[field]]
names = ["dateOfBirth", "date_of_birth", "dob", "birthDate", "birth_date"]
category = "Demographics"

[[field]]
names = ["address", "streetAddress", "street_address", "homeAddress", "billingAddress", "shippingAddress"]
category = "Location"

[[field]]
names = ["ipAddress", "ip_address", "userIp", "user_ip", "clientIp", "client_ip"]
category = "Network"

[[field]]
names = ["passportNumber", "passport_number", "driverLicense", "driver_license", "licenseNumber"]
category = "GovernmentId"
severity = "critical"

[[field]]
names = ["medicalRecord", "medical_record", "diagnosis", "healthCondition", "health_condition", "prescription"]
category = "Medical"
severity = "critical"

[[field]]
names = ["apiKey", "api_key", "secretKey", "secret_key", "accessToken", "access_token", "refreshToken", "refresh_token", "privateKey", "private_key"]
category = "Credentials"
severity = "critical"

[[field]]
names = ["bankAccount", "bank_account", "accountNumber", "account_number", "routingNumber", "routing_number", "iban", "swift"]
category = "Financial"
severity = "critical"
```

---

# FILE 8 — `entropy_config.toml`
# Shannon entropy thresholds for hardcoded secret detection

```toml
[thresholds]
# Minimum entropy score (0.0–8.0) to flag as potential secret
# 4.0 = roughly random alphanumeric strings
# 5.5 = high entropy (base64, hex keys, JWT secrets)
medium_entropy = 4.0
high_entropy = 5.5

# Minimum string length to evaluate
min_length = 16

# Strings above this length with high entropy are always checked
# even without secret-sounding variable names
unconditional_length = 32

[variable_name_patterns]
# If variable name matches any of these (case-insensitive contains),
# apply entropy check at medium_entropy threshold
secret_indicators = [
    "key", "secret", "token", "password", "passwd", "pwd",
    "apikey", "api_key", "auth", "credential", "cred",
    "cert", "private", "priv", "signing", "encryption",
    "webhook", "hmac", "bearer", "oauth", "jwt", "salt",
    "seed", "nonce", "passphrase", "pin", "otp"
]

[exclusion_patterns]
# String values matching these patterns are NOT flagged even with high entropy
# (common test/example/placeholder values)
exclude_contains = [
    "example", "sample", "test", "fake", "demo", "placeholder",
    "change_me", "changeme", "your_", "replace_", "insert_",
    "xxxxxxxx", "00000000", "11111111",
    "lorem", "ipsum", "foobar", "foo", "bar", "baz"
]

# File path patterns to skip entirely for secret detection
exclude_file_patterns = [
    "*.test.ts", "*.spec.ts", "*.test.js", "*.spec.js",
    "__tests__/", "test/", "fixtures/", "mocks/",
    "*.md", "README*", "CHANGELOG*"
]

[known_non_secrets]
# High-entropy strings that are clearly not secrets (public identifiers, hashes of public data)
# Listed as regex patterns
patterns = [
    "^[0-9a-f]{64}$",  # SHA-256 hash of public content
    "^Bearer\\s",       # Bearer prefix — not a secret itself
    "^Basic\\s"         # Basic auth prefix — not a secret itself
]
```

---

# FILE 9 — `framework_models/hono.toml`

```toml
[framework]
name = "hono"
language = "typescript"
detection_import = "hono"
detection_import_alt = "@hono/hono"

[[source]]
origin = "UserInput"
kind = "method_return"
receiver_type = "HonoContext"
method = "req.json"
note = "c.req.json() — parsed request body"

[[source]]
origin = "UserInput"
kind = "method_return"
receiver_type = "HonoContext"
method = "req.query"
note = "c.req.query(key) — URL query parameter"

[[source]]
origin = "UserInput"
kind = "method_return"
receiver_type = "HonoContext"
method = "req.header"
note = "c.req.header(name) — request header"

[[source]]
origin = "UserInput"
kind = "method_return"
receiver_type = "HonoContext"
method = "req.param"
note = "c.req.param(name) — URL path parameter"

[[source]]
origin = "UserInput"
kind = "method_return"
receiver_type = "HonoContext"
method = "req.text"
note = "c.req.text() — raw request body as string"

[[source]]
origin = "UserInput"
kind = "method_return"
receiver_type = "HonoContext"
method = "req.parseBody"
note = "c.req.parseBody() — multipart or URL-encoded body"

[[sink]]
name = "c_redirect"
qualified = "c.redirect"
category = "OpenRedirect"
severity = "high"

[[sink]]
name = "KV_put"
qualified = "env.KV.put"
category = "StorageWrite"
severity = "high"

[[sink]]
name = "DB_prepare"
qualified = "env.DB.prepare"
category = "SqlInjection"
severity = "critical"
condition = "argument_contains_template_expression"

[[sanitizer]]
name = "zValidator"
type = "full"
note = "Hono zod-validator middleware clears taint on validated fields"

[middleware_ordering]
# Hono applies middleware in registration order
# Flag when auth middleware registered after route
auth_middleware_names = [
    "sessionMiddleware", "requireAuth", "authMiddleware",
    "bearerAuth", "basicAuth", "jwtMiddleware"
]
```

---

# FILE 10 — `framework_models/express.toml`

```toml
[framework]
name = "express"
language = "typescript"
detection_import = "express"

[[source]]
origin = "UserInput"
kind = "property_access"
receiver_type = "Request"
property = "body"

[[source]]
origin = "UserInput"
kind = "property_access"
receiver_type = "Request"
property = "query"

[[source]]
origin = "UserInput"
kind = "property_access"
receiver_type = "Request"
property = "params"

[[source]]
origin = "UserInput"
kind = "property_access"
receiver_type = "Request"
property = "headers"

[[source]]
origin = "UserInput"
kind = "property_access"
receiver_type = "Request"
property = "cookies"

[[source]]
origin = "UserInput"
kind = "property_access"
receiver_type = "Request"
property = "files"
note = "multer file upload — filename is user-controlled"

[[sink]]
name = "res_send"
qualified = "res.send"
category = "ResponseLeak"
severity = "medium"
origin_filter = "Database"

[[sink]]
name = "res_json"
qualified = "res.json"
category = "ResponseLeak"
severity = "medium"
origin_filter = "Database"

[[sink]]
name = "res_redirect"
qualified = "res.redirect"
category = "OpenRedirect"
severity = "high"

[[sink]]
name = "res_render"
qualified = "res.render"
category = "TemplateInjection"
severity = "high"

[[sanitizer]]
name = "express-validator check"
type = "full"
pattern = "validationResult(req)"
note = "express-validator result cleared means input was validated"
```

---

# FILE 11 — `framework_models/nextjs.toml`

```toml
[framework]
name = "nextjs"
language = "typescript"
detection_import = "next"
detection_import_alt = "next/server"

[[source]]
origin = "UserInput"
kind = "property_access"
receiver_type = "NextRequest"
property = "nextUrl.searchParams"

[[source]]
origin = "UserInput"
kind = "method_return"
receiver_type = "NextRequest"
method = "json"

[[source]]
origin = "UserInput"
kind = "function_parameter"
pattern = "params"
context = "generateStaticParams|generateMetadata|page|layout"
note = "Next.js route params object is user-controlled"

[[source]]
origin = "UserInput"
kind = "function_parameter"
pattern = "searchParams"
context = "page|layout"
note = "Next.js searchParams is user-controlled URL query"

[[sink]]
name = "redirect"
qualified = "redirect"
category = "OpenRedirect"
severity = "high"
import_from = "next/navigation"

[[sink]]
name = "revalidatePath"
qualified = "revalidatePath"
category = "CachePoisoning"
severity = "medium"

[[security_note]]
feature = "server_actions"
note = "Server actions are exposed as POST endpoints. Always validate input and check auth."
auth_required = true

[[security_note]]
feature = "api_routes"
path_pattern = "pages/api/**"
note = "API routes must implement their own authentication."
```

---

# FILE 12 — `framework_models/prisma.toml`

```toml
[framework]
name = "prisma"
language = "typescript"
detection_import = "@prisma/client"

[[sink]]
name = "queryRawUnsafe"
qualified = "prisma.queryRawUnsafe"
category = "SqlInjection"
severity = "critical"
always_register = true

[[sink]]
name = "executeRawUnsafe"
qualified = "prisma.executeRawUnsafe"
category = "SqlInjection"
severity = "critical"
always_register = true

[[sanitizer]]
name = "queryRaw_tagged_template"
type = "context"
clears_for = ["SqlInjection"]
pattern = "prisma.$queryRaw`"
note = "Tagged template literal version is safe — interpolations are parameterised"

[[sanitizer]]
name = "executeRaw_tagged_template"
type = "context"
clears_for = ["SqlInjection"]
pattern = "prisma.$executeRaw`"

[advisory_patterns]
# These are not bugs but patterns to note
findMany_no_limit = "findMany() without take: — potential full table scan"
upsert_no_unique = "upsert() on non-unique field — potential duplicate creation"
```

---

# FILE 13 — `framework_models/cloudflare_workers.toml`

```toml
[framework]
name = "cloudflare_workers"
language = "typescript"
detection_import = "@cloudflare/workers-types"
detection_binding_type = "KVNamespace|D1Database|R2Bucket|DurableObjectNamespace|Queue"

[[source]]
origin = "UserInput"
kind = "parameter_type"
type_name = "Request"
note = "CF Workers Request parameter is the standard source"

[[source]]
origin = "Database"
kind = "method_return"
receiver_type = "KVNamespace"
method = "get"

[[source]]
origin = "Database"
kind = "method_return"
receiver_type = "D1PreparedStatement"
method = "first"

[[source]]
origin = "Database"
kind = "method_return"
receiver_type = "D1PreparedStatement"
method = "all"

[[source]]
origin = "Database"
kind = "method_return"
receiver_type = "R2Bucket"
method = "get"

[[sink]]
name = "KV_put"
qualified = "KVNamespace.put"
category = "StorageWrite"
severity = "high"

[[sink]]
name = "KV_delete"
qualified = "KVNamespace.delete"
category = "StorageWrite"
severity = "high"

[[sink]]
name = "R2_put"
qualified = "R2Bucket.put"
category = "StorageWrite"
severity = "high"

[[sink]]
name = "D1_prepare_template"
qualified = "D1Database.prepare"
category = "SqlInjection"
severity = "critical"
condition = "argument_contains_template_expression"

[[sink]]
name = "DO_fetch"
qualified = "DurableObjectStub.fetch"
category = "StorageWrite"
severity = "medium"

[[sink]]
name = "Queue_send"
qualified = "Queue.send"
category = "MessageQueueWrite"
severity = "low"

[lifecycle]
waitUntil_required_for = ["fetch in handler", "publish", "sendEmail", "analytics.writeDataPoint"]
note = "Long-running operations must be wrapped in ctx.waitUntil() to survive response"

[isolation_rules]
module_level_mutable_state = "error"
note = "Module-level let/var mutations persist across requests in same isolate but not across isolates. Race conditions and unexpected state sharing."
```

---

# FILE 14 — `suppression_contexts.toml`
# File path and function name patterns that reduce severity or suppress findings

```toml
[global]
# Suppress ALL findings in these path patterns
suppress_file_patterns = [
    "*.test.ts",
    "*.spec.ts",
    "*.test.js",
    "*.spec.js",
    "**/__tests__/**",
    "**/test/**",
    "**/tests/**",
    "**/fixtures/**",
    "**/mocks/**",
    "**/__mocks__/**",
    "**/*.mock.ts",
    "**/seed.ts",
    "**/seed.js",
    "**/migrations/**",
    "**/scripts/dev-*"
]

[per_pattern]
# These patterns should not fire in these function names
[per_pattern.ts_unauthenticated_db_write]
suppress_function_names = [
    "register", "signup", "createAccount", "signUp",
    "resetPassword", "forgotPassword",
    "verifyEmail", "confirmEmail",
    "handleHealthCheck", "healthCheck", "ping",
    "handleWebhook"
]

[per_pattern.ts_missing_payment_gate]
suppress_function_names = [
    "healthCheck", "ping", "handleOptions",
    "handleWebhook", "processWebhook",
    "getCatalog", "getPublicPricing",
    "preflight", "handleCors"
]

[per_pattern.ts_rate_limit_missing]
suppress_function_names = [
    "handleWebhook", "internalCallback",
    "healthCheck", "metrics"
]

[per_pattern.ts_csa_validate_unconditional]
suppress_function_names = [
    "validateForAdmin",     # Admin-only: intentionally permissive
    "validateTest",         # Test helper
    "validateMock"          # Mock
]

[per_pattern.ts_insecure_random]
suppress_function_names = [
    "generatePlaceholder",
    "createLoadingSpinnerKey",
    "getAnimationId",
    "createDisplayId"       # Non-security IDs
]

[severity_downgrade]
# Downgrade severity in these contexts (finding still emitted, lower priority)

[severity_downgrade.ts_debug_stack_leak]
# Stack trace in response is warning not error in internal admin endpoints
function_name_pattern = "admin|internal|debug"
downgrade_to = "warning"

[severity_downgrade.ts_console_log_credential]
# Console.log in development-only modules is informational
file_path_pattern = "**/dev/**|**/development/**"
downgrade_to = "info"
```

---

# FILE 15 — `collection_propagators.toml`
# How taint flows through Map, Set, Array, and Object container operations

```toml
# Map operations
[[collection]]
type = "Map"
write_methods = ["set"]
read_methods = ["get", "values", "entries"]
taint_behavior = "value_to_value"
note = "map.set(key, taintedValue) → map.get(anyKey) is tainted"
languages = ["typescript", "javascript"]

# Set operations
[[collection]]
type = "Set"
write_methods = ["add"]
read_methods = ["values", "entries", "forEach"]
taint_behavior = "value_to_value"
languages = ["typescript", "javascript"]

# Array operations — push/pop/shift/unshift
[[collection]]
type = "Array"
write_methods = ["push", "unshift", "splice", "fill", "copyWithin"]
read_methods = ["pop", "shift", "at", "indexOf", "find", "findIndex"]
taint_behavior = "value_to_value"
languages = ["typescript", "javascript"]

# Object spread and property assignment
[[collection]]
type = "Object"
write_methods = ["property_assignment", "spread"]
read_methods = ["property_access", "bracket_access"]
taint_behavior = "value_to_value"
note = "{ ...taintedObj } — spread propagates taint. obj[key] = tainted — property assignment propagates."
languages = ["typescript", "javascript"]

# WeakMap / WeakRef
[[collection]]
type = "WeakMap"
write_methods = ["set"]
read_methods = ["get"]
taint_behavior = "value_to_value"
languages = ["typescript", "javascript"]

# Rust collection types
[[collection]]
type = "HashMap"
write_methods = ["insert", "entry"]
read_methods = ["get", "get_mut", "values", "iter"]
taint_behavior = "value_to_value"
languages = ["rust"]

[[collection]]
type = "Vec"
write_methods = ["push", "insert", "extend", "append"]
read_methods = ["get", "iter", "into_iter", "first", "last", "pop"]
taint_behavior = "value_to_value"
languages = ["rust"]

[[collection]]
type = "BTreeMap"
write_methods = ["insert"]
read_methods = ["get", "values", "iter"]
taint_behavior = "value_to_value"
languages = ["rust"]
```

---

# Summary: What Each File Does and How Often It Changes

| File | Purpose | Change frequency |
|------|---------|-----------------|
| `temporal_rules.toml` | Sequence constraints (A must follow B) | Rarely — add when new temporal pattern found |
| `semantic_filters.toml` | FP suppression by function/call context | Occasionally — add when new FP pattern found |
| `sinks.toml` | Qualified sink names → category+severity | Rarely — add when new platform/framework supported |
| `sanitizers.toml` | Taint-clearing functions | Rarely — fundamental programming truth |
| `propagators.toml` | Taint-flowing functions | Rarely — language semantics don't change |
| `sources.toml` | Known source types/methods | Occasionally — add when new framework supported |
| `pii_field_names.toml` | PII field name taxonomy | Occasionally — add domain-specific PII fields |
| `entropy_config.toml` | Secret detection thresholds | Rarely — tune once per year |
| `framework_models/hono.toml` | Hono-specific sources/sinks | Rarely — framework APIs are stable |
| `framework_models/express.toml` | Express-specific sources/sinks | Never — Express API frozen |
| `framework_models/nextjs.toml` | Next.js-specific sources/sinks | Occasionally — major Next.js releases |
| `framework_models/prisma.toml` | Prisma ORM sources/sinks | Rarely — Prisma API stable |
| `framework_models/cloudflare_workers.toml` | CF Workers sources/sinks | Occasionally — new CF primitives |
| `suppression_contexts.toml` | FP suppression by file/function name | Occasionally — tune as FPs found |
| `collection_propagators.toml` | Container taint propagation rules | Never — collection semantics are stable |
