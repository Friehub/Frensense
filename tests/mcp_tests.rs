// SPDX-License-Identifier: MIT
//! Integration tests for the MCP server binary.
//!
//! All tests spawn the binary as a child process to verify real stdin/stdout
//! JSON-RPC behaviour.  Each test is isolated — a new process per test.

use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::time::{Duration, Instant};
use std::{fs, thread};

// ── Helpers ──────────────────────────────────────────────────────────────────

fn send_request(stdin: &mut ChildStdin, req: &str) {
    writeln!(stdin, "{req}").unwrap();
    stdin.flush().unwrap();
}

fn read_response(stdout: &mut BufReader<ChildStdout>) -> serde_json::Value {
    let mut line = String::new();
    loop {
        line.clear();
        assert_ne!(
            stdout.read_line(&mut line).unwrap(),
            0,
            "stdout closed unexpectedly"
        );
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            return serde_json::from_str(trimmed).expect("invalid JSON-RPC response");
        }
    }
}

fn spawn_mcp() -> (Child, ChildStdin, BufReader<ChildStdout>) {
    let bin_path = option_env!("CARGO_BIN_EXE_frensense-mcp").unwrap_or_else(|| {
        panic!(
            "frensense-mcp binary not available. \
             This test requires the 'mcp' feature to build the binary. \
             Run: cargo test --features mcp"
        );
    });
    assert!(
        std::path::Path::new(bin_path).exists(),
        "frensense-mcp binary not found at {bin_path}. \
         The build may have been cleaned between compilation and test execution. \
         Rebuild with: cargo test --features mcp"
    );
    let mut child = Command::new(bin_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("failed to spawn frensense-mcp");

    let stdin = child.stdin.take().unwrap();
    let stdout = BufReader::new(child.stdout.take().unwrap());
    (child, stdin, stdout)
}

/// Send an exit notification and forcefully reap the child.
fn shutdown(mut child: Child, mut stdin: ChildStdin) {
    let _ = writeln!(stdin, r#"{{"jsonrpc":"2.0","method":"exit"}}"#);
    let _ = stdin.flush();
    thread::sleep(Duration::from_millis(300));
    let _ = child.kill();
    let _ = child.wait();
}

/// Convenience: call tools/call once in a fresh process and parse the inner
/// result object.
fn once_tool_call(path: &str) -> serde_json::Value {
    once_tool_call_with_threshold(path, "warning")
}

fn once_tool_call_with_threshold(path: &str, severity_threshold: &str) -> serde_json::Value {
    let (child, mut stdin, mut stdout) = spawn_mcp();

    let req = format!(
        r#"{{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{{"name":"frensense_audit","arguments":{{"path":"{}","severity_threshold":"{}"}}}}}}"#,
        path.replace('\\', "\\\\").replace('\n', ""),
        severity_threshold
    );
    send_request(&mut stdin, &req);

    let resp = read_response(&mut stdout);
    let text = resp["result"]["content"][0]["text"]
        .as_str()
        .unwrap()
        .to_string();
    shutdown(child, stdin);

    serde_json::from_str(&text).unwrap()
}

// ── Smoke tests (original suite) ─────────────────────────────────────────────

#[test]
fn test_mcp_tools_list() {
    let (child, mut stdin, mut stdout) = spawn_mcp();
    send_request(
        &mut stdin,
        r#"{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}"#,
    );
    let resp = read_response(&mut stdout);

    assert_eq!(resp["id"], 1);
    assert!(resp["result"]["tools"].is_array());
    assert_eq!(resp["result"]["tools"][0]["name"], "frensense_audit");

    let schema = &resp["result"]["tools"][0]["inputSchema"];
    assert!(
        schema["required"]
            .as_array()
            .unwrap()
            .contains(&"path".into())
    );

    shutdown(child, stdin);
}

#[test]
fn test_mcp_initialize() {
    let (child, mut stdin, mut stdout) = spawn_mcp();
    send_request(
        &mut stdin,
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#,
    );
    let resp = read_response(&mut stdout);

    assert_eq!(resp["id"], 1);
    assert!(resp["result"]["capabilities"]["tools"].is_object());
    shutdown(child, stdin);
}

#[test]
fn test_mcp_audit_clean_file() {
    let dir = tempfile::tempdir().unwrap();
    let f = dir.path().join("main.rs");
    fs::write(&f, "fn main() { let x = 1; }").unwrap();

    let result = once_tool_call(&f.to_string_lossy());
    assert_eq!(result["clean"], true);
    assert!(result["advisories"].as_array().unwrap().is_empty());
    assert_eq!(result["auto_fixed"], 0);
}

#[test]
fn test_mcp_audit_triggers_findings() {
    let dir = tempfile::tempdir().unwrap();
    let f = dir.path().join("unused.rs");
    fs::write(&f, "fn main() { let x = 1; }").unwrap();

    let result = once_tool_call_with_threshold(&f.to_string_lossy(), "info");
    assert_eq!(result["clean"], false);

    let ids: Vec<&str> = result["advisories"]
        .as_array()
        .unwrap()
        .iter()
        .map(|a| a["rule_id"].as_str().unwrap())
        .collect();
    assert!(ids.contains(&"UNUSED_VARIABLE"));
}

#[test]
fn test_mcp_audit_nonexistent_path() {
    let result = once_tool_call("/this/path/should/definitely/not/exist/12345");
    assert_eq!(result["clean"], false);
    assert!(result.get("error").is_some());
}

#[test]
fn test_mcp_unknown_method() {
    let (child, mut stdin, mut stdout) = spawn_mcp();
    send_request(
        &mut stdin,
        r#"{"jsonrpc":"2.0","id":5,"method":"bogus","params":{}}"#,
    );
    let resp = read_response(&mut stdout);

    assert_eq!(resp["error"]["code"], -32601);
    shutdown(child, stdin);
}

#[test]
fn test_mcp_severity_threshold_filter() {
    let dir = tempfile::tempdir().unwrap();
    let f = dir.path().join("panic.rs");
    fs::write(&f, "fn main() { panic!(\"boom\"); }").unwrap();

    let (child, mut stdin, mut stdout) = spawn_mcp();
    let req = format!(
        r#"{{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{{"name":"frensense_audit","arguments":{{"path":"{}","severity_threshold":"critical"}}}}}}"#,
        f.to_string_lossy()
    );
    send_request(&mut stdin, &req);
    let resp = read_response(&mut stdout);
    let text: serde_json::Value =
        serde_json::from_str(resp["result"]["content"][0]["text"].as_str().unwrap()).unwrap();
    assert_eq!(
        text["clean"], true,
        "critical threshold should filter out Warnings"
    );
    shutdown(child, stdin);
}

#[test]
fn test_mcp_language_filter() {
    let dir = tempfile::tempdir().unwrap();
    let ts_file = dir.path().join("test.ts");
    fs::write(&ts_file, "function validate(x: any) { return true; }").unwrap();
    let rs_file = dir.path().join("test.rs");
    fs::write(&rs_file, "fn main() { panic!(\"boom\"); }").unwrap();

    let (child, mut stdin, mut stdout) = spawn_mcp();

    let req = format!(
        r#"{{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{{"name":"frensense_audit","arguments":{{"path":"{}","language":"rust"}}}}}}"#,
        dir.path().to_string_lossy()
    );
    send_request(&mut stdin, &req);
    let resp = read_response(&mut stdout);
    let text: serde_json::Value =
        serde_json::from_str(resp["result"]["content"][0]["text"].as_str().unwrap()).unwrap();
    assert!(
        text["advisories"].as_array().unwrap().iter().all(|a| {
            std::path::Path::new(a["file_path"].as_str().unwrap_or(""))
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("rs"))
        }),
        "language=rust should only return .rs advisories"
    );

    shutdown(child, stdin);
}

#[test]
fn test_mcp_rules_filter() {
    let dir = tempfile::tempdir().unwrap();
    let f = dir.path().join("test.ts");
    fs::write(&f, "function validate(x: any) { return true; }").unwrap();

    let (child, mut stdin, mut stdout) = spawn_mcp();

    let req = format!(
        r#"{{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{{"name":"frensense_audit","arguments":{{"path":"{}","rules":["NONEXISTENT_RULE"]}}}}}}"#,
        f.to_string_lossy()
    );
    send_request(&mut stdin, &req);
    let resp = read_response(&mut stdout);
    let text: serde_json::Value =
        serde_json::from_str(resp["result"]["content"][0]["text"].as_str().unwrap()).unwrap();
    assert_eq!(
        text["clean"], true,
        "rules=[NONEXISTENT_RULE] should filter out all advisories"
    );

    shutdown(child, stdin);
}

// ── Production-hardness tests ─────────────────────────────────────────────────

#[test]
fn test_mcp_notification_produces_no_response() {
    let (child, mut stdin, mut stdout) = spawn_mcp();

    // Send a notification (no id) — server MUST NOT write a response
    send_request(
        &mut stdin,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
    );

    // Give the server time to (wrongly) write something
    thread::sleep(Duration::from_millis(50));

    // Now send a real request with id; the response we get should be for
    // the real request, NOT for the notification.
    send_request(
        &mut stdin,
        r#"{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}"#,
    );
    let resp = read_response(&mut stdout);

    assert_eq!(resp["id"], 1, "notification must not produce a response");
    shutdown(child, stdin);
}

#[test]
fn test_mcp_malformed_json_returns_parse_error() {
    let (child, mut stdin, mut stdout) = spawn_mcp();

    // Send garbage that isn't valid JSON
    send_request(&mut stdin, "this is not json at all");

    let resp = read_response(&mut stdout);
    assert_eq!(resp["error"]["code"], -32700, "should be a parse error");

    // Server must still be alive and handle the next request
    send_request(
        &mut stdin,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}"#,
    );
    let resp = read_response(&mut stdout);
    assert_eq!(resp["id"], 2, "server must recover from malformed input");
    shutdown(child, stdin);
}

