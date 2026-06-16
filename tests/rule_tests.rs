// SPDX-License-Identifier: MIT

use frensense::engine::project::Engine;
use std::path::Path;

fn run_test(rule_id: &str, content: &str, expected_count: usize, ext: &str) {
    let mut engine = Engine::new();
    let path = Path::new("test").with_extension(ext);
    let advisories = engine.run_content(&path, content).unwrap();

    let rule_findings: Vec<_> = advisories.iter().filter(|a| a.rule_id == rule_id).collect();
    assert_eq!(
        rule_findings.len(),
        expected_count,
        "Rule {} expected {} findings but got {}. Content: {}",
        rule_id,
        expected_count,
        rule_findings.len(),
        content
    );
}

#[test]
fn test_rust_blocking_io() {
    let rule_id = "RUST_ASYNC_BLOCKING_IO";

    // Positive
    run_test(rule_id, "async fn t() { std::thread::sleep(d); }", 1, "rs");

    // Negative (not in async)
    run_test(rule_id, "fn t() { std::thread::sleep(d); }", 0, "rs");

    // Negative (async but safe)
    run_test(
        rule_id,
        "async fn t() { tokio::time::sleep(d).await; }",
        0,
        "rs",
    );
}

#[test]
fn test_rust_clone_in_loop() {
    let rule_id = "RUST_CLONE_IN_LOOP";

    // Positive
    run_test(
        rule_id,
        "fn t() { for x in v { let y = x.clone(); } }",
        1,
        "rs",
    );

    // Negative
    run_test(
        rule_id,
        "fn t() { let y = x.clone(); for x in v { } }",
        0,
        "rs",
    );
}

#[test]
fn test_rust_panic_in_lib() {
    let rule_id = "RUST_PANIC_IN_LIB";

    // Positive
    run_test(rule_id, "fn t() { panic!(\"error\"); }", 1, "rs");

    // Negative
    run_test(rule_id, "fn t() { return Err(\"error\"); }", 0, "rs");

    // Positive from corpus fixture
    let fixture_content = std::fs::read_to_string("corpus/targets/rust_panic_in_lib_positive.rs")
        .expect("fixture file not found — run from project root");
    run_test(rule_id, &fixture_content, 1, "rs");

    // #[cfg(test)] mod block should suppress panics (BTL-06)
    run_test(
        rule_id,
        "#[cfg(test)]\nmod tests {\n    fn helper() {\n        panic!(\"in test\");\n    }\n}\n",
        0,
        "rs",
    );

    // Plain mod tests { ... } without #[cfg(test)] should still fire (not excluded)
    run_test(
        rule_id,
        "mod tests {\n    fn helper() {\n        panic!(\"in non-cfg mod\");\n    }\n}\n",
        1,
        "rs",
    );

    // #[test] function should suppress panic
    run_test(
        rule_id,
        "#[test]\nfn unit() {\n    panic!(\"in test fn\");\n}\n",
        0,
        "rs",
    );
}

#[test]
fn test_ts_ssrf_vulnerability() {
    let rule_id = "TS_SSRF_VULNERABILITY";

    let bad_code = r"
        async function handleRequest(req, res) {
            let url = req.query.url;
            // Unsafe flow to fetch
            let response = await fetch(url);
            res.send(await response.text());
        }
    ";
    run_test(rule_id, bad_code, 1, "ts");

    let good_code = r"
        async function handleRequest(req, res) {
            let url = req.query.url;
            let safeUrl = sanitizeUrl(url);
            // Safe flow to fetch
            let response = await fetch(safeUrl);
            res.send(await response.text());
        }
    ";
    // We expect 0 here because our data leak tracker (the taint engine)
    // will see that `sanitizeUrl` interrupts the flow from `req.query.url` to `fetch`.
    run_test(rule_id, good_code, 0, "ts");
}

