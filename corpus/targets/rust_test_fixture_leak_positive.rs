// [frensense]
// observation: A test creates a side-effectful resource (temp directory, database connection, file) and does not ensure cleanup on panic, so a test failure leaks files, connections, or processes that interfere with subsequent tests.
// impact: Leaked state causes cascading test failures, flaky CI, and filesystem pollution. Temp directories or databases that aren't cleaned up can exhaust disk space or cause authentication failures.
// improvement: Use a drop-based guard (e.g., `TempDir`, `ScopeGuard`) that cleans up on both success and panic, or create resources inside `#[test]` with explicit `defer` cleanup.

use std::fs;
use std::path::Path;

fn create_temp_workspace(name: &str) -> String {
    let path = format!("/tmp/test_{}", name);
    fs::create_dir_all(&path).unwrap();
    path
}

#[test]
fn test_workspace_processing() {
    let dir = create_temp_workspace("proc_test");
    // work with `dir` ...
    panic!("something went wrong!");
    // Cleanup never runs — leak!
}

#[test]
fn test_workspace_aggregation() {
    let dir = create_temp_workspace("agg_test");
    // ...
    fs::remove_dir_all(&dir).unwrap();
}