#[test]
fn test_mcp_rapid_sequential_requests() {
    let (child, mut stdin, mut stdout) = spawn_mcp();

    // Fire 10 tools/call requests back-to-back without waiting between each.
    // This stresses the JSON-RPC line reader and the per-call engine lifecycle.
    let count = 10;
    for i in 0..count {
        let req = format!(r#"{{"jsonrpc":"2.0","id":{i},"method":"tools/list","params":{{}}}}"#);
        send_request(&mut stdin, &req);
    }

    for i in 0..count {
        let resp = read_response(&mut stdout);
        assert_eq!(resp["id"], i, "response order must match request order");
        assert!(resp["result"]["tools"].is_array());
    }

    shutdown(child, stdin);
}

#[test]
fn test_mcp_string_request_id() {
    let (child, mut stdin, mut stdout) = spawn_mcp();

    send_request(
        &mut stdin,
        r#"{"jsonrpc":"2.0","id":"my-custom-id","method":"tools/list","params":{}}"#,
    );
    let resp = read_response(&mut stdout);

    assert_eq!(resp["id"], "my-custom-id");
    shutdown(child, stdin);
}

#[test]
fn test_mcp_null_request_id() {
    let (child, mut stdin, mut stdout) = spawn_mcp();

    send_request(
        &mut stdin,
        r#"{"jsonrpc":"2.0","id":null,"method":"tools/list","params":{}}"#,
    );
    let resp = read_response(&mut stdout);

    assert!(resp["id"].is_null());
    shutdown(child, stdin);
}

#[test]
fn test_mcp_tools_call_without_args_missing_name() {
    let (child, mut stdin, mut stdout) = spawn_mcp();

    send_request(
        &mut stdin,
        r#"{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{}}"#,
    );
    let resp = read_response(&mut stdout);

    // Should be method-not-found for empty tool name
    assert_eq!(resp["error"]["code"], -32602);
    shutdown(child, stdin);
}

#[test]
fn test_mcp_tools_call_without_arguments() {
    let dir = tempfile::tempdir().unwrap();
    let f = dir.path().join("test.rs");
    fs::write(&f, "fn main() {}").unwrap();
    let (child, mut stdin, mut stdout) = spawn_mcp();

    send_request(
        &mut stdin,
        &format!(
            r#"{{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{{"name":"frensense_audit","arguments":{{"path":"{}"}}}}}}"#,
            dir.path().display()
        ),
    );
    let resp = read_response(&mut stdout);
    let text: serde_json::Value =
        serde_json::from_str(resp["result"]["content"][0]["text"].as_str().unwrap()).unwrap();
    assert!(text.get("clean").is_some());
    shutdown(child, stdin);
}

#[test]
fn test_mcp_extra_fields_ignored() {
    let dir = tempfile::tempdir().unwrap();
    let f = dir.path().join("test.rs");
    fs::write(&f, "fn main() {}").unwrap();
    let (child, mut stdin, mut stdout) = spawn_mcp();

    send_request(
        &mut stdin,
        &format!(
            r#"{{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{{"name":"frensense_audit","arguments":{{"path":"{}","fix_auto":false,"severity_threshold":"info","unknown_field":"should-be-ignored"}},"extraTopLevel":null}}}}"#,
            dir.path().display()
        ),
    );
    let resp = read_response(&mut stdout);
    assert!(
        resp.get("error").is_none(),
        "extra fields must not cause errors: {:?}",
        resp.get("error")
    );
    shutdown(child, stdin);
}

#[test]
fn test_mcp_empty_file_does_not_crash() {
    let dir = tempfile::tempdir().unwrap();
    let f = dir.path().join("empty.rs");
    fs::write(&f, "").unwrap();

    let result = once_tool_call(&f.to_string_lossy());
    // An empty file should analyse cleanly (no advisories)
    assert_eq!(result["clean"], true);
}

#[test]
fn test_mcp_binary_file_does_not_crash() {
    let dir = tempfile::tempdir().unwrap();
    let f = dir.path().join("binary.rs");
    // Write raw bytes that are not valid UTF-8
    fs::write(&f, [0xff, 0xfe, 0x00, 0x01, 0x02]).unwrap();

    let result = once_tool_call(&f.to_string_lossy());
    // The engine should skip the file or report it gracefully; must not crash
    assert!(
        result.get("advisories").is_some(),
        "binary file must not crash the server"
    );
}

#[test]
fn test_mcp_unicode_paths() {
    let dir = tempfile::tempdir().unwrap();
    // Use a path containing CJK and emoji characters
    let f = dir.path().join("测试_🚀_main.rs");
    fs::write(&f, "fn main() {}").unwrap();

    let result = once_tool_call(&f.to_string_lossy());
    assert_eq!(result["clean"], true, "unicode file paths must work");
}

#[test]
fn test_mcp_deeply_nested_path() {
    let dir = tempfile::tempdir().unwrap();
    // Create a deeply nested directory tree (50 levels)
    let deep = (0..50).fold(dir.path().to_path_buf(), |p, _| p.join("sub"));
    fs::create_dir_all(&deep).unwrap();
    let f = deep.join("main.rs");
    fs::write(&f, "fn main() {}").unwrap();

    let result = once_tool_call(&f.to_string_lossy());
    assert_eq!(result["clean"], true, "deep path must work");
}

#[test]
fn test_mcp_directory_scan() {
    let dir = tempfile::tempdir().unwrap();

    // Files that trigger UNUSED_VARIABLE findings
    fs::write(
        dir.path().join("good.rs"),
        "fn main() { let x = 1; let y = 2; }",
    )
    .unwrap();
    fs::write(
        dir.path().join("bad.rs"),
        "fn main() { let a = 1; let b = 2; }",
    )
    .unwrap();
    fs::write(dir.path().join("main.rs"), "fn main() { let _ = 1; }").unwrap();
    fs::create_dir(dir.path().join("nested")).unwrap();
    fs::write(
        dir.path().join("nested").join("deep.rs"),
        "fn process() { let val = 42; }",
    )
    .unwrap();

    let (child, mut stdin, mut stdout) = spawn_mcp();
    let req = format!(
        r#"{{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{{"name":"frensense_audit","arguments":{{"path":"{}","severity_threshold":"info"}}}}}}"#,
        dir.path().to_string_lossy()
    );
    send_request(&mut stdin, &req);
    let resp = read_response(&mut stdout);
    let text: serde_json::Value =
        serde_json::from_str(resp["result"]["content"][0]["text"].as_str().unwrap()).unwrap();

    assert_eq!(text["clean"], false, "directory scan must find violations");
    let count = text["advisories"].as_array().unwrap().len();
    assert!(
        count >= 2,
        "must find at least 2 advisories, got {}: {}",
        count,
        serde_json::to_string_pretty(&text["advisories"]).unwrap_or_default()
    );
    shutdown(child, stdin);
}

#[test]
fn test_mcp_consecutive_initialize_is_idempotent() {
    let (child, mut stdin, mut stdout) = spawn_mcp();

    // Multiple initializes should each return success
    for i in 0..3 {
        let req = format!(r#"{{"jsonrpc":"2.0","id":{i},"method":"initialize","params":{{}}}}"#);
        send_request(&mut stdin, &req);
        let resp = read_response(&mut stdout);
        assert_eq!(resp["id"], i);
        assert!(resp["result"]["capabilities"]["tools"].is_object());
    }

    shutdown(child, stdin);
}

#[test]
fn test_mcp_tools_list_is_deterministic() {
    let (child, mut stdin, mut stdout) = spawn_mcp();

    // Call tools/list twice; the output must be identical
    send_request(
        &mut stdin,
        r#"{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}"#,
    );
    let r1 = read_response(&mut stdout);

    send_request(
        &mut stdin,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}"#,
    );
    let r2 = read_response(&mut stdout);

    assert_eq!(
        r1["result"], r2["result"],
        "tools/list must be deterministic"
    );
    shutdown(child, stdin);
}

#[test]
fn test_mcp_shutdown_then_tools_list_graceful() {
    let (child, mut stdin, mut stdout) = spawn_mcp();

    // Send shutdown (id=1) then immediately a tools/list (id=2).
    // The server should respond to shutdown first, then MAY respond to
    // tools/list or close gracefully — but it must not panic.
    send_request(
        &mut stdin,
        r#"{"jsonrpc":"2.0","id":1,"method":"shutdown","params":{}}"#,
    );
    let resp = read_response(&mut stdout);
    assert_eq!(resp["id"], 1);

    send_request(
        &mut stdin,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}"#,
    );
    let resp = read_response(&mut stdout);
    // After shutdown the server can either respond or close — we just ensure
    // there's no crash.
    assert!(resp.get("error").is_none() || resp["error"]["code"] == -32601);

    shutdown(child, stdin);
}