#[test]
fn test_ts_unawaited_test_assertion() {
    let rule_id = "TS_UNAWAITED_TEST_ASSERTION";

    let bad_code = r"
        it('should fail if unawaited', () => {
            expect(Promise.resolve(1)).resolves.toBe(1);
        });
    ";
    run_test(rule_id, bad_code, 1, "ts");

    let good_code = r"
        it('should pass if awaited', async () => {
            await expect(Promise.resolve(1)).resolves.toBe(1);
        });
    ";
    run_test(rule_id, good_code, 0, "ts");
}

#[test]
fn test_ts_tautological_assert() {
    let rule_id = "TS_TAUTOLOGICAL_ASSERT";

    // Positive: expect(x).toBe(x)
    run_test(
        rule_id,
        "it('test', () => { expect(x).toBe(x); });",
        1,
        "ts",
    );

    // Positive: expect(x).toEqual(x)
    run_test(
        rule_id,
        "it('test', () => { expect(result).toEqual(result); });",
        1,
        "ts",
    );

    // Positive: expect(true).toBeTruthy() — literal matches matcher
    run_test(
        rule_id,
        "it('test', () => { expect(true).toBeTruthy(); });",
        1,
        "ts",
    );

    // Positive: expect(null).toBeNull()
    run_test(
        rule_id,
        "it('test', () => { expect(null).toBeNull(); });",
        1,
        "ts",
    );

    // Negative: expect(x).toBe(y) — different variables
    run_test(
        rule_id,
        "it('test', () => { expect(x).toBe(y); });",
        0,
        "ts",
    );

    // Negative: expect(x).toBe(42) — different values
    run_test(
        rule_id,
        "it('test', () => { expect(result).toBe(42); });",
        0,
        "ts",
    );

    // Negative: non-expect chain
    run_test(rule_id, "const x = foo().bar();", 0, "ts");
}

#[test]
fn test_rust_csa_validate_unconditional() {
    let rule_id = "RUST_CSA_VALIDATE_UNCONDITIONAL";

    // Positive: validate function with no rejection path
    run_test(rule_id, "fn validate_user() -> bool { true }", 1, "rs");
}

#[test]
fn test_ts_csa_validate_unconditional() {
    let rule_id = "TS_CSA_VALIDATE_UNCONDITIONAL";

    // Positive from corpus fixture
    let content =
        std::fs::read_to_string("corpus/targets/ts_csa_validate_unconditional_positive.ts")
            .expect("fixture file not found");
    run_test(rule_id, &content, 1, "ts");

    // Negative from corpus fixture
    let content =
        std::fs::read_to_string("corpus/targets/ts_csa_validate_unconditional_negative.ts")
            .expect("fixture file not found");
    run_test(rule_id, &content, 0, "ts");
}

#[test]
fn test_ts_csa_auth_no_rejection() {
    let rule_id = "TS_CSA_AUTH_NO_REJECTION";

    // Positive from corpus fixture
    let content = std::fs::read_to_string("corpus/targets/ts_csa_auth_no_rejection_positive.ts")
        .expect("fixture file not found");
    run_test(rule_id, &content, 1, "ts");

    // Negative from corpus fixture
    let content = std::fs::read_to_string("corpus/targets/ts_csa_auth_no_rejection_negative.ts")
        .expect("fixture file not found");
    run_test(rule_id, &content, 0, "ts");
}

#[test]
fn test_ts_csa_sanitize_passthrough() {
    let rule_id = "TS_CSA_SANITIZE_PASSTHROUGH";

    // Positive from corpus fixture
    let content = std::fs::read_to_string("corpus/targets/ts_csa_sanitize_passthrough_positive.ts")
        .expect("fixture file not found");
    run_test(rule_id, &content, 1, "ts");

    // Negative from corpus fixture
    let content = std::fs::read_to_string("corpus/targets/ts_csa_sanitize_passthrough_negative.ts")
        .expect("fixture file not found");
    run_test(rule_id, &content, 0, "ts");
}

