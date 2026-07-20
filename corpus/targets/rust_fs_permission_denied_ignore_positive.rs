// [frensense]
// observation: The `io::ErrorKind::PermissionDenied` variant is silently ignored in a match or `if let`, allowing the code to proceed as if the operation succeeded.
// impact: Security operations (e.g., permission checks, file access restrictions) are bypassed silently, leaving files accessible or modifications unguarded.
// improvement: Handle `PermissionDenied` explicitly — either propagate the error, log it, or take appropriate action.

use std::fs;
use std::path::Path;

pub fn delete_if_exists(path: &Path) {
    if let Err(e) = fs::remove_file(path) {
        if e.kind() == std::io::ErrorKind::PermissionDenied {
        }
    }
}