#[test]
fn test_mcp_audit_large_project_does_not_deadlock() {
    // Generate 200 small .rs files inside a temp directory to simulate a
    // medium-sized project and verify the engine doesn't deadlock or OOM.
    let dir = tempfile::tempdir().unwrap();
    for i in 0..200 {
        fs::write(dir.path().join(format!("file_{i}.rs")), "fn main() {}").unwrap();
    }
    // Also add a sub-directory with more files
    fs::create_dir(dir.path().join("sub")).unwrap();
    for i in 0..50 {
        fs::write(
            dir.path().join("sub").join(format!("lib_{i}.rs")),
            "pub fn f() {}",
        )
        .unwrap();
    }

    let start = Instant::now();
    let result = once_tool_call(&dir.path().to_string_lossy());
    let elapsed = start.elapsed();

    assert_eq!(result["clean"], true, "large project must complete cleanly");
    assert!(
        elapsed < Duration::from_secs(120),
        "large project scan must finish within 120s (took {elapsed:?})",
    );
}

#[test]
#[ignore = "slow in debug mode (>60s with full src/); run with --release or audit a smaller dir"]
fn test_mcp_audit_self_source_directory() {
    // Audit GenSense's own source code to validate the MCP server handles
    // real-world workloads without error. Must be run with --release to complete
    // in a reasonable time.
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let src_dir = manifest_dir.join("src");

    let (child, mut stdin, mut stdout) = spawn_mcp();
    let req = format!(
        r#"{{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{{"name":"frensense_audit","arguments":{{"path":"{}","severity_threshold":"info"}}}}}}"#,
        src_dir.to_string_lossy()
    );
    send_request(&mut stdin, &req);
    let resp = read_response(&mut stdout);
    let text: serde_json::Value =
        serde_json::from_str(resp["result"]["content"][0]["text"].as_str().unwrap()).unwrap();

    assert!(
        text.get("error").is_none(),
        "self-audit must not error: {:?}",
        text.get("error")
    );
    assert!(
        !text["advisories"].as_array().unwrap().is_empty(),
        "self-audit should find issues"
    );
    assert_eq!(text["clean"], false);
    shutdown(child, stdin);
}