#[test]
fn test_ts_csa_find_never_empty() {
    let rule_id = "TS_CSA_FIND_NEVER_EMPTY";

    // Positive from corpus fixture
    let content = std::fs::read_to_string("corpus/targets/ts_csa_find_never_empty_positive.ts")
        .expect("fixture file not found");
    run_test(rule_id, &content, 1, "ts");

    // Negative from corpus fixture
    let content = std::fs::read_to_string("corpus/targets/ts_csa_find_never_empty_negative.ts")
        .expect("fixture file not found");
    run_test(rule_id, &content, 0, "ts");
}

#[test]
fn test_ts_csa_validate_unconditional_delegate() {
    // body_may_delegate_via: functions delegating to safeParse/validate/verify/check/assert
    // should NOT fire because delegation is acknowledged.
    run_test(
        "TS_CSA_VALIDATE_UNCONDITIONAL",
        "function validateInput(input: any) { return safeParse(input); }",
        0,
        "ts",
    );
    run_test(
        "TS_CSA_VALIDATE_UNCONDITIONAL",
        "function checkAuth(token: string) { return validate(token); }",
        0,
        "ts",
    );
}

#[test]
fn test_rust_connection_leak() {
    let rule_id = "RUST_CONNECTION_LEAK";

    // Positive from corpus fixture
    let content = std::fs::read_to_string("corpus/targets/rust_connection_leak_positive.rs")
        .expect("fixture file not found");
    run_test(rule_id, &content, 1, "rs");

    // Negative from corpus fixture
    let content = std::fs::read_to_string("corpus/targets/rust_connection_leak_negative.rs")
        .expect("fixture file not found");
    run_test(rule_id, &content, 0, "rs");
}

#[test]
fn test_rust_network_in_txn() {
    let rule_id = "RUST_NETWORK_IN_TXN";

    // Positive from corpus fixture
    let content = std::fs::read_to_string("corpus/targets/rust_network_in_txn_positive.rs")
        .expect("fixture file not found");
    run_test(rule_id, &content, 1, "rs");

    // Negative from corpus fixture
    let content = std::fs::read_to_string("corpus/targets/rust_network_in_txn_negative.rs")
        .expect("fixture file not found");
    run_test(rule_id, &content, 0, "rs");
}

#[test]
fn test_rust_mutate_after_response() {
    let rule_id = "RUST_MUTATE_AFTER_RESPONSE";

    // Positive from corpus fixture
    let content = std::fs::read_to_string("corpus/targets/rust_mutate_after_response_positive.rs")
        .expect("fixture file not found");
    run_test(rule_id, &content, 1, "rs");

    // Negative from corpus fixture
    let content = std::fs::read_to_string("corpus/targets/rust_mutate_after_response_negative.rs")
        .expect("fixture file not found");
    run_test(rule_id, &content, 0, "rs");
}

#[test]
fn test_ts_cookie_security() {
    let rule_id = "TS_COOKIE_SECURITY";

    // Positive from corpus fixture
    let content = std::fs::read_to_string("corpus/targets/ts_cookie_security_positive.ts")
        .expect("fixture file not found");
    run_test(rule_id, &content, 1, "ts");

    // Negative from corpus fixture
    let content = std::fs::read_to_string("corpus/targets/ts_cookie_security_negative.ts")
        .expect("fixture file not found");
    run_test(rule_id, &content, 0, "ts");
}

#[test]
fn test_ts_as_any_escape() {
    let rule_id = "TS_AS_ANY_ESCAPE";

    // Positive from corpus fixture
    let content = std::fs::read_to_string("corpus/targets/ts_as_any_escape_positive.ts")
        .expect("fixture file not found");
    run_test(rule_id, &content, 1, "ts");

    // Negative from corpus fixture
    let content = std::fs::read_to_string("corpus/targets/ts_as_any_escape_negative.ts")
        .expect("fixture file not found");
    run_test(rule_id, &content, 0, "ts");
}

#[test]
fn test_ts_llm_any_parameter() {
    let rule_id = "TS_LLM_ANY_PARAMETER";

    // Positive from corpus fixture
    let content = std::fs::read_to_string("corpus/targets/ts_llm_any_parameter_positive.ts")
        .expect("fixture file not found");
    run_test(rule_id, &content, 1, "ts");

    // Negative from corpus fixture
    let content = std::fs::read_to_string("corpus/targets/ts_llm_any_parameter_negative.ts")
        .expect("fixture file not found");
    run_test(rule_id, &content, 0, "ts");
}

