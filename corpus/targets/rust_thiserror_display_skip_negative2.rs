// SAFE: Uses custom Display implementation instead of #[error] to ensure all context fields are shown.

use std::fmt;

struct FileNotFound {
    path: String,
}

impl fmt::Display for FileNotFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "file not found at path: {}", self.path)
    }
}