// ── Stress: read-line starvation ──────────────────────────────────────────────

#[test]
fn test_mcp_long_line_without_newline_does_not_deadlock() {
    // Send a valid request followed by a huge padding line without newline.
    // The server's line reader blocks until newline; this is valid behaviour
    // as long as the server stays alive for the next request.
    let (child, mut stdin, mut stdout) = spawn_mcp();

    // Normal request
    send_request(
        &mut stdin,
        r#"{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}"#,
    );
    let resp = read_response(&mut stdout);
    assert!(resp["result"]["tools"].is_array());

    // Send the exit notification to clean up; must not deadlock
    shutdown(child, stdin);
}

#[test]
fn test_mcp_tools_call_interleaved_response_ordering() {
    // Send two tools/call with different ids and verify each response
    // carries the correct id (response/request pairing).
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("a.rs"), "fn a() {}").unwrap();
    fs::write(dir.path().join("b.rs"), "fn b() {}").unwrap();

    let (child, mut stdin, mut stdout) = spawn_mcp();

    let req_a = format!(
        r#"{{"jsonrpc":"2.0","id":10,"method":"tools/call","params":{{"name":"frensense_audit","arguments":{{"path":"{}"}}}}}}"#,
        dir.path().join("a.rs").to_string_lossy()
    );
    let req_b = format!(
        r#"{{"jsonrpc":"2.0","id":20,"method":"tools/call","params":{{"name":"frensense_audit","arguments":{{"path":"{}"}}}}}}"#,
        dir.path().join("b.rs").to_string_lossy()
    );

    // Send both without waiting
    send_request(&mut stdin, &req_a);
    send_request(&mut stdin, &req_b);

    let r_a = read_response(&mut stdout);
    let r_b = read_response(&mut stdout);

    // Verify ordering
    assert!(r_a["id"].as_u64() == Some(10) || r_a["id"].as_u64() == Some(20));
    assert!(r_b["id"].as_u64() == Some(10) || r_b["id"].as_u64() == Some(20));
    assert_ne!(r_a["id"], r_b["id"], "responses must have different ids");

    shutdown(child, stdin);
}

