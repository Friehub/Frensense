// SAFE: Uses a guard struct with a Drop implementation to ensure cleanup.
use std::fs;
use std::path::{Path, PathBuf};

struct Workspace {
    path: PathBuf,
}

impl Workspace {
    fn new(name: &str) -> Self {
        let path = PathBuf::from(format!("/tmp/test_{}", name));
        fs::create_dir_all(&path).unwrap();
        Self { path }
    }
}

impl Drop for Workspace {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

#[test]
fn test_workspace_processing() {
    let ws = Workspace::new("proc_test");
    panic!("something went wrong!");
}