#[test]
fn test_ts_llm_promise_catch() {
    let rule_id = "TS_LLM_PROMISE_CATCH";

    // Positive from corpus fixture
    let content = std::fs::read_to_string("corpus/targets/ts_llm_promise_catch_positive.ts")
        .expect("fixture file not found");
    run_test(rule_id, &content, 1, "ts");

    // Negative from corpus fixture
    let content = std::fs::read_to_string("corpus/targets/ts_llm_promise_catch_negative.ts")
        .expect("fixture file not found");
    run_test(rule_id, &content, 0, "ts");
}

#[test]
fn test_rust_llm_clone_literal() {
    let rule_id = "RUST_LLM_CLONE_LITERAL";

    // Positive from corpus fixture
    let content = std::fs::read_to_string("corpus/targets/rust_llm_clone_literal_positive.rs")
        .expect("fixture file not found");
    run_test(rule_id, &content, 1, "rs");

    // Negative from corpus fixture
    let content = std::fs::read_to_string("corpus/targets/rust_llm_clone_literal_negative.rs")
        .expect("fixture file not found");
    run_test(rule_id, &content, 0, "rs");
}

#[test]
fn test_rust_llm_await_in_sync() {
    let rule_id = "RUST_LLM_AWAIT_IN_SYNC";

    // Positive from corpus fixture
    let content = std::fs::read_to_string("corpus/targets/rust_llm_await_in_sync_positive.rs")
        .expect("fixture file not found");
    run_test(rule_id, &content, 1, "rs");

    // Negative from corpus fixture
    let content = std::fs::read_to_string("corpus/targets/rust_llm_await_in_sync_negative.rs")
        .expect("fixture file not found");
    run_test(rule_id, &content, 0, "rs");
}

#[test]
fn test_ts_command_injection() {
    let rule_id = "TS_COMMAND_INJECTION";

    // Positive from corpus fixture
    let content = std::fs::read_to_string("corpus/targets/ts_command_injection_positive.ts")
        .expect("fixture file not found");
    run_test(rule_id, &content, 1, "ts");

    // Negative from corpus fixture
    let content = std::fs::read_to_string("corpus/targets/ts_command_injection_negative.ts")
        .expect("fixture file not found");
    run_test(rule_id, &content, 0, "ts");
}

#[test]
fn test_ts_hardcoded_secret() {
    let rule_id = "TS_HARDCODED_SECRET";

    // Positive from corpus fixture
    let content = std::fs::read_to_string("corpus/targets/ts_hardcoded_secret_positive.ts")
        .expect("fixture file not found");
    run_test(rule_id, &content, 1, "ts");

    // Negative from corpus fixture
    let content = std::fs::read_to_string("corpus/targets/ts_hardcoded_secret_negative.ts")
        .expect("fixture file not found");
    run_test(rule_id, &content, 0, "ts");
}

#[test]
fn test_ts_prototype_pollution() {
    let rule_id = "TS_PROTOTYPE_POLLUTION";

    // Positive from corpus fixture
    let content = std::fs::read_to_string("corpus/targets/ts_prototype_pollution_positive.ts")
        .expect("fixture file not found");
    run_test(rule_id, &content, 1, "ts");

    // Negative from corpus fixture
    let content = std::fs::read_to_string("corpus/targets/ts_prototype_pollution_negative.ts")
        .expect("fixture file not found");
    run_test(rule_id, &content, 0, "ts");
}

#[test]
fn test_ts_llm_console_log() {
    let rule_id = "TS_LLM_CONSOLE_LOG";

    // Positive from corpus fixture
    let content = std::fs::read_to_string("corpus/targets/ts_llm_console_log_positive.ts")
        .expect("fixture file not found");
    run_test(rule_id, &content, 1, "ts");

    // Negative from corpus fixture
    let content = std::fs::read_to_string("corpus/targets/ts_llm_console_log_negative.ts")
        .expect("fixture file not found");
    run_test(rule_id, &content, 0, "ts");
}