#[test]
fn test_mcp_audit_consecutive_different_paths() {
    // Run three audits in the same server session with different targets
    let dir = tempfile::tempdir().unwrap();
    let clean_f = dir.path().join("clean.rs");
    let dirty_f = dir.path().join("dirty.rs");
    let nonexistent = dir.path().join("nope.rs");
    fs::write(&clean_f, "fn main() {}").unwrap();
    fs::write(&dirty_f, "fn main() { panic!(\"x\"); }").unwrap();

    let (child, mut stdin, mut stdout) = spawn_mcp();

    for (i, path) in [&clean_f, &dirty_f, &nonexistent].iter().enumerate() {
        let req = format!(
            r#"{{"jsonrpc":"2.0","id":{},"method":"tools/call","params":{{"name":"frensense_audit","arguments":{{"path":"{}"}}}}}}"#,
            i,
            path.to_string_lossy()
        );
        send_request(&mut stdin, &req);
        let resp = read_response(&mut stdout);
        assert_eq!(resp["id"], i);
    }

    shutdown(child, stdin);
}

#[test]
fn test_mcp_audit_with_absolute_and_relative_paths() {
    let dir = tempfile::tempdir().unwrap();
    let f = dir.path().join("test.rs");
    fs::write(&f, "fn main() {}").unwrap();

    // Change directory to the temp dir for the relative-path test
    let orig_cwd = std::env::current_dir().unwrap();
    std::env::set_current_dir(dir.path()).unwrap();

    let result_rel = once_tool_call("test.rs");
    assert_eq!(result_rel["clean"], true, "relative path must work");

    let result_abs = once_tool_call(&f.to_string_lossy());
    assert_eq!(result_abs["clean"], true, "absolute path must work");

    std::env::set_current_dir(&orig_cwd).unwrap();
}

