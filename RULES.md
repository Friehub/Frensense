# GenSense Rule Catalog

This catalog lists all semantic rules currently active in the GenSense engine.

| Rule ID | Severity | Category | Description |
| :--- | :--- | :--- | :--- |
| `RUST_ASYNC_MUTEX_DEADLOCK` | Warning | Reliability | Potential deadlock: async lock guard held across .await point. |
| `RUST_ASYNC_PANIC_PREVENTION` | Warning | General | Unsafe error handling pattern (unwrap/expect/panic) in async scope. |
| `RUST_FAKE_ASYNC` | Warning | General | Function marked as async but contains no .await points (Fake Async). |
| `RUST_ASYNC_BLOCKING` | Warning | General | Potentially blocking call (sleep/fs/net) detected in async context. |
| `RUST_MISSING_TRACING_SPAN` | Warning | Observability | Async function lacks observability instrumentation (tracing span). |
| `RUST_MISSING_TIMEOUT` | Critical | General | Async I/O operation missing explicit timeout protection. |
| `AI_PLACEHOLDER_PANIC` | Warning | General | Unimplemented placeholder panic detected. |
| `AI_TAUTOLOGICAL_ASSERT` | Warning | General | Tautological assertion detected (e.g. assert!(true)). |
| `AI_DEAD_RESULT_DISCARD` | Warning | General | Silent result discard (let _ = ...) detected. |
| `AI_USELESS_TEST` | Warning | General | Test function detected that logs output but lacks assertions. |
| `RUST_REDUNDANT_COMMENT` | Warning | General | Documentation that merely restates the identifier name. |
| `TS_FLOATING_PROMISE` | Warning | General | Unawaited promise detected (fetch/prisma/db). |
| `SECRET_LEAK_DETECTION` | Critical | Security | Potential hardcoded secret or cryptographic key detected. |
| `GLOBAL_TODO_PLACEHOLDER` | Warning | General | Unresolved TODO or FIXME detected. |
| `TS_HARDCODED_ENV_URL` | Warning | General | I found a hardcoded environment URL. This usually indicates configuration that should be externalized. |
| `JS_DYNAMIC_EXECUTION` | Warning | General | We observed a pattern of dynamic code execution using 'eval()'. |
| `TS_DYNAMIC_EXECUTION` | Warning | General | We observed a pattern of dynamic code execution using 'eval()'. |
| `TS_ALIASED_DYNAMIC_EXECUTION` | Warning | General | Aliased dynamic code execution detected. |
| `TS_CONSOLE_USAGE` | Warning | General | I noticed 'console.log' being used here. |
| `TS_BLOCKING_IO` | Warning | General | We observed a synchronous I/O call within a potentially asynchronous execution path. |
| `TS_EMPTY_CATCH` | Warning | General | Hey, it looks like this catch block is empty. |
| `TS_NON_NULL_ASSERTION` | Warning | General | We noticed a non-null assertion (!) being used. |
| `TS_MAGIC_NUMBER` | Warning | General | We noticed a raw numeric literal (magic number) being used. |
| `TS_ANY_TYPE` | Warning | General | I spotted an 'any' type annotation. |
| `TS_REACT_HOOK_DEPS` | Warning | General | I noticed a 'useEffect' hook with an empty dependency array []. |
| `TS_BUNDLE_BLOAT` | Warning | General | Hey, it looks like a heavy library is being imported entirely using 'import *'. |
| `TS_ASYNC_FOR_EACH` | Warning | General | I noticed a '.forEach' call with an 'async' callback. |
| `TS_SENSITIVE_DATA_LOGGING` | Warning | General | I noticed a logging call that seems to include sensitive keywords (password, token, secret). |
| `TS_DATA_LEAK_TRACKER` | Warning | General | Sensitive data flow into unsafe sink detected via Taint Analysis |
| `SOL_INTEGER_OVERFLOW` | Warning | General | I found an 'unchecked' arithmetic block. Please verify that overflow is physically impossible here. |
| `SOL_MISSING_ACCESS_CONTROL` | Warning | General | I noticed a public/external function that doesn't appear to have access control. Is this intended to be permissionless? |
| `SOL_STATE_MUTATION_CHECK` | Warning | General | Hey, I noticed a state mutation that looks like it might be inside a view or pure context. |
| `SOL_SELFDESTRUCT_ADVISORY` | Warning | General | Just a quick note: 'selfdestruct' is being used here. |
| `SOL_PRAGMA_STABILITY` | Warning | General | We noticed a floating pragma (e.g., ^0.8.0) in the contract header. |
| `SOL_ACCESS_CONTROL_SETTER` | Warning | General | We observed a state mutation in a function that may be missing access control. |
| `SOL_ZERO_ADDRESS_CHECK` | Warning | General | We noticed an address assignment that might be missing a zero-address check. |
| `SOL_MISSING_EVENT_INDEX` | Warning | General | We noticed an event parameter that is not 'indexed'. |
| `SOL_HARDCODED_ADDRESS` | Warning | General | We observed a hardcoded hexadecimal address literal. |
| `SOL_TX_ORIGIN` | Warning | General | We noticed 'tx.origin' being used for authentication. |
| `SOL_TIMESTAMP_DEPENDENCY` | Warning | General | I noticed the use of 'block.timestamp' for timing logic. |
| `SOL_REENTRANCY_RISK` | Warning | General | I noticed an external call that might be missing a reentrancy guard. |
| `SOL_PRECISION_LOSS` | Warning | General | It looks like a division operation occurs before multiplication here. |
| `RUST_UNSAFE_BLOCK` | Warning | General | Hey, I noticed an 'unsafe' block here. |
| `RUST_HOST_INTERACTION` | Warning | General | We observed a direct interaction with host processes using 'std::process'. |
| `RUST_STD_OUTPUT` | Warning | General | We observed the use of println! or eprintln! in library code. |
| `RUST_ALLOCATION_IN_LOG` | Warning | General | We noticed 'format!' being used inside a logging macro. |
| `RUST_CLONE_IN_LOOP` | Warning | General | Just a heads-up, there's a '.clone()' call inside this loop. |
| `RUST_CONSTRUCTOR_BLOAT` | Warning | General | It looks like this constructor has quite a few arguments. |
| `RUST_SILENT_FAILURE` | Warning | General | We observed a pattern of logging a failure but returning 'Ok(())'. |
| `RUST_VEC_FRONT_REMOVE` | Warning | General | We noticed '.remove(0)' being called on a Vec. |
| `RUST_ALGO_N2_LOOP` | Warning | General | We observed a search operation (.contains/.find) inside a loop over a collection. |
| `RUST_CHANNEL_UNBOUNDED` | Warning | General | We observed the creation of an unbounded channel (unbounded_channel). |
| `RUST_OVER_GENERAL_VARIABLE` | Warning | General | Semantic Smell: Over-generic variable naming detected. |
| `RUST_LOCK_IO` | Warning | General | I spotted a lock (Mutex/RwLock) that might be held across an I/O or await point. |
| `RUST_UNCHECKED_IO` | Warning | General | I noticed an I/O operation followed directly by an `.unwrap()`. |
| `RUST_UNWRAP_SAFETY` | Warning | General | I noticed an '.unwrap()' call that doesn't seem to have a '// SAFETY:' comment nearby. |
| `RUST_TOKIO_SELECT_ELSE` | Warning | General | I noticed a 'tokio::select!' block that might be missing an 'else' or 'default' branch. |
| `RUST_LARGE_STACK_ALLOCATION` | Warning | General | I noticed a large array allocation on the stack. |
| `RUST_PANIC_IN_LIB` | Warning | General | I found an unconditional panic macro. Library code should generally avoid hard crashes. |
| `RUST_GOD_FUNCTION` | Warning | General | God Function: block length exceeds standardized threshold (100 lines). |
| `RUST_NESTING_LIMIT` | Warning | General | Deep Nesting: logical depth exceeds standardized limit (4 levels). |
| `TS_GOD_FUNCTION` | Warning | General | God Function: block length exceeds standardized threshold (100 lines). |
| `TS_NESTING_LIMIT` | Warning | General | Deep Nesting: logical depth exceeds standardized limit (4 levels). |
| `TS_PRISMA_SELECT_STAR` | Warning | General | I noticed a Prisma query that fetches all fields. This is inefficient for production workloads. |