#[test]
fn test_ts_llm_mutate_after_response() {
    let rule_id = "TS_LLM_MUTATE_AFTER_RESPONSE";

    // Positive from corpus fixture
    let content =
        std::fs::read_to_string("corpus/targets/ts_llm_mutate_after_response_positive.ts")
            .expect("fixture file not found");
    run_test(rule_id, &content, 1, "ts");

    // Negative from corpus fixture
    let content =
        std::fs::read_to_string("corpus/targets/ts_llm_mutate_after_response_negative.ts")
            .expect("fixture file not found");
    run_test(rule_id, &content, 0, "ts");
}

#[test]
fn test_rust_transmute() {
    let rule_id = "RUST_TRANSMUTE";

    // Positive from corpus fixture
    let content = std::fs::read_to_string("corpus/targets/rust_transmute_positive.rs")
        .expect("fixture file not found");
    run_test(rule_id, &content, 1, "rs");

    // Negative from corpus fixture
    let content = std::fs::read_to_string("corpus/targets/rust_transmute_negative.rs")
        .expect("fixture file not found");
    run_test(rule_id, &content, 0, "rs");
}

#[test]
fn test_rust_llm_never_err() {
    let rule_id = "RUST_LLM_NEVER_ERR";

    // Positive from corpus fixture
    let content = std::fs::read_to_string("corpus/targets/rust_llm_never_err_positive.rs")
        .expect("fixture file not found");
    run_test(rule_id, &content, 1, "rs");

    // Negative from corpus fixture
    let content = std::fs::read_to_string("corpus/targets/rust_llm_never_err_negative.rs")
        .expect("fixture file not found");
    run_test(rule_id, &content, 0, "rs");
}

#[test]
fn test_ts_unhandled_async_rejection() {
    let rule_id = "TS_UNHANDLED_ASYNC_REJECTION";

    // Positive: async function with await but no try/catch or .catch()
    run_test(
        rule_id,
        "async function fetchData() { const data = await fetch(url); return data.json(); }",
        1,
        "ts",
    );

    // Negative: async function with await and try/catch
    run_test(
        rule_id,
        "async function fetchData() { try { const data = await fetch(url); return data.json(); } catch(e) { console.error(e); } }",
        0,
        "ts",
    );

    // Negative: async function without await (no error handling needed, and filter skips it)
    run_test(rule_id, "async function noop() { return 42; }", 0, "ts");

    // Negative: async function with .catch() (instead of try/catch)
    run_test(
        rule_id,
        "async function fetchData() { await fetch(url).catch(e => console.error(e)); }",
        0,
        "ts",
    );
}

#[test]
fn test_rust_missing_await() {
    let rule_id = "RUST_MISSING_AWAIT";

    // Positive: sleep without .await
    run_test(
        rule_id,
        "async fn t() { tokio::time::sleep(dur); }",
        1,
        "rs",
    );

    // Negative: sleep with .await
    run_test(
        rule_id,
        "async fn t() { tokio::time::sleep(dur).await; }",
        0,
        "rs",
    );

    // Negative: not a tokio call
    run_test(rule_id, "fn t() { std::thread::sleep(dur); }", 0, "rs");
}

#[test]
fn test_rust_discarded_result() {
    let rule_id = "RUST_DISCARDED_RESULT";

    // Positive: .ok() discarded as expression statement
    run_test(
        rule_id,
        "fn t() { let r: Result<i32, Error> = Ok(1); r.ok(); }",
        1,
        "rs",
    );

    // Positive: .unwrap_or_default() discarded as expression statement
    run_test(
        rule_id,
        "fn t() { let r: Result<i32, Error> = Ok(1); r.unwrap_or_default(); }",
        1,
        "rs",
    );

    // Negative: .ok() used in let binding (not discarded)
    run_test(
        rule_id,
        "fn t() { let r: Result<i32, Error> = Ok(1); let val = r.ok(); }",
        0,
        "rs",
    );
}