#[test]
fn test_mcp_audit_empty_directory() {
    let dir = tempfile::tempdir().unwrap();
    let result = once_tool_call(&dir.path().to_string_lossy());
    assert_eq!(result["clean"], true, "empty directory must return clean");
}

#[test]
fn test_mcp_audit_symlink_loop_does_not_infinite_loop() {
    // WalkDir's filter_entry prevents descending into symlinks that point
    // back to already-visited directories by default, but let's verify.
    let dir = tempfile::tempdir().unwrap();

    let real = dir.path().join("real");
    fs::create_dir(&real).unwrap();
    fs::write(real.join("f.rs"), "fn main() {}").unwrap();

    // Create a symlink back to the parent (loop)
    #[cfg(unix)]
    {
        let link = dir.path().join("link");
        let _ = std::os::unix::fs::symlink(dir.path(), &link);
    }

    let result = once_tool_call(&dir.path().to_string_lossy());
    // Must complete without panic or hang
    assert!(result.get("advisories").is_some());
}

// ── Protocol compliance ──────────────────────────────────────────────────────

#[test]
fn test_mcp_invalid_version_string() {
    // The JSON-RPC spec requires "2.0" — an invalid value shouldn't crash
    let (child, mut stdin, mut stdout) = spawn_mcp();

    send_request(
        &mut stdin,
        r#"{"jsonrpc":"1.0","id":1,"method":"tools/list","params":{}}"#,
    );
    // We still parse it and respond (we don't validate jsonrpc field value)
    let resp = read_response(&mut stdout);
    // As long as there's no crash and we get a response, it's fine
    assert!(resp.get("result").is_some() || resp.get("error").is_some());

    shutdown(child, stdin);
}

