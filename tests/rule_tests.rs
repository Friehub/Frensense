// SPDX-License-Identifier: MIT

use gensense::engine::project::Engine;
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
