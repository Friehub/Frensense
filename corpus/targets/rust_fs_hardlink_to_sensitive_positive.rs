// [frensense]
// observation: `std::fs::hard_link` creates a hard link to a sensitive file (e.g., `/etc/shadow`, a private key, or database file) from a publicly accessible path, making the sensitive data accessible through the link.
// impact: Unauthorized access to sensitive files through the hard link, which may bypass permission checks.
// improvement: Do not create hard links to sensitive files, or ensure the link target is in a protected directory.

use std::fs;
use std::path::Path;

pub fn create_link(target: &Path, link: &Path) -> std::io::Result<()> {
    fs::hard_link(target, link)
}