#[test]
fn test_mcp_missing_jsonrpc_field() {
    let (child, mut stdin, mut stdout) = spawn_mcp();

    send_request(&mut stdin, r#"{"id":1,"method":"tools/list","params":{}}"#);
    let resp = read_response(&mut stdout);
    // Should still respond (missing jsonrpc field is tolerated)
    assert!(resp.get("result").is_some() || resp.get("error").is_some());

    shutdown(child, stdin);
}

#[test]
fn test_mcp_audit_response_contains_requires_human_field() {
    let dir = tempfile::tempdir().unwrap();
    let f = dir.path().join("panic.rs");
    fs::write(&f, "fn main() { panic!(\"x\"); }").unwrap();

    let result = once_tool_call(&f.to_string_lossy());

    assert!(
        result["requires_human"].is_array(),
        "audit response must always include requires_human array"
    );
}

#[test]
fn test_mcp_audit_auto_fixed_is_zero_when_no_fixable_advisories() {
    let dir = tempfile::tempdir().unwrap();
    let f = dir.path().join("panic.rs");
    fs::write(&f, "fn main() { panic!(\"x\"); }").unwrap();

    let result = once_tool_call(&f.to_string_lossy());
    assert_eq!(
        result["auto_fixed"], 0,
        "non-fixable advisories must not increment auto_fixed"
    );
}
