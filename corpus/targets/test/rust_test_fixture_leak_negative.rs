// SAFE: Uses `TempDir` from the `tempfile` crate for automatic cleanup on drop.
use tempfile::TempDir;

#[test]
fn test_workspace_processing() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().to_owned();
    // work with `path` ...
    // TempDir cleans up on drop even after a panic
}
